#![allow(unused)]

use std::fmt::Display;
use std::marker::PhantomData;

use ffi::command::parse_int;

//use pyo3::prelude::*;

mod ffi {
    pub mod command {
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]

        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use proptest::prelude::*;
    use proptest::num::u32::{ANY as ANYu32};
    use proptest::num::u64::{ANY as ANYu64};

    proptest!{
        #[test]
        fn encode_lengths_match_docs(value in ANYu32) {
            let r = encode_int(value);
            // https://www.klipper3d.org/Protocol.html#variable-length-quantities
            let expected_len = match value as i32 {
                -32 ..= 95 => 1,
                -4096 ..= 12_287 => 2,
                -524_288 ..= 1_572_863 => 3,
                -67_108_864 ..= 201_326_591 => 4,
                _ => 5,
            };
            assert_eq!(expected_len, r.len());
        }

        /// Generate 1000 random numbers from any seed and test that they round trip
        #[test]
        fn round_trip_subset(seed in ANYu64) {
            let mut rng = StdRng::seed_from_u64(seed);
            for _ in 0u32..1000 {
                let encode_me = rng.gen();
                let decode_me = encode_int(encode_me);
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

pub fn decode_int(data: &[u8]) -> u32 {
    let mut copy_of_data = data.to_vec();
    let mut get_mangled = copy_of_data.as_mut_ptr();
    //let out = unsafe { parse_int(data.as_mut_slice() as *mut [u8]) };
    unsafe { parse_int(&mut get_mangled) }
}

pub fn encode_int(v: u32) -> Vec<u8> {
    let mut buf = [0x81u8; 10]; // blub
    let mut buf_start = buf.as_mut_ptr();
    let count = unsafe {
        let buf_end = ffi::command::encode_int(buf_start, v);
        assert!(
            buf_end > buf_start,
            "encode_int returned a pointer with negative offset"
        );
        buf_end.offset_from(buf_start) as usize
    };
    buf[..count].into()
}

mod cli;
mod configfile;
mod kinematics;
mod mcu;
mod msgblock;
mod serialqueue;

// TODO! make the unit system based on a feature?
//pub type Meter = dimensioned::si::Meter<f32>;
//pub type MeterPerSecond = dimensioned::si::MeterPerSecond<f32>;
#[repr(transparent)]
#[derive(derive_more::From)]
pub struct Millimeters(f32);
#[repr(transparent)]
#[derive(derive_more::From)]
pub struct MillimetersPerSecond(f32);

pub trait PrinterState: Display {}

pub struct Printer<P: PrinterState> {
    state: PhantomData<P>,
}

pub struct Startup;
pub struct Halted;
