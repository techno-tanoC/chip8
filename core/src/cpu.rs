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
}
