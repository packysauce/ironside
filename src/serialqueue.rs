use crossbeam::channel::{unbounded, Receiver, Sender};
use std::ffi::OsStr;
use std::io::Read;

// use tokio_util::codec::{Decoder, Encoder};

pub struct FastReader {}

pub enum Message {}

/// Handles communicating with an mcu
pub struct SerialQueue {
    sender: Option<Sender<Message>>,
    receiver: Receiver<Message>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open serial port")]
    Serial(#[from] serial::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

impl SerialQueue {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender: Some(sender),
            receiver,
        }
    }

    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<()> {
        let mut port = serial::open(port)?;

        let mut buf = vec![0u8; 4096];
        while let Ok(msg) = port.read(&mut buf) {}
        Ok(())
    }
}
