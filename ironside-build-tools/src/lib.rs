//! Klipper build-time helper analogs

use std::collections::HashMap;
use std::str::FromStr;

use derive_more::Deref;
use indexmap::IndexMap;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use syn::parse_quote;
use syn::token::Brace;

#[derive(Debug, Serialize, Deserialize)]
/// Direct replica of the `klipper.dict` format generated by the C build system
pub struct Dictionary {
    pub build_versions: String,
    pub version: String,
    commands: CommandDefs,
    config: ConfigDefs,
    responses: ResponseDefs,
    #[serde(rename = "enumerations")]
    enums: EnumDefs,
}

/// shitty struct to hold command names for now
#[derive(Debug, Default, Clone)]
pub struct Command {
    /// Name of the struct
    pub name: String,
    /// Definition of the struct
    pub def: String,
    /// Map of field name to the "type" of field
    pub fields: IndexMap<String, EnumType>,
}

#[derive(thiserror::Error, Debug)]
pub enum CommandParseError {
    #[error("Expected an identifier")]
    MissingIdent,
    #[error("Expected a scanf token")]
    MissingScanf,
    #[error("Invalid scanf literal `{what}`")]
    InvalidLiteral {
        what: String,
        #[source]
        inner: ::strum::ParseError,
    },
    #[error("No such id: {0}")]
    NoSuchId(u8),
}

impl Dictionary {
    /// Pure convenience, saves you from diggin out the import
    pub fn to_token_stream(&self) -> TokenStream {
        ToTokens::to_token_stream(&self)
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(def: &str) -> Result<Self, Self::Err> {
        let def = def.to_string();
        let mut split_iter = def.split_whitespace();
        let name = split_iter
            .next()
            .ok_or(CommandParseError::MissingIdent)?
            .into();

        let mut fields = IndexMap::new();
        for arg_string in split_iter {
            // strings of the form k=%v
            // where k is a field and %v is a scanf literal
            let mut args = arg_string.splitn(2, '=');
            let field_name = args.next().ok_or(CommandParseError::MissingIdent)?;
            let ty_name = args.next().ok_or(CommandParseError::MissingScanf)?;

            let ty =
                EnumType::from_str(ty_name).map_err(|e| CommandParseError::InvalidLiteral {
                    what: ty_name.to_string(),
                    inner: e,
                })?;
            fields.insert(field_name.to_string(), ty);
        }
        Ok(Self { name, def, fields })
    }
}

#[derive(PartialEq, EnumString, Clone, Copy, Debug)]
pub enum EnumType {
    #[strum(serialize = "%c")]
    U8,
    #[strum(serialize = "%hu")]
    U16,
    #[strum(serialize = "%u")]
    U32,
    #[strum(serialize = "%hi")]
    I16,
    #[strum(serialize = "%i")]
    I32,
    #[strum(serialize = "%s", serialize = "%*s", serialize = "%.*s")]
    Bytes,
}

impl ToTokens for EnumType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let t = match *self {
            EnumType::U8 => quote! { u8 },
            EnumType::U16 => quote! { u16 },
            EnumType::U32 => quote! { u32 },
            EnumType::I16 => quote! { i16 },
            EnumType::I32 => quote! { i32 },
            EnumType::Bytes => quote! { String },
        };
        t.to_tokens(tokens)
    }
}

/// A 'message' type that is generated via scanf string in the klipper source
#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct MessageDef(HashMap<String, u8>);

/// A newtype `MessageDef` for a `Command`, from DECL_COMMAND
#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct CommandDefs(MessageDef);

/// The DECL_CONSTANT definitions
#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct ConfigDefs(HashMap<String, u32>);

/// A newtype `MessageDef` for a `Response`, which is a message destined for
/// the host, from the mcu, typically in response to a command
#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct ResponseDefs(MessageDef);

