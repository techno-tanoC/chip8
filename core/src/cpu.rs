use super::ram::Ram;
use super::ret::Ret::*;

#[derive(Debug)]
pub struct Cpu {
    // general purpose registers
    v: [u8; 16],
    // index register
    i: u16,
    // program counter
    pc: u16,
    // stack pointer
    sp: u8,
    // stack
    stack: [u16; 16],
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn tick(&mut self, ram: &mut Ram) {
        let pc = self.pc as usize;
        let o1: u8 = ram.buf[pc] >> 4;
        let o2: u8 = ram.buf[pc] & 0xF;
        let o3: u8 = ram.buf[pc + 1] >> 4;
        let o4: u8 = ram.buf[pc + 1] & 0xF;

        let ret = match (o1, o2, o3, o4) {
            // 00E0
            (0x0, 0x0, 0xE, 0x0) => {
                unimplemented!()
            },
            // 00EE
            (0x0, 0x0, 0xE, 0xE) => {
                unimplemented!()
            },
            // 0nnn
            (0x0, n1, n2, n3) => {
                unimplemented!()
            },
            // 1nnn
            (0x1, n1, n2, n3) => {
                unimplemented!()
            },
            // 2nnn
            (0x2, n1, n2, n3) => {
                unimplemented!()
            },
            // 3xkk
            (0x3, x, k1, k2) => {
                unimplemented!()
            },
            // 4xkk
            (0x4, x, k1, k2) => {
                unimplemented!()
            },
            // 5xy0
            (0x5, x, y, 0x0) => {
                unimplemented!()
            },
            // 6xkk
            (0x6, x, k1, k2) => {
                unimplemented!()
            },
            // 7xkk
            (0x7, x, k1, k2) => {
                unimplemented!()
            },
            // 8xy0
            (0x8, x, y, 0x0) => {
                unimplemented!()
            },
            // 8xy1
            (0x8, x, y, 0x1) => {
                unimplemented!()
            },
            // 8xy2
            (0x8, x, y, 0x2) => {
                unimplemented!()
            },
            // 8xy3
            (0x8, x, y, 0x3) => {
                unimplemented!()
            },
            // 8xy4
            (0x8, x, y, 0x4) => {
                unimplemented!()
            },
            // 8xy5
            (0x8, x, y, 0x5) => {
                unimplemented!()
            },
            // 8xy6
            (0x8, x, y, 0x6) => {
                unimplemented!()
            },
            // 8xy7
            (0x8, x, y, 0x7) => {
                unimplemented!()
            },
            // 8xyE
            (0x8, x, y, 0xE) => {
                unimplemented!()
            },
            // 9xy0
            (0x9, x, y, 0x0) => {
                unimplemented!()
            },
            // Annn
            (0xA, n1, n2, n3) => {
                unimplemented!()
            },
            // Bnnn
            (0xB, n1, n2, n3) => {
                unimplemented!()
            },
            // Cxkk
            (0xC, x, k1, k2) => {
                unimplemented!()
            },
            // Dxyn
            (0xD, x, y, n) => {
                unimplemented!()
            },
            // Ex9E
            (0xE, x, 0x9, 0xE) => {
                unimplemented!()
            },
            // ExA1
            (0xE, x, 0xA, 0x1) => {
                unimplemented!()
            },
            // Fx07
            (0xF, x, 0x0, 0x7) => {
                unimplemented!()
            },
            // Fx0A
            (0xF, x, 0x0, 0xA) => {
                unimplemented!()
            },
            // Fx15
            (0xF, x, 0x1, 0x5) => {
                unimplemented!()
            },
            // Fx18
            (0xF, x, 0x1, 0x8) => {
                unimplemented!()
            },
            // Fx1E
            (0xF, x, 0x1, 0xE) => {
                unimplemented!()
            }
            // Fx29
            (0xF, x, 0x2, 0x9) => {
                unimplemented!()
            },
            // Fx33
            (0xF, x, 0x3, 0x3) => {
                unimplemented!()
            },
            // Fx55
            (0xF, x, 0x5, 0x5) => {
                unimplemented!()
            },
            // Fx65
            (0xF, x, 0x6, 0x5) => {
                unimplemented!()
            },
            _ => {
                panic!("N/A {:x}{:x}{:x}{:x}", o1, o2, o3, o4);
                Next
            }
        };
    }
}
