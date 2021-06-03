use crate::utils::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub trait MBC {
    fn new(path: &Path) -> Box<Self>
    where
        Self: Sized;
    fn get(&self, addr: u16, su: bool) -> Option<u8>;
    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()>;
}

pub struct MBC0();

impl MBC for MBC0 {
    fn new(path: &Path) -> Box<Self> {
        Box::new(MBC0())
    }

    fn get(&self, addr: u16, su: bool) -> Option<u8> {
        None
    }

    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()> {
        None
    }
}

pub struct MBC1 {
    cart: Vec<u8>,
    rom_nb: usize,
}

impl MBC for MBC1 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC1 {
            cart: vec![],
            rom_nb: 0x01,
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.cart)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result
    }

    fn get(&self, addr: u16, su: bool) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => {
                return Some(self.cart[addr as usize - 0x4000 + self.rom_nb * 0x4000]);
            }
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()> {
        match addr {
            0x2000..=0x3fff => {
                self.rom_nb = val as usize & 0x3;
                if self.rom_nb == 0 {
                    self.rom_nb = 1;
                }
                return Some(());
            }
            _ => None,
        }
    }
}
