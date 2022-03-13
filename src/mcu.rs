use dimensioned::ucum::Radian;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Timer too close")]
    TimerTooClose,
    #[error("Missed scheduling of next {0}")]
    MissedSchedule(&'static str),
    #[error("ADC out of range")]
    AdcOutOfRange,
    #[error("Attempted to schedule event in the past")]
    TimeParadox, // couldnt think of a good name
}

pub struct Mcu;
pub struct Oid;

pub struct Stepper {
    name: String,
    rotation_distance: f64,
    steps_per_rotation: f64,
    step_pulse_duration: f64,
    units_in_radians: Radian<f64>,
    mcu: Mcu,
    oid: Oid,
    step: McuPin,
    dir: McuPin,
}

impl Stepper {
    fn step_dist(&self) -> f64 {
        self.rotation_distance / self.steps_per_rotation
    }
}

pub struct PinRef;

pub struct McuPin {
    pin: String,
    invert: bool,
}

// <1 byte length><1 byte sequence><n-byte content><2 byte crc><1 byte sync>

#[repr(C)]
struct RawMessage {
    len: u8,
    seq: u8,
    data: Vec<u8>,
    crc: u16,
    sync: u8,
}

#[repr(u8)]
pub enum Command {
    Identify,
    SetDigitalOut {
        pin: PinRef,
        value: u8,
    },
    SetPwmOut {
        pin: PinRef,
        cycle_ticks: usize,
        value: u8,
    },
}

#[repr(u8)]
pub enum Response {
    Identify = 1,
}
