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
            let (crc, payload) = if let [
                payload @ ..,
                crc1,
                crc2,
                0x7e
            ] = &buf[..total_len] {
                let crc = u16::from_be_bytes([*crc1, *crc2]);
                (crc, payload)
            } else {
                panic!("invalid trailer!")
            };
            assert_eq!(payload[1] & 0xF0, 0x10, "Expected a seq of 0 and magic of 0x10");
            let k_crc = klipper_ffi_crc(payload);
            let rust_crc = klipper_crc(payload);
            assert_eq!(k_crc, crc);
            assert_eq!(k_crc, rust_crc);
        });
    }
}
