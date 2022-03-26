use std::path::PathBuf;
use std::process::Command;
use std::process::{ChildStdin, ChildStdout, Stdio};

#[cfg(test)]
fn in_simulator(test_fn: impl Fn(ChildStdin, ChildStdout)) {
    let base_path = env!("CARGO_MANIFEST_DIR");
    let sim_path: PathBuf = [base_path, "klipper", "out", "klipper.elf"]
        .iter()
        .collect();
    let mut simulator = Command::new(&sim_path)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn simulator");

    let stdout = simulator.stdout.take().unwrap();
    let stdin = simulator.stdin.take().unwrap();

    test_fn(stdin, stdout);
    // simulator dies because stdin closes, not because Drop killed process
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter, Read};

    use hexdump::hexdump;
    use nom::bytes::streaming::take;

    use crate::msgblock::{klipper_crc, klipper_ffi_crc};

    use super::*;

    use std::convert::TryInto;

    fn read_le_u16(input: &[u8]) -> u16 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
        u16::from_le_bytes(int_bytes.try_into().unwrap())
    }

    fn read_be_u16(input: &[u8]) -> u16 {
        let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
        u16::from_be_bytes(int_bytes.try_into().unwrap())
    }

    /// Just fiddle with the simulator to check it works
    #[test]
    fn test_handshake() {
        in_simulator(|inp, out| {
            let mut reader = BufReader::new(out);
            let mut writer = BufWriter::new(inp);

            // klipper's got a single byte msg length
            let mut buf = [0u8; 255];
            reader.read_exact(&mut buf[..1]).unwrap();
            let total_len = buf[0] as usize;
            assert!(total_len >= 5, "got impossibly small len");

            reader.read_exact(&mut buf[1..total_len]).unwrap();
            hexdump(&buf[..total_len]);
            let payload_len = total_len - 3;
            let (packet, trailer) = buf[..].split_at(payload_len);
            // the seq is the low 4 bytes...
            let seq = packet[1] & 0xF;
            // and the high is reserved
            let reserved = packet[1] >> 4;
            let crc = if let [crc1, crc2, 0x7e, ..] = *trailer {
                u16::from_be_bytes([crc1, crc2])
            } else {
                panic!("invalid trailer!")
            };
            assert_eq!(trailer[2], 0x7e, "sync byte incorrect");
            assert_eq!(packet[1] & 0xF0, 0x10, "Expected a seq of 0 and magic of 0x10");
            let k_crc = klipper_ffi_crc(packet);
            let rust_crc = klipper_crc(packet);
            assert_eq!(k_crc, crc);
            assert_eq!(k_crc, rust_crc);
        });
    }
}
