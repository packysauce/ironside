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
    pub build_versions: String,
    pub version: String,
    commands: CommandDefs,
    responses: ResponseDefs,
    #[serde(with = "tuple_vec_map", rename = "enumerations")]
    enums: Vec<(String, Variants)>,
}

impl Dictionary {
    pub fn to_token_stream(&self) -> TokenStream {
        ToTokens::to_token_stream(&self)
    }
}

pub trait FromCommandDecl: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
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
    #[strum(serialize = "%*s")]
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

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct CommandDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
struct ResponseDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Debug)]
struct EnumDefs(Vec<String>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct Variants(#[serde(with = "tuple_vec_map")] Vec<(String, EnumValue<u8>)>);

/// Store either a constant value or a range of values
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum EnumValue<T> {
    Static(T),
    Ranged(T, T),
}

impl ToTokens for Dictionary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (scanf_strings, string_ids): (Vec<String>, Vec<u8>) =
            self.commands.iter().map(|(x, y)| (x.clone(), y)).unzip();

        let t: syn::ItemImpl = parse_quote! {
            impl ::std::convert::TryFrom<u8> for crate::proto::command::Command {
                type Error = crate::proto::command::CommandParseError;

                fn try_from(value: u8) -> Result<Self, Self::Error> {
                    let out = match value {
                        #(#string_ids => #scanf_strings),*,
                        other => return Err(Self::Error::NoSuchId(other)),
                    };
                    Self::from_str(out)
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
