use super::ram::Ram;
use super::ret::Ret::*;
use super::address;

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
            // ???
            (0x0, 0x0, 0xE, 0xE) => {
                let pc = self.stack[self.sp as usize - 1];
                self.sp -= 1;
                Jump(pc + 2)
            },
            // 0nnn
            (0x0, n1, n2, n3) => {
                let nnn = address::addr(n1, n2, n3);
                Jump(nnn)
            },
            // 1nnn
            (0x1, n1, n2, n3) => {
                let nnn = address::addr(n1, n2, n3);
                Jump(nnn)
            },
            // 2nnn
            (0x2, n1, n2, n3) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                let nnn = address::addr(n1, n2, n3);
                Jump(nnn)
            },
            // 3xkk
            (0x3, x, k1, k2) => {
                let vx = self.v[address::idx(x)];
                let kk = address::var(k1, k2);
                if vx == kk {
                    Skip
                } else {
                    Next
                }
            },
            // 4xkk
            (0x4, x, k1, k2) => {
                let vx = self.v[address::idx(x)];
                let kk = address::var(k1, k2);
                if vx != kk {
                    Skip
                } else {
                    Next
                }
            },
            // 5xy0
            (0x5, x, y, 0x0) => {
                let vx = self.v[address::idx(x)];
                let vy = self.v[address::idx(y)];
                if vx == vy {
                    Skip
                } else {
                    Next
                }
            },
            // 6xkk
            (0x6, x, k1, k2) => {
                let kk = address::var(k1, k2);
                self.v[address::idx(x)] = kk;
                Next
            },
            // 7xkk
            (0x7, x, k1, k2) => {
                let kk = address::var(k1, k2);
                let x = address::idx(x);
                // ???
                self.v[x] = self.v[x].overflowing_add(kk).0;
                Next
            },
            // 8xy0
            (0x8, x, y, 0x0) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                self.v[ix] = self.v[iy];
                Next
            },
            // 8xy1
            (0x8, x, y, 0x1) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                self.v[ix] |= self.v[iy];
                Next
            },
            // 8xy2
            (0x8, x, y, 0x2) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                self.v[ix] &= self.v[iy];
                Next
            },
            // 8xy3
            (0x8, x, y, 0x3) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                self.v[ix] ^= self.v[iy];
                Next
            },
            // 8xy4
            (0x8, x, y, 0x4) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                let xy = self.v[ix] as u16 + self.v[iy] as u16;
                if xy > 0xff {
                    self.v[0xf] = 1;
                } else {
                    self.v[0xf] = 0;
                }
                self.v[ix] = (xy & 0xff) as u8;
                Next
            },
            // 8xy5
            (0x8, x, y, 0x5) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                let vx = self.v[ix];
                let vy = self.v[iy];
                let (val, overflow) = vx.overflowing_sub(vy);
                if !overflow {
                    self.v[0xff] = 1;
                } else {
                    self.v[0xff] = 0;
                }
                self.v[ix] = val;
                Next
            },
            // 8xy6
            (0x8, x, y, 0x6) => {
                let ix = address::idx(x);
                self.v[0xf] = self.v[ix] & 0x1;
                self.v[ix] >>= 1;
                Next
            },
            // 8xy7
            (0x8, x, y, 0x7) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                let vx = self.v[ix];
                let vy = self.v[iy];
                let (val, overflow) = vx.overflowing_sub(vy);
                if !overflow {
                    self.v[0xff] = 1;
                } else {
                    self.v[0xff] = 0;
                }
                self.v[ix] = val;
                Next
            },
            // 8xyE
            (0x8, x, y, 0xE) => {
                let ix = address::idx(x);
                self.v[0xf] = self.v[ix] >> 7;
                self.v[ix] = self.v[ix].overflowing_mul(2).0;
                Next
            },
            // 9xy0
            (0x9, x, y, 0x0) => {
                let vx = self.v[address::idx(x)];
                let vy = self.v[address::idx(y)];
                if vx != vy {
                    Skip
                } else {
                    Next
                }
            },
            // Annn
            (0xA, n1, n2, n3) => {
                self.i = address::addr(n1, n2, n3);
                Next
            },
            // Bnnn
            (0xB, n1, n2, n3) => {
                let pc = address::addr(n1, n2, n3) + self.v[0] as u16;
                Jump(pc)
            },
            // Cxkk
            (0xC, x, k1, k2) => {
                unimplemented!()
            },
            // Dxyn
            (0xD, x, y, n) => {
                let ix = address::idx(x);
                let iy = address::idx(y);
                let vx = self.v[ix];
                let vy = self.v[iy];
                let since = self.i as usize;
                let until = since + address::idx(n);
                let bytes = (&ram.buf[since..until]).to_vec();
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
                Next
            },
            // Fx1E
            (0xF, x, 0x1, 0xE) => {
                let ix = address::idx(x);
                self.i += self.v[ix] as u16;
                Next
            }
            // Fx29
            (0xF, x, 0x2, 0x9) => {
                let ix = address::idx(x);
                let vx = self.v[ix];
                self.i = address::fontaddr(vx);
                Next
            },
            // Fx33
            (0xF, x, 0x3, 0x3) => {
                let i = self.i as usize;
                let vx = self.v[address::idx(x)];
                ram.buf[i] = (vx / 100) as u8 % 10;
                ram.buf[i + 1] = (vx / 10) as u8 % 10;
                ram.buf[i + 2] = vx % 10;
                Next
            },
            // Fx55
            (0xF, x, 0x5, 0x5) => {
                for n in 0..(x + 1) {
                    let an = address::idx(n);
                    ram.buf[self.i as usize + an] = self.v[an];
                }
                Next
            },
            // Fx65
            (0xF, x, 0x6, 0x5) => {
                for n in 0..(x + 1) {
                    let an = address::idx(n);
                    self.v[an] = ram.buf[self.i as usize + an]
                }
                Next
            },
            _ => {
                panic!("N/A {:x}{:x}{:x}{:x}", o1, o2, o3, o4);
                Next
            }
        };
    }
}
