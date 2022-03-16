use std::io::Cursor;

use crate::command::encoded_len;
use crate::ffi::generated;

use derive_more::{Deref, From};
use vlq_rust::*;

pub mod data;

pub type OID = u8;
pub type Result<T> = std::result::Result<T, std::io::Error>;

pub enum DataType<T> {
    Const(T),
    Array(Vec<u8>),
}

/// Newtype around a byte array because the klipper side has a different byte order
#[derive(PartialEq, Deref, From, Debug)]
pub struct KlipperVarint(pub Vec<u8>);

impl<T> PartialEq<T> for KlipperVarint
where
    Vec<u8>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

pub trait KlipperBytes {
    fn to_klipper_bytes(self) -> KlipperVarint;
    fn from_klipper_bytes(bytes: &KlipperVarint) -> Self;
}

impl<T> KlipperBytes for T
where
    T: Into<u32> + From<u32>,
{
    fn to_klipper_bytes(self) -> KlipperVarint {
        let s = self.into();
        let mut buf = [0x81u8; 5]; // blub
        let mut buf_start = buf.as_mut_ptr();
        let count = unsafe {
            let buf_end = generated::encode_int(buf_start, s);
            assert!(
                buf_end > buf_start,
                "encode_int returned a pointer with negative offset"
            );
            buf_end.offset_from(buf_start) as usize
        };
        KlipperVarint(buf[..count].into())
    }

    fn from_klipper_bytes(bytes: &KlipperVarint) -> Self {
        let mut data = bytes.as_ptr() as *mut u8;
        /// SAFETY: the _pointer_ gets mangled, but not the array underneath
        /// solution? just use a different pointer
        let u = unsafe { generated::parse_int(&mut data) };
        u.into()
    }
}

pub trait ToVarintBytes {
    fn to_varint_bytes(&self) -> Result<Vec<u8>>;
}

pub trait FromVarintBytes: Sized {
    fn from_varint_bytes(bytes: &[u8]) -> Result<Self>;
}

impl<T> ToVarintBytes for T
where
    T: Copy + Vlq,
{
    fn to_varint_bytes(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        out.write_vlq(*self)?;
        Ok(out)
    }
}

impl<T> FromVarintBytes for T
where
    T: Vlq,
{
    fn from_varint_bytes(bytes: &[u8]) -> Result<Self> {
        let mut c = Cursor::new(bytes);
        c.read_vlq()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::num::u32::ANY as ANYu32;
    use proptest::num::u64::ANY as ANYu64;
    use proptest::prelude::*;
    use rand::prelude::*;

    proptest! {
        /// Generate 1000 random numbers from any seed and test that they round trip
        #[test]
        fn round_trip_subset(seed in ANYu64) {
            let mut rng = StdRng::seed_from_u64(seed);
            for _ in 0u32..1000 {
                let encode_me: u32 = rng.gen();
                let mut decode_me = encode_me.to_varint_bytes().unwrap();
                let decoded = u32::from_varint_bytes(&decode_me).unwrap();
                assert_eq!(encode_me, decoded);
            }
        }
    }

    #[test]
    fn test_encode_one() {
        assert_eq!(1234.to_varint_bytes().unwrap(), &[0x52, 0x89]);
    }

    #[test]
    fn test_decode_one() {
        assert_eq!(i32::from_varint_bytes(&[0x52, 0x89]).unwrap(), 1234);
    }
}
