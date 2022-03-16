pub enum Clock {
    Command { minimum: u64, required: u64 },
    Send { sent: f64, received: f64 },
}

const MSG_START: usize = 2;

struct Message {
    len: u32,
    msg: Vec<u8>,
    clock: Clock,
    notify_id: u64,
    // list node huh
}

/// Wrapper around the klipper `msgblock_crc16_ccitt` function
pub fn klipper_ffi_crc(buf: &[u8]) -> u16 {
    use crate::ffi::generated::msgblock_crc16_ccitt;
    unsafe {
        msgblock_crc16_ccitt(
            buf.as_ptr() as *mut u8,
            buf.len()
                .try_into()
                .expect("u8 overlfow in klipper_ffi_crc"),
        )
    }
}

/// Klipper's (transliterated) fn
pub fn klipper_crc(buf: &[u8]) -> u16 {
    let mut crc: u16 = 0xffff;
    for mut data in buf.iter().copied() {
        data ^= crc as u8;
        data ^= data << 4;
        // crc = ((((uint16_t)data << 8) | (crc >> 8)) ^ (uint8_t)(data >> 4)
        //        ^ ((uint16_t)data << 3));
        crc = (((data as u16) << 8) | (crc >> 8)) ^ ((data >> 4) as u16) ^ ((data as u16) << 3);
    }
    crc
}

#[cfg(test)]
mod tests {
    use crc_any::CRCu16;

    use super::*;
    #[test]
    fn test_msgblocks() {
        let inputs = vec![
            b"123456789".to_vec(),
            b"8902haikjhgboiuabskdjbagh".to_vec(),
            b"iuh3iuahdsgji".to_vec(),
            b"23098hjasdghiajsdhf ahesd q98w3h a".to_vec(),
        ];
        for input in inputs {
            let mut crc = CRCu16::crc16mcrf4cc();
            crc.digest(&input);
            let klipper_rust = klipper_crc(&input);
            let klipper_c = klipper_ffi_crc(&input);
            assert_eq!(crc.get_crc(), klipper_rust);
            assert_eq!(crc.get_crc(), klipper_c);
        }
    }
}
