use super::Millimeters;
use enumflags2::BitFlags;
use std::time::Instant;

/// Known travelling axes
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    X = 0b0001,
    Y = 0b0010,
    Z = 0b0100,
}

pub struct Stepper {
    /// Amount the stepper moves per step
    step_distance: Millimeters,
    /// Position this stepper has been commanded to
    position: Millimeters,
    /// Time of last trapq flush
    last_flushed: Instant,
    /// Time of last stepper activity
    last_moved: Instant,
    /// Currently active axes
    active: BitFlags<Axis>,
    /// Steps generated right before the stepper goes active
    leading_steps: f64,
    /// Steps generated right after the stepper goes active
    trailing_steps: f64,
    // stepcompress: StepCompressor?
    // trapq
}

pub struct Cartesian {
    x: Stepper,
    y: Stepper,
    z: Stepper,
}

pub struct CoreXY;
pub struct CoreXZ;
pub struct Winch;

pub trait Kinematics {
    type Position;
    fn calculate_position(&self, next_time: Instant) -> Self::Position;
    fn generate_steps(&mut self) -> Vec<Instant>;
    fn will_be_active(&self, at: Instant) -> Axis;
    fn active_axes(&self) -> BitFlags<Axis>;
}
