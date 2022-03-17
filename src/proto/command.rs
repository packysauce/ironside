use ironside_build_tools::{EnumType, FromCommandDecl};
use std::collections::HashMap;
use std::str::FromStr;

/// shitty struct to hold command names for now
#[derive(Clone)]
pub struct Command {
    /// Name of the struct
    pub name: String,
    /// Definition of the struct
    pub def: String,
    /// Map of field name to the "type" of field
    pub fields: HashMap<String, EnumType>,
}

#[derive(thiserror::Error, Debug)]
pub enum CommandParseError {
    #[error("Expected an identifier")]
    MissingIdent,
    #[error("Expected a scanf token")]
    MissingScanf,
    #[error("Invalid scanf literal")]
    InvalidLiteral(#[from] ::strum::ParseError),
    #[error("No such id: {0}")]
    NoSuchId(u8),
}

impl FromCommandDecl for Command {
    type Err = CommandParseError;

    fn from_str(def: &str) -> Result<Self, Self::Err> {
        let def = def.to_string();
        let mut split_iter = def.split_whitespace();
        let name = split_iter
            .next()
            .ok_or(CommandParseError::MissingIdent)?
            .into();

        let mut fields = HashMap::new();
        for arg_string in split_iter {
            // strings of the form k=%v
            // where k is a field and %v is a scanf literal
            let mut args = arg_string.splitn(2, '=');
            let field_name = args.next().ok_or(CommandParseError::MissingIdent)?;
            let ty = args
                .next()
                .ok_or(CommandParseError::MissingScanf)
                .and_then(|s| Ok(EnumType::from_str(s)?))
                .map_err(CommandParseError::from)?;
            fields.insert(field_name.to_string(), ty);
        }
        Ok(Self { name, def, fields })
    }
}

include!(concat!(env!("OUT_DIR"), "/command_gen.rs"));

#[cfg(test)]
mod tests {
    use super::*;

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
