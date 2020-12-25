#[derive(Clone)]
pub struct CanLine {
    pub id: u32,
    bytes: [u8; 8],
}

impl CanLine {
    pub fn new(line: &str) -> CanLine {
        let mut id: u32 = 0;
        let mut bytes: [u8; 8] = [0; 8];
        let mut i = 0;
        for value in line.split(";") {
            if i == 0 {
                match value.parse::<u32>() {
                    Ok(parsed_id) => {
                        id = parsed_id;
                    }
                    _ => {}
                }
            } else {
                match value.parse::<u8>() {
                    Ok(val) => {
                        bytes[i - 1] = val;
                    }
                    _ => {}
                }
            }
            i += 1;
        }

        CanLine {
            id: id,
            bytes: bytes,
        }
    }

    pub fn get_value(self, byte_start: u8, byte_end: u8) -> f64 {
        let mut vals: [u8; 8] = [0; 8];
        let mut i = 0;
        let dif = byte_end - byte_start;

        for byte in &self.bytes {
            if i >= byte_start && i <= byte_end {
                vals[(7 - dif + (i - byte_start)) as usize] = *byte;
            }
            i += 1;
        }

        i64::from_be_bytes(vals) as f64
    }
}
