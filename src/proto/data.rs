//! Klipper Data-Dictionary stuff

use std::collections::HashMap;

use derive_more::{Deref, From};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use serde::de::{Error, Expected, Unexpected, Visitor};
use serde::{Deserialize, Serialize};
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
pub struct Command(pub String);

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
    use super::Dictionary;

    const KLIPPER_DICT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/klipper/out/klipper.dict"
    ));

    #[test]
    fn test_klipper_dict() {
        let d: Dictionary = serde_json::from_str(KLIPPER_DICT).unwrap();
    }
}
