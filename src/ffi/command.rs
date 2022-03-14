use derive_more::{Deref, From};

#[allow(deref_nullptr)]
mod generated {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Newtype around a byte array because the klipper side has a different byte order
#[derive(PartialEq, Deref, From, Debug)]
pub struct KlipperVarint(Vec<u8>);

impl<T> PartialEq<T> for KlipperVarint
where
    Vec<u8>: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

/// Safe wrapper to Klipper's C version
pub fn decode_int(data: &[u8]) -> u32 {
    let mut data = data.as_ptr() as *mut u8;
    /// SAFETY: the _pointer_ gets mangled, but not the array underneath
    /// solution? just use a different pointer
    unsafe {
        generated::parse_int(&mut data)
    }
}

/// Safe wrapper to Klipper's C version
pub fn encode_int(v: u32) -> KlipperVarint {
    let mut buf = [0x81u8; 5]; // blub
    let mut buf_start = buf.as_mut_ptr();
    let count = unsafe {
        let buf_end = generated::encode_int(buf_start, v);
        assert!(
            buf_end > buf_start,
            "encode_int returned a pointer with negative offset"
        );
        buf_end.offset_from(buf_start) as usize
    };
    KlipperVarint(buf[..count].into())
}

pub const fn encoded_len(value: i32) -> usize {
    match value {
        -32..=95 => 1,
        -4096..=12_287 => 2,
        -524_288..=1_572_863 => 3,
        -67_108_864..=201_326_591 => 4,
        _ => 5,
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
        #[test]
        fn encode_lengths_match_docs(value in ANYu32) {
            let r = encode_int(value);
            // https://www.klipper3d.org/Protocol.html#variable-length-quantities
            assert_eq!(encoded_len(value as i32), r.len());
        }

        /// Generate 1000 random numbers from any seed and test that they round trip
        #[test]
        fn round_trip_subset(seed in ANYu64) {
            let mut rng = StdRng::seed_from_u64(seed);
            for _ in 0u32..1000 {
                let encode_me = rng.gen();
                let mut decode_me = encode_int(encode_me);
                let decoded = decode_int(&decode_me);
                assert_eq!(encode_me, decoded);
            }
        }
    }

    #[test]
    fn test_encode_one() {
        assert_eq!(encode_int(1234), &[0x89, 0x52]);
    }

    #[test]
    fn test_decode_one() {
        assert_eq!(decode_int(&[0x89, 0x52]), 1234);
    }
}
