use std::fmt::Display;
use std::marker::PhantomData;

mod cli;
mod data;
mod ffi;
mod kinematics;
mod mcu;
mod msgblock;
#[cfg(test)]
mod testutils;
pub mod proto;
//mod serialqueue;

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
