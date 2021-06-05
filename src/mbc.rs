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
    rom_sz: usize,
    ram_sz: usize,
    rom_nb: usize,
    ram_nb: usize,
    ram_en: bool,
}

impl MBC for MBC1 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC1 {
            cart: vec![],
            rom_sz: 0,
            ram_sz: 0,
            rom_nb: 0x01,
            ram_nb: 0x0,
            ram_en: false,
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.cart)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result.rom_sz = result.cart[0x148] as usize;
        result.ram_sz = result.cart[0x149] as usize;
        println!(">>>{}", result.cart.len());
        result
    }

    fn get(&self, addr: u16, su: bool) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => {
                return Some(self.cart[addr as usize - 0x4000 + self.rom_nb * 0x4000]);
            }
            0xa000..=0xbfff => {
                return if !self.ram_en {
                    None
                } else {
                    //need to split rom and ram
                    Some(self.cart[addr as usize - 0xa000 + self.ram_nb * 0x2000])
                };
            }
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8, su: bool) -> Option<()> {
        match addr {
            0x0000..=0x1fff => {
                self.ram_en = val & 0xf == 0xa;
            }
            0x2000..=0x3fff => {
                self.rom_nb = val as usize & ((0x1 << (self.rom_sz + 1)) - 1);
                if self.rom_nb == 0 {
                    self.rom_nb = 1;
                }
            }
            0x4000..=0x5fff => {
                self.ram_nb = val as usize & 0x3;
            }
            _ => return None,
        }
        Some(())
    }
}
