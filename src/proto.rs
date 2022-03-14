use std::io::Cursor;

use crate::command::encoded_len;

use vlq_rust::*;

pub type Result<T> = std::result::Result<T, std::io::Error>;

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
