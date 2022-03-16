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
    use crate::proto::KlipperBytes;

    use super::*;
    use proptest::num::u32::ANY as ANYu32;
    use proptest::num::u64::ANY as ANYu64;
    use proptest::prelude::*;
    use rand::prelude::*;

    proptest! {
        #[test]
        fn encode_lengths_match_docs(value in ANYu32) {
            let r = value.to_klipper_bytes();
            // https://www.klipper3d.org/Protocol.html#variable-length-quantities
            assert_eq!(encoded_len(value as i32), r.len());
        }

        /// Generate 1000 random numbers from any seed and test that they round trip
        #[test]
        fn round_trip_subset(seed in ANYu64) {
            let mut rng = StdRng::seed_from_u64(seed);
            for _ in 0u32..1000 {
                let encode_me: u32 = rng.gen();
                let decode_me = encode_me.to_klipper_bytes();
                let decoded = u32::from_klipper_bytes(&decode_me);
                assert_eq!(encode_me, decoded);
            }
        }
    }

    #[test]
    fn test_encode_one() {
        assert_eq!(1234u32.to_klipper_bytes(), &[0x89, 0x52]);
    }

    #[test]
    fn test_decode_one() {
        let expected = [0x89, 0x52].to_vec().into();
        assert_eq!(u32::from_klipper_bytes(&expected), 1234);
    }
}