#[derive(Serialize, Deserialize, Debug)]
struct EnumDefs(#[serde(with = "tuple_vec_map")] Vec<(String, Variants)>);

#[derive(Deref, Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Variants(#[serde(with = "tuple_vec_map")] Vec<(String, EnumValue<u8>)>);

/// Store either a constant value or a range of values
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum EnumValue<T> {
    Static(T),
    Ranged(T, T), // im gross, this should just be usize...
}

struct VariantIter(Box<dyn Iterator<Item = TokenStream>>);

impl Iterator for VariantIter {
    type Item = TokenStream;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

macro_rules! mkid {
    ($prefix:ident, $idx:tt) => {
        format_ident!("{}{}", ::heck::AsUpperCamelCase(&$prefix).to_string(), $idx)
    };
}

impl EnumValue<u8> {
    fn to_tokens(&self, prefix: String, mut tokens: &mut TokenStream) {
        match *self {
            Self::Static(value) => {
                let ident = mkid!(prefix, "");
                let value = Literal::usize_unsuffixed(value as usize);
                let t = quote! { #ident = #value , };
                t.to_tokens(tokens)
            }
            Self::Ranged(start, stride) => (0..)
                .take(stride.into())
                .map(|s| (s, mkid!(prefix, s)))
                .map(|(v, ident)| {
                    let v = Literal::usize_unsuffixed(start as usize + v);
                    quote! { #ident = #v , }
                })
                .for_each(|f| f.to_tokens(&mut tokens)),
        }
    }
}

impl MessageDef {
    fn to_tokens(&self, enum_name: &str, struct_name: &str, tokens: &mut TokenStream) {
        let struct_name = Ident::new(struct_name, Span::call_site());
        let enum_name = Ident::new(enum_name, Span::call_site());
        let outer = quote! {
            #[derive(::nom::Nom, ::strum::EnumString,::strum::FromRepr)]
            #[derive(Debug)]
            pub enum #enum_name
        };
        outer.to_tokens(tokens);
        let mut enum_body = TokenStream::default();
        let mut impl_body = TokenStream::default();
        for (scanf, str_id) in self.iter() {
            let cmd = Command::from_str(scanf).expect("invalid scanf string");
            let cmd_name = Ident::new(
                &::heck::AsPascalCase(cmd.name.as_str()).to_string(),
                Span::call_site(),
            );
            let (field, ty): (Vec<Ident>, Vec<EnumType>) = cmd
                .fields
                .iter()
                .map(|(f, t)| (Ident::new(f, Span::call_site()), t))
                .unzip();
            let variant = quote! {
                #[strum(serialize = #scanf)]
                #cmd_name {
                    #(#field: #ty,)*
                },
            };
            variant.to_tokens(&mut enum_body);
            let cmd_id = Literal::u8_suffixed(*str_id);
            let match_arm = quote! {
                [#cmd_id, buf @ ..] => {
                    Self::#cmd_name {

                    }
                },
            };
            match_arm.to_tokens(&mut impl_body);
        }
        Brace::default().surround(tokens, |x| enum_body.to_tokens(x));

        let (scanf_strings, string_ids): (Vec<&String>, Vec<u8>) = self.iter().unzip();
        // Cheat, use quote to generate the code all hygenic-like
        let t: syn::ItemImpl = parse_quote! {
            impl ::std::convert::TryFrom<u8> for crate::data::#struct_name {
                type Error = crate::data::CommandParseError;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    let out = match value {
                        #(#string_ids => #scanf_strings),*,
                        other => return Err(Self::Error::NoSuchId(other)),
                    };
                    Self::from_str(out)
                }
            }
        };
        t.to_tokens(tokens);
    }
}

impl ToTokens for CommandDefs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens("Commands", "Command", tokens)
    }
}

impl ToTokens for ResponseDefs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens("Responses", "Response", tokens)
    }
}

impl ToTokens for EnumDefs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // now do the enums!
        for (name, variants) in self.0.iter() {
            // spit out just the first part...
            let ident = format_ident!("{}", heck::AsUpperCamelCase(name).to_string());
            let t = quote! {
                #[derive(::strum::EnumString,::strum::FromRepr)]
                #[derive(Debug,Copy,Clone,PartialEq)]
                #[repr(C)]
                pub enum #ident
            };
            t.to_tokens(tokens);
            // this tokenstream holds all the enum's variants
            let mut inner = TokenStream::new();
            for (prefix_source, value) in variants.iter() {
                let prefix = if prefix_source.ends_with('0') {
                    // TODO! not specific enough, needs to end with _a single_ 0
                    prefix_source
                        .trim_end_matches(|c: char| c.is_ascii_digit())
                        .to_owned()
                } else {
                    prefix_source.to_owned()
                };
                value.to_tokens(prefix, &mut inner)
            }
            // and we stuff it in some braces...
            let b = Brace::default();
            b.surround(tokens, |x| inner.to_tokens(x));
            // ...then attach it back to the rest
        }
    }
}

impl ToTokens for Dictionary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.commands.to_tokens(tokens);
        self.enums.to_tokens(tokens);
        self.responses.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KLIPPER_DICT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/klipper/out/klipper.dict"
    ));

    #[test]
    fn test_klipper_dict() {
        let _: Dictionary = serde_json::from_str(KLIPPER_DICT).unwrap();
    }

    #[test]
    fn test_parse_struct_from_command_def() {
        let s = "config_st7920 oid=%c cs_pin=%u sclk_pin=%u sid_pin=%u sync_delay_ticks=%u cmd_delay_ticks=%u";
        let command = Command::from_str(s).unwrap();
        let fields = [
            ("oid", EnumType::U8),
            ("cs_pin", EnumType::U32),
            ("sclk_pin", EnumType::U32),
            ("sid_pin", EnumType::U32),
            ("sync_delay_ticks", EnumType::U32),
            ("cmd_delay_ticks", EnumType::U32),
        ]
        .iter()
        .map(|(s, t)| (s.to_string(), *t))
        .collect();
        assert_eq!(command.fields, fields);
    }
}
