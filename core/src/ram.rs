use super::error::Error;
use std::io::Read;

const RAM_SIZE: usize = 1 << 12;

pub struct Ram {
    pub(crate) buf: [u8; RAM_SIZE],
}

impl Ram {
    const FONTSET: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub fn new() -> Self {
        Ram { buf: [0; RAM_SIZE] }
    }

    pub fn load<R: Read>(&mut self, r: &mut R) -> Result<(), Error> {
        self.load_fontset();
        loop {
            if r.read(&mut self.buf[0x200..])? == 0 {
                break;
            }
        }
        Ok(())
    }

    fn load_fontset(&mut self) {
        self.buf[..Self::FONTSET.len()].copy_from_slice(&Self::FONTSET);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ram = Ram::new();
        assert_eq!(ram.buf[..], [0; 4096][..]);
    }

    #[test]
    fn test_load_fontset() {
        let mut ram = Ram::new();
        ram.load_fontset();
        assert_eq!(ram.buf[..80], Ram::FONTSET[..]);
    }
}
