#![allow(unused)]

use std::fmt::Display;
use std::marker::PhantomData;

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
#[test]
fn test_encode_int() {
    let encode_me = 1234;
    let out = my_encode_int(encode_me);
    let mut new_out = Vec::new();
    let r = vlq::encode(encode_me.into(), &mut new_out).unwrap();
    assert_eq!(out, new_out);
}

fn my_encode_int(v: u32) -> Vec<u8> {
    let mut buf = [0u8; 10];
    let buf_start = buf.as_mut_ptr();
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
