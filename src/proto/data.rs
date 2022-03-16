//! Klipper Data-Dictionary stuff

use std::collections::HashMap;

use derive_more::Deref;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use syn::parse_quote;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dictionary {
    build_versions: String,
    version: String,
    commands: CommandDefs,
    responses: ResponseDefs,
    #[serde(with = "tuple_vec_map", rename = "enumerations")]
    enums: Vec<(String, Variants)>,
}

/// shitty struct to hold command names for now
#[derive(Clone)]
pub struct Command {
    /// Name of the struct
    pub name: String,
    /// Definition of the struct
    pub def: String,
    /// Map of field name to the "type" of field
    pub fields: HashMap<String, ScanfToken>,
}

pub trait FromCommandDecl: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

#[derive(PartialEq, EnumString, Clone, Copy, Debug)]
pub enum ScanfToken {
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
    #[strum(serialize = "%*s")]
    Bytes,
}

impl ToTokens for ScanfToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let t = match *self {
            ScanfToken::U8 => quote! { u8 },
            ScanfToken::U16 => quote! { u16 },
            ScanfToken::U32 => quote! { u32 },
            ScanfToken::I16 => quote! { i16 },
            ScanfToken::I32 => quote! { i32 },
            ScanfToken::Bytes => quote! { String },
        };
        t.to_tokens(tokens)
    }
}

pub type Variant = (String, EnumValue<u8>);
#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
pub struct CommandDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
pub struct ResponseDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Debug)]
struct EnumDefs(Vec<String>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Variants(#[serde(with = "tuple_vec_map")] Vec<(String, EnumValue<u8>)>);

/// Store either a constant value or a range of values
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EnumValue<T> {
    Static(T),
    Ranged(T, T),
}

impl ToTokens for Dictionary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (scanf_strings, string_ids): (Vec<String>, Vec<u8>) =
            self.commands.iter().map(|(x, y)| (x.clone(), y)).unzip();

        let t: syn::ItemImpl = parse_quote! {
            impl TryFrom<u8> for Command {
                type Error = ();

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    let out = match value {
                        #(#string_ids => #scanf_strings),*,
                        _ => return Err(()),
                    };
                    Ok(out)
                }
            }
        };
        t.to_tokens(tokens)
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
}
