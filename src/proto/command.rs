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

/*
impl TryFrom<u8> for Command {
    type Error = CommandParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let out = match value {
            1u8 => "identify offset=%u count=%c",
            2u8 => "finalize_config crc=%u",
            3u8 => "config_trsync oid=%c",
            4u8 => "query_counter oid=%c clock=%u poll_ticks=%u sample_ticks=%u",
            5u8 => "config_buttons oid=%c button_count=%c",
            6u8 => "clear_shutdown",
            7u8 => "set_digital_out pin=%u value=%c",
            8u8 => "buttons_ack oid=%c count=%c",
            9u8 => "endstop_query_state oid=%c",
            10u8 => "query_adxl345 oid=%c clock=%u rest_ticks=%u",
            11u8 => "trsync_set_timeout oid=%c clock=%u",
            12u8 => "config_spi oid=%c pin=%u",
            13u8 => "get_uptime",
            14u8 => "endstop_home oid=%c clock=%u sample_ticks=%u sample_count=%c rest_ticks=%u pin_value=%c trsync_oid=%c trigger_reason=%c",
            15u8 => "config_pwm_out oid=%c pin=%u cycle_ticks=%u value=%hu default_value=%hu max_duration=%u",
            16u8 => "stepper_stop_on_trigger oid=%c trsync_oid=%c",
            17u8 => "debug_ping data=%*s",
            18u8 => "set_pwm_out pin=%u cycle_ticks=%u value=%hu",
            19u8 => "queue_step oid=%c interval=%u count=%hu add=%hi",
            20u8 => "allocate_oids count=%c",
            21u8 => "buttons_query oid=%c clock=%u rest_ticks=%u retransmit_count=%c invert=%c",
            22u8 => "config_endstop oid=%c pin=%c pull_up=%c",
            23u8 => "config_spi_shutdown oid=%c spi_oid=%c shutdown_msg=%*s",
            24u8 => "config_digital_out oid=%c pin=%u value=%c default_value=%c max_duration=%u",
            25u8 => "stepper_get_position oid=%c",
            26u8 => "st7920_send_data oid=%c data=%*s",
            27u8 => "query_analog_in oid=%c clock=%u sample_ticks=%u sample_count=%c rest_ticks=%u min_value=%hu max_value=%hu range_check_count=%c",
            28u8 => "reset_step_clock oid=%c clock=%u",
            29u8 => "i2c_modify_bits oid=%c reg=%*s clear_set_bits=%*s",
            30u8 => "config_adxl345 oid=%c spi_oid=%c",
            31u8 => "hd44780_send_data oid=%c data=%*s",
            32u8 => "buttons_add oid=%c pos=%c pin=%u pull_up=%c",
            33u8 => "trsync_start oid=%c report_clock=%u report_ticks=%u expire_reason=%c",
            34u8 => "config_thermocouple oid=%c spi_oid=%c thermocouple_type=%c",
            35u8 => "spi_transfer oid=%c data=%*s",
            36u8 => "i2c_write oid=%c data=%*s",
            37u8 => "reset",
            38u8 => "config_st7920 oid=%c cs_pin=%u sclk_pin=%u sid_pin=%u sync_delay_ticks=%u cmd_delay_ticks=%u",
            39u8 => "emergency_stop",
            40u8 => "neopixel_send oid=%c",
            41u8 => "spi_set_bus oid=%c spi_bus=%u mode=%u rate=%u",
            42u8 => "config_counter oid=%c pin=%u pull_up=%c",
            43u8 => "get_clock",
            44u8 => "tmcuart_send oid=%c write=%*s read=%c",
            45u8 => "config_stepper oid=%c step_pin=%c dir_pin=%c invert_step=%c step_pulse_ticks=%u",
            46u8 => "set_digital_out_pwm_cycle oid=%c cycle_ticks=%u",
            47u8 => "debug_write order=%c addr=%u val=%u",
            48u8 => "queue_pwm_out oid=%c clock=%u value=%hu",
            49u8 => "hd44780_send_cmds oid=%c cmds=%*s",
            50u8 => "spi_set_software_bus oid=%c miso_pin=%u mosi_pin=%u sclk_pin=%u mode=%u rate=%u",
            51u8 => "neopixel_update oid=%c pos=%hu data=%*s",
            52u8 => "debug_read order=%c addr=%u",
            53u8 => "query_thermocouple oid=%c clock=%u rest_ticks=%u min_value=%u max_value=%u",
            54u8 => "config_analog_in oid=%c pin=%u",
            55u8 => "query_adxl345_status oid=%c",
            56u8 => "config_spi_without_cs oid=%c",
            57u8 => "config_hd44780 oid=%c rs_pin=%u e_pin=%u d4_pin=%u d5_pin=%u d6_pin=%u d7_pin=%u delay_ticks=%u",
            58u8 => "config_neopixel oid=%c pin=%u data_size=%hu bit_max_ticks=%u reset_min_ticks=%u",
            59u8 => "spi_send oid=%c data=%*s",
            60u8 => "i2c_read oid=%c reg=%*s read_len=%u",
            61u8 => "trsync_trigger oid=%c reason=%c",
            62u8 => "st7920_send_cmds oid=%c cmds=%*s",
            63u8 => "get_config",
            64u8 => "set_next_step_dir oid=%c dir=%c",
            65u8 => "config_tmcuart oid=%c rx_pin=%u pull_up=%c tx_pin=%u bit_time=%u",
            66u8 => "update_digital_out oid=%c value=%c",
            67u8 => "queue_digital_out oid=%c clock=%u on_ticks=%u",
            68u8 => "config_i2c oid=%c i2c_bus=%u rate=%u address=%u",
            69u8 => "debug_nop",
            other => return Err(CommandParseError::NoSuchId(other)),
        };
        Command::from_str(out)
    }
}
*/

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
