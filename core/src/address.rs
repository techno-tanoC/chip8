pub fn addr(n1: u8, n2: u8, n3: u8) -> u16 {
    ((n1 as u16) << 8) + ((n2 as u16) << 4) + n3 as u16
}

pub fn fontaddr(n: u8) -> u16 {
    n as u16 * 5
}

pub fn var(x1: u8, x2: u8) -> u8 {
    ((x1 as u8) << 4) + x2 as u8
}

pub fn idx(x: u8) -> usize {
    x as usize
}