use derive_more::{Deref, DerefMut, FromStr};
use std::str::FromStr;

pub use ironside_build_tools::CommandParseError;

#[derive(Debug, Default, Deref, DerefMut, FromStr)]
pub struct Command(ironside_build_tools::Command);

include!(concat!(env!("OUT_DIR"), "/command_gen.rs"));
