/*!
 * Klipper serialization format
 * Need the klipper.dict to generate a "Deserializer" because it has the
 * IDs of commands and responses. Then that can pull out the proper enums
 * and such. Shouldn't need a Deserialize impl, just a Deserializer
 */



use crossbeam::channel::{unbounded, Receiver, Sender};
use paste::paste;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serializer;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Read;

use crate::proto::{KlipperBytes, OID};

// use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Default, Clone)]
/// Klipper serial format!
pub struct KSF {
    known_constants: HashMap<String, ()>,
    known_enums: HashMap<String, OID>,
    buf: Vec<u8>,
}

mod ser {
    #[derive(thiserror::Error, Debug, Clone)]
    pub enum Error {
        #[error("Unexpected end of input")]
        Eof,
        #[error("Truly unexpected error: {0}")]
        Custom(String),
    }

    impl serde::ser::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            Self::Custom(msg.to_string())
        }
    }
}

macro_rules! impl_klipper {
    ($($ty:ty),+) => {
        paste! {
            $(fn [<serialize_ $ty>](self, v: $ty) -> ::std::result::Result<Self::Ok, Self::Error> {
                self.serialize_bytes(&v.to_klipper_bytes())
            })+
        }
    };
}

macro_rules! impl_element {
    ($name:ident, $fn_suffix:literal) => {
        impl $name for KSF {
            type Ok = ();
            type Error = ser::Error;

            paste! {
                fn [<serialize_ $fn_suffix>]<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
                where
                    T: serde::Serialize,
                {
                    value.serialize(self)
                }
            }

            fn end(self) -> Result<Self::Ok, Self::Error> {
                Ok(())
            }
        }
    };
}

impl_element!(SerializeSeq, "element");
impl_element!(SerializeTuple, "element");
impl_element!(SerializeTupleStruct, "field");
impl_element!(SerializeTupleVariant, "field");
//impl_element!(SerializeMap, "field");

impl SerializeMap for KSF {
    type Ok;

    type Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl Serializer for KSF {
    type Ok = ();
    type Error = ser::Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    impl_klipper!(bool, char);
    impl_klipper!(i8, i16, i32, i64);
    impl_klipper!(u8, u16, u32, u64);
    impl_klipper!(f32, f64);

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.buf.extend(v.len().to_klipper_bytes().iter());
        Ok(self.buf.extend(v.iter()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub trait McuCommand {
    const OID: u32;
    type Response;
    fn to_command_string(&self) -> String;
}

pub mod commands {
    use serde::{Deserialize, Serialize};

    /// Query the "data dictionary" from the micro-controller
    #[derive(Serialize, Deserialize)]
    pub struct Identify {
        offset: usize,
        count: usize,
    }

    #[derive(Serialize, Deserialize)]
    pub struct IdentifyResponse {
        offset: usize,
        data: Vec<u8>,
    }
}

pub struct FastReader {}

pub enum Message {}

/// Handles communicating with an mcu
pub struct SerialQueue {
    sender: Option<Sender<Message>>,
    receiver: Receiver<Message>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open serial port")]
    Serial(#[from] serial::Error),
}

impl SerialQueue {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender: Some(sender),
            receiver,
        }
    }

    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<(), Error> {
        let mut port = serial::open(port)?;

        let mut buf = vec![0u8; 4096];
        while let Ok(msg) = port.read(&mut buf) {}
        Ok(())
    }
}
