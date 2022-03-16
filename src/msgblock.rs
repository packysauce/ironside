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

/// Klipper's (transliterated) fn
pub fn msgblock_crc16_ccitt(buf: &[u8]) -> u16 {
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
            let klipper = msgblock_crc16_ccitt(&input);
            assert_eq!(crc.get_crc(), klipper)
        }
    }
}
