#[derive(Debug)]
pub enum Ret {
    Next,
    Skip,
    Jump(u16)
}