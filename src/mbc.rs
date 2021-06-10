use crate::header::*;
use crate::utils::*;
use chrono::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub trait MBC {
    fn new(path: &Path) -> Box<Self>
    where
        Self: Sized;
    fn get(&self, addr: u16) -> Option<u8>;
    fn set(&mut self, addr: u16, val: u8) -> Option<()>;
}

pub struct MBC0();

#[allow(unused_variables)]
impl MBC for MBC0 {
    fn new(path: &Path) -> Box<Self> {
        Box::new(MBC0())
    }

    fn get(&self, addr: u16) -> Option<u8> {
        None
    }

    fn set(&mut self, addr: u16, val: u8) -> Option<()> {
        None
    }
}

pub struct MBC1 {
    rom: Vec<u8>,
    rom_sz: usize,
    rom_nb: usize,
    ram: Vec<u8>,
    ram_sz: usize,
    ram_nb: usize,
    ram_en: bool,
}

impl MBC for MBC1 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC1 {
            rom: vec![],
            rom_sz: 0,
            rom_nb: 0x01,
            ram: vec![],
            ram_sz: 0,
            ram_nb: 0x0,
            ram_en: false,
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.rom)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result.rom_sz = result.rom[0x148] as usize;
        result.ram_sz = result.rom[0x149] as usize;
        result.ram = vec![0; RAM_SZ[result.ram_sz] / 8 * 0x2000];
        result
    }

    fn get(&self, addr: u16) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => Some(self.rom[addr as usize - 0x4000 + self.rom_nb * 0x4000]),
            0xa000..=0xbfff if self.ram_sz != 0 && self.ram_en => {
                Some(self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000])
            }
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Option<()> {
        match addr {
            0x0000..=0x1fff => {
                self.ram_en = val & 0xf == 0xa;
            }
            0x2000..=0x3fff => {
                self.rom_nb = val as usize & ((0x1 << (self.rom_sz + 1)) - 1);
                if self.rom_nb == 0 {
                    self.rom_nb += 1;
                }
            }
            0x4000..=0x5fff if self.ram_sz != 0 => {
                self.ram_nb = val as usize & 0x3;
            }
            0xa000..=0xbfff if self.ram_sz != 0 => {
                if self.ram_en {
                    self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000] = val;
                }
            }
            _ => return None,
        }
        Some(())
    }
}

pub struct MBC2 {
    rom: Vec<u8>,
    rom_nb: usize,
    ram: Vec<u8>,
    ram_en: bool,
}

impl MBC for MBC2 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC2 {
            rom: vec![],
            rom_nb: 0x1,
            ram: vec![],
            ram_en: false,
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.rom)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result.ram = vec![0; 0x200];
        result
    }

    fn get(&self, addr: u16) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => Some(self.rom[addr as usize - 0x4000 + self.rom_nb * 0x4000]),
            0xa000..=0xbfff if self.ram_en => Some(self.ram[addr as usize & 0x1ff]),
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Option<()> {
        match addr {
            0x0000..=0x3fff => {
                if addr & 0x100 == 0 {
                    self.ram_en = val & 0xf == 0xa;
                } else {
                    self.rom_nb = if val & 0xf == 0 {
                        0x1
                    } else {
                        val as usize & 0xf
                    };
                }
            }
            0xa000..=0xbfff => {
                if self.ram_en {
                    self.ram[addr as usize & 0x1ff] = val & 0xf;
                }
            }
            _ => return None,
        }
        Some(())
    }
}

enum RamClk {
    RAM,
    S,
    M,
    H,
    DL,
    DH,
}

pub struct MBC3 {
    rom: Vec<u8>,
    rom_nb: usize,
    ram: Vec<u8>,
    ram_sz: usize,
    ram_nb: usize,
    ram_en: bool,
    lat_clk: bool,
    ram_clk: RamClk,
    loc_tm: DateTime<Local>,
}

