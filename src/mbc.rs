use crate::mem::*;

pub trait MBC {
    fn get(&self, addr: u16, su: bool) -> Option<u8>;
    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()>;
}

pub struct MBC0();

impl MBC for MBC0 {
    fn get(&self, addr: u16, su: bool) -> Option<u8> {
        None
    }

    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()> {
        None
    }
}
