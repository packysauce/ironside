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
    let mut buf = vec![0u8; 64];
    my_encode_int(&mut buf, 1234)
}

fn my_encode_int(buf: &mut [u8], v: u32) {
    dbg!(&buf);
    let ended = unsafe {
        let mangle_me = buf.as_mut_ptr();
        let copy_of_ptr = mangle_me;
        let returned = ffi::command::encode_int(mangle_me, v);
        println!(
            "mangle: {:?}, copy: {:?}, returned: {:?}",
            mangle_me, copy_of_ptr, returned
        );
    };
    dbg!(&buf);
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
