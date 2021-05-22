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

    pub fn get(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
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
}

pub type Mmy<'a> = &'a mut Mem;
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