impl MBC for MBC3 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC3 {
            rom: vec![],
            rom_nb: 0x01,
            ram: vec![],
            ram_sz: 0,
            ram_nb: 0x0,
            ram_en: false,
            lat_clk: true,
            ram_clk: RamClk::RAM,
            loc_tm: Local::now(),
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.rom)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result.ram_sz = result.rom[0x149] as usize;
        result.ram = vec![0; RAM_SZ[result.ram_sz] / 8 * 0x2000];
        result
    }

    fn get(&self, addr: u16) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => Some(self.rom[addr as usize - 0x4000 + self.rom_nb * 0x4000]),
            0xa000..=0xbfff if self.ram_en => match self.ram_clk {
                RamClk::RAM => {
                    if self.ram_sz == 0 {
                        None
                    } else {
                        Some(self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000])
                    }
                }
                RamClk::S => Some(self.now(RamClk::S)),
                RamClk::M => Some(self.now(RamClk::M)),
                RamClk::H => Some(self.now(RamClk::H)),
                RamClk::DL => Some(self.now(RamClk::DL)),
                RamClk::DH => Some(self.now(RamClk::DH)),
            },
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Option<()> {
        match addr {
            0x0000..=0x1fff => {
                self.ram_en = val & 0xf == 0xa;
            }
            0x2000..=0x3fff => {
                self.rom_nb = val as usize & 0x7f;
                if self.rom_nb == 0 {
                    self.rom_nb += 1;
                }
            }
            0x4000..=0x5fff => match val {
                0x00..=0x03 => {
                    self.ram_clk = RamClk::RAM;
                    self.ram_nb = val as usize;
                }
                0x08..=0x0c => {
                    self.ram_clk = match val {
                        0x08 => RamClk::S,
                        0x09 => RamClk::M,
                        0x0a => RamClk::H,
                        0x0b => RamClk::DL,
                        0x0c => RamClk::DH,
                        _ => RamClk::RAM,
                    }
                }
                _ => (),
            },
            0x6000..=0x7fff => match val {
                0x00 if self.lat_clk => self.lat_clk = false,
                0x01 if !self.lat_clk => {
                    self.lat_clk = true;
                    self.loc_tm = Local::now();
                }
                _ => (),
            },
            0xa000..=0xbfff => {
                if let RamClk::RAM = self.ram_clk {
                    if self.ram_sz == 0 {
                        return None;
                    } else if self.ram_en {
                        self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000] = val;
                    }
                }
            }
            _ => return None,
        }
        Some(())
    }
}

impl MBC3 {
    fn now(&self, typ: RamClk) -> u8 {
        match typ {
            RamClk::S => self.loc_tm.second() as u8,
            RamClk::M => self.loc_tm.minute() as u8,
            RamClk::H => self.loc_tm.hour() as u8,
            RamClk::DL => self.loc_tm.ordinal0() as u8,
            RamClk::DH => ((self.loc_tm.ordinal0() >> 8) & 0x1) as u8,
            _ => 0,
        }
    }
}

pub struct MBC5 {
    rom: Vec<u8>,
    rom_nb: usize,
    ram: Vec<u8>,
    ram_sz: usize,
    ram_nb: usize,
    ram_en: bool,
}

impl MBC for MBC5 {
    fn new(path: &Path) -> Box<Self> {
        let mut result = Box::new(MBC5 {
            rom: vec![],
            rom_nb: 0x01,
            ram: vec![],
            ram_sz: 0,
            ram_nb: 0x0,
            ram_en: false,
        });
        let mut file = File::open(path).unwrap_or_else(|_| fatal_err("Can't open rom", 99));

        file.read_to_end(&mut result.rom)
            .unwrap_or_else(|_| fatal_err("Can't read from rom", 102));
        result.ram_sz = result.rom[0x149] as usize;
        result.ram = vec![0; RAM_SZ[result.ram_sz] / 8 * 0x2000];
        result
    }

    fn get(&self, addr: u16) -> Option<u8> {
        match addr {
            0x4000..=0x7fff => Some(self.rom[addr as usize - 0x4000 + self.rom_nb * 0x4000]),
            0xa000..=0xbfff if self.ram_sz != 0 && self.ram_en => {
                Some(self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000])
            }
            _ => None,
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Option<()> {
        match addr {
            0x0000..=0x1fff => {
                self.ram_en = val & 0xf == 0xa;
            }
            0x2000..=0x2fff => {
                self.rom_nb = (self.rom_nb & 0x100) | val as usize;
            }
            0x3000..=0x3fff => {
                self.rom_nb = (self.rom_nb & 0xff) | (val as usize & 0x1 << 8);
            }
            0x4000..=0x5fff if self.ram_sz != 0 => {
                self.ram_nb = val as usize;
            }
            0xa000..=0xbfff if self.ram_sz != 0 => {
                if self.ram_en {
                    self.ram[addr as usize - 0xa000 + self.ram_nb * 0x2000] = val;
                }
            }
            _ => return None,
        }
        Some(())
    }
}
