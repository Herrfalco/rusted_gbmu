use crate::reg::api::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MEM_SZ: usize = 0x10000;

pub struct Mem {
    data: Vec<u8>,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            data: vec![0; MEM_SZ],
        }
    }

    fn dma(&mut self, val: u8) {
        let tmp = (val as u16) << 8;

        for i in 0x0..0x9f {
            self.oam_set(0xfe00 | i, self.get(tmp | i), true);
        }
    }

    pub fn get(&self, addr: u16) -> u8 {
        self.oam_get(addr, false)
    }

    pub fn oam_get(&self, addr: u16, oam: bool) -> u8 {
        match addr {
            P1 => return 0xff,
            0xfe00..=0xfeff if !oam => match self.data[STAT as usize] & 0x3 {
                2 | 3 => return 0xff,
                _ => (),
            },
            _ => (),
        }
        return self.data[addr as usize];
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        self.oam_set(addr, val, false);
    }

    pub fn oam_set(&mut self, addr: u16, val: u8, oam: bool) {
        let mut tmp = val;

        match addr {
            DIV => tmp = 0,
            DMA => return self.dma(val),
            0xfe00..=0xfe9f if !oam => match self.data[STAT as usize] & 0x3 {
                2 | 3 => return,
                _ => (),
            },
            _ => (),
        }
        self.data[addr as usize] = tmp;
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
        self.set(DIV, 0x00);
        self.set(TIMA, 0x00);
        self.set(TMA, 0x00);
        self.set(TAC, 0x00);
        self.set(NR10, 0x80);
        self.set(NR11, 0xbf);
        self.set(NR12, 0xf3);
        self.set(NR14, 0xbf);
        self.set(NR21, 0x3f);
        self.set(NR22, 0x00);
        self.set(NR24, 0xbf);
        self.set(NR30, 0x7f);
        self.set(NR31, 0xff);
        self.set(NR32, 0x9f);
        self.set(NR34, 0xbf);
        self.set(NR41, 0xff);
        self.set(NR42, 0x00);
        self.set(NR43, 0x00);
        self.set(NR44, 0xbf);
        self.set(NR50, 0x77);
        self.set(NR51, 0xf3);
        self.set(NR52, 0xf1);
        self.set(LCDC, 0x91);
        self.set(SCY, 0x00);
        self.set(SCX, 0x00);
        self.set(LYC, 0x00);
        self.set(BGP, 0xfc);
        self.set(OBP0, 0xff);
        self.set(OBP1, 0xff);
        self.set(WY, 0x00);
        self.set(WX, 0x00);
        self.set(IE, 0x00);
    }
}

pub type MMy<'a> = &'a mut Mem;
pub type My<'a> = &'a Mem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mem = Mem::new();

        assert_eq!(mem.data.len(), 0x10000);
        for byte in mem.data {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn access() {
        let mut mem = Mem::new();

        assert_eq!(mem.get(0), 0);
        assert_eq!(mem.get(0xffff), 0);
        mem.set(0, 0x11);
        mem.set(0xaa, 0xaa);
        mem.set(0xfff, 0xbb);
        mem.set(0xffff, 0xff);
        assert_eq!(mem.get(0), 0x11);
        assert_eq!(mem.get(0xaa), 0xaa);
        assert_eq!(mem.get(0xfff), 0xbb);
        assert_eq!(mem.get(0xffff), 0xff);
    }

    #[test]
    fn rom_load() {
        let mut mem = Mem::new();
        let first_bytes = [
            0xc3, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0x8b, 0x02, 0xff,
        ];

        assert!(mem.load_rom(12, Path::new("???")).is_err());
        assert!(mem.load_rom(12, Path::new("./roms/Tetris.gb")).is_ok());
        assert_eq!(first_bytes, mem.data[..12]);
        assert_eq!(mem.data[12..15], [0, 0, 0]);
    }
}
