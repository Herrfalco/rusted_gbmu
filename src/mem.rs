use crate::input::*;
use crate::mbc::*;
use crate::reg::api::*;
use crate::utils::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MEM_SZ: usize = 0x10000;

pub struct Mem {
    pub data: Vec<u8>,
    pub inputs: Inputs,
    mbc: Box<dyn MBC>,
}

impl Mem {
    pub fn new(path: &str) -> Mem {
        let mut result = Mem {
            data: vec![0; MEM_SZ],
            inputs: Inputs::new(),
            mbc: MBC0::new(Path::new("")),
        };
        if path != "" {
            if let Err(msg) = result.load_rom(0x8000, Path::new(path)) {
                fatal_err(msg, 2);
            }
            if let Err(_) = result.load_rom(0x100, Path::new("../roms/DMG_ROM.gb")) {
                fatal_err("Can't load bootrom", 11);
            }
            result.mbc = match result.data[0x147] {
                0x01 | 0x02 | 0x03 => MBC1::new(Path::new(path)),
                0x05 | 0x06 => MBC2::new(Path::new(path)),
                0x0f | 0x10 | 0x11 => MBC3::new(Path::new(path)),
                0x19 | 0x1a | 0x1b | 0x1c | 0x1d | 0x1e => MBC5::new(Path::new(path)),
                _ => MBC0::new(Path::new(path)),
            };
        }
        result
    }

    fn dma(&mut self, val: u8) {
        let tmp = (val as u16) << 8;

        for i in 0x0..=0x9f {
            self.su_set(0xfe00 | i, self.su_get(tmp | i));
        }
    }

    pub fn nu_get(&self, addr: u16) -> u8 {
        self.get(addr, false)
    }

    pub fn su_get(&self, addr: u16) -> u8 {
        self.get(addr, true)
    }

    pub fn get(&self, addr: u16, su: bool) -> u8 {
        if let Some(res) = self.mbc.get(addr) {
            res
        } else {
            if !su {
                match addr {
                    P1 => return Inputs::get_p1(self),
                    0xfe00..=0xfeff => match self.data[STAT as usize] & 0x3 {
                        2 | 3 => return 0xff,
                        _ => (),
                    },
                    _ => (),
                }
            }
            self.data[addr as usize]
        }
    }

    pub fn nu_set(&mut self, addr: u16, val: u8) {
        self.set(addr, val, false);
    }

    pub fn su_set(&mut self, addr: u16, val: u8) {
        self.set(addr, val, true);
    }

    pub fn set(&mut self, addr: u16, val: u8, su: bool) {
        if let Some(_) = self.mbc.set(addr, val) {
        } else {
            let mut tmp = val;

            if !su {
                match addr {
                    DIV => tmp = 0,
                    DMA => return self.dma(val),
                    0xfe00..=0xfe9f => match self.data[STAT as usize] & 0x3 {
                        2 | 3 => return,
                        _ => (),
                    },
                    _ => (),
                }
            }
            self.data[addr as usize] = tmp;
        }
    }

    pub fn load_rom(&mut self, len: usize, path: &Path) -> Result<(), &str> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err("Can't open rom"),
        };

        if let Err(_) = file.read(&mut self.data[..len]) {
            return Err("Can't read from rom");
        }
        Ok(())
    }

    pub fn init_spe_reg(&mut self) {
        self.su_set(DIV, 0x00);
        self.su_set(TIMA, 0x00);
        self.su_set(TMA, 0x00);
        self.su_set(TAC, 0x00);
        self.su_set(NR10, 0x80);
        self.su_set(NR11, 0xbf);
        self.su_set(NR12, 0xf3);
        self.su_set(NR14, 0xbf);
        self.su_set(NR21, 0x3f);
        self.su_set(NR22, 0x00);
        self.su_set(NR24, 0xbf);
        self.su_set(NR30, 0x7f);
        self.su_set(NR31, 0xff);
        self.su_set(NR32, 0x9f);
        self.su_set(NR34, 0xbf);
        self.su_set(NR41, 0xff);
        self.su_set(NR42, 0x00);
        self.su_set(NR43, 0x00);
        self.su_set(NR44, 0xbf);
        self.su_set(NR50, 0x77);
        self.su_set(NR51, 0xf3);
        self.su_set(NR52, 0xf1);
        self.su_set(LCDC, 0x91);
        self.su_set(SCY, 0x00);
        self.su_set(SCX, 0x00);
        self.su_set(LYC, 0x00);
        self.su_set(BGP, 0xfc);
        self.su_set(OBP0, 0xff);
        self.su_set(OBP1, 0xff);
        self.su_set(WY, 0x00);
        self.su_set(WX, 0x00);
        self.su_set(IE, 0x00);
    }
}

pub type MMy<'a> = &'a mut Mem;
pub type My<'a> = &'a Mem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mem = Mem::new("");

        assert_eq!(mem.data.len(), 0x10000);
        for byte in mem.data {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn access() {
        let mut mem = Mem::new("");

        assert_eq!(mem.su_get(0), 0);
        assert_eq!(mem.su_get(0xffff), 0);
        mem.su_set(0, 0x11);
        mem.su_set(0xaa, 0xaa);
        mem.su_set(0xfff, 0xbb);
        mem.su_set(0xffff, 0xff);
        assert_eq!(mem.su_get(0), 0x11);
        assert_eq!(mem.su_get(0xaa), 0xaa);
        assert_eq!(mem.su_get(0xfff), 0xbb);
        assert_eq!(mem.su_get(0xffff), 0xff);
    }

    #[test]
    fn rom_load() {
        let mut mem = Mem::new("");
        let first_bytes = [
            0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x8b, 0x02, 0xff,
        ];

        assert!(mem.load_rom(12, Path::new("???")).is_err());
        assert!(mem.load_rom(12, Path::new("./roms/Tetris.gb")).is_ok());
        assert_eq!(first_bytes, mem.data[..12]);
        assert_eq!(mem.data[12..15], [0, 0, 0]);
    }
}
