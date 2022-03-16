//! Klipper Data-Dictionary stuff

use std::collections::HashMap;

use derive_more::Deref;
use serde::de::{Error, Expected, Unexpected, Visitor};
use serde::{Deserialize, Serialize};
use syn::Ident;

#[derive(Serialize, Deserialize)]
pub struct Dictionary {
    build_versions: String,
    version: String,
    commands: CommandDefs,
    responses: ResponseDefs,
    #[serde(rename = "enumerations")]
    enums: EnumDefs,
}

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
pub struct CommandDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
pub struct ResponseDefs(HashMap<String, u8>);

#[derive(Serialize, Deserialize, Deref, Debug)]
#[serde(transparent)]
pub struct EnumDefs(HashMap<String, EnumDef>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct EnumDef(HashMap<String, EnumValue<u8>>);

/// Store either a constant value or a range of values
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum EnumValue<T> {
    Static(T),
    Ranged(T, T),
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
