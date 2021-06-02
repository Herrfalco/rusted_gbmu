use crate::utils::*;
use lazy_static::*;
use std::collections::HashMap;
use std::fmt::{self, Display};

lazy_static! {
    static ref MBCS: HashMap<u8, &'static str> = [
        (0x00, "ROM ONLY"),
        (0x01, "MBC1"),
        (0x02, "MBC1+RAM"),
        (0x03, "MBC1+RAM+BATTERY"),
        (0x05, "MBC2"),
        (0x06, "MBC2+BATTERY"),
        (0x08, "ROM+RAM"),
        (0x09, "ROM+RAM+BATTERY"),
        (0x0B, "MMM01"),
        (0x0C, "MMM01+RAM"),
        (0x0D, "MMM01+RAM+BATTERY"),
        (0x0F, "MBC3+TIMER+BATTERY"),
        (0x10, "MBC3+TIMER+RAM+BATTERY"),
        (0x11, "MBC3"),
        (0x12, "MBC3+RAM"),
        (0x13, "MBC3+RAM+BATTERY"),
        (0x19, "MBC5"),
        (0x1A, "MBC5+RAM"),
        (0x1B, "MBC5+RAM+BATTERY"),
        (0x1C, "MBC5+RUMBLE"),
        (0x1D, "MBC5+RUMBLE+RAM"),
        (0x1E, "MBC5+RUMBLE+RAM+BATTERY"),
        (0x20, "MBC6"),
        (0x22, "MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
        (0xFC, "POCKET CAMERA"),
        (0xFD, "BANDAI TAMA5"),
        (0xFE, "HuC3"),
        (0xFF, "HuC1+RAM+BATTERY"),
    ]
    .iter()
    .cloned()
    .collect();
    static ref N_LICS: HashMap<&'static str, &'static str> = [
        ("00", "None"),
        ("01", "Nintendo R&D1"),
        ("08", "Capcom"),
        ("13", "Electronic Arts"),
        ("18", "Hudson Soft"),
        ("19", "b-ai"),
        ("20", "kss"),
        ("22", "pow"),
        ("24", "PCM Complete"),
        ("25", "san-x"),
        ("28", "Kemco Japan"),
        ("29", "seta"),
        ("30", "Viacom"),
        ("31", "Nintendo"),
        ("32", "Bandai"),
        ("33", "Ocean/Acclaim"),
        ("34", "Konami"),
        ("35", "Hector"),
        ("37", "Taito"),
        ("38", "Hudson"),
        ("39", "Banpresto"),
        ("41", "Ubi Soft"),
        ("42", "Atlus"),
        ("44", "Malibu"),
        ("46", "angel"),
        ("47", "Bullet-Proof"),
        ("49", "irem"),
        ("50", "Absolute"),
        ("51", "Acclaim"),
        ("52", "Activision"),
        ("53", "American sammy"),
        ("54", "Konami"),
        ("55", "Hi tech entertainment"),
        ("56", "LJN"),
        ("57", "Matchbox"),
        ("58", "Mattel"),
        ("59", "Milton Bradley"),
        ("60", "Titus"),
        ("61", "Virgin"),
        ("64", "LucasArts"),
        ("67", "Ocean"),
        ("69", "Electronic Arts"),
        ("70", "Infogrames"),
        ("71", "Interplay"),
        ("72", "Broderbund"),
        ("73", "sculptured"),
        ("75", "sci"),
        ("78", "THQ"),
        ("79", "Accolade"),
        ("80", "misawa"),
        ("83", "lozc"),
        ("86", "Tokuma Shoten Intermedia"),
        ("87", "Tsukuda Original"),
        ("91", "Chunsoft"),
        ("92", "Video system"),
        ("93", "Ocean/Acclaim"),
        ("95", "Varie"),
        ("96", "Yonezawa/s’pal"),
        ("97", "Kaneko"),
        ("99", "Pack in soft"),
        ("A4", "Konami (Yu-Gi-Oh!)"),
    ]
    .iter()
    .cloned()
    .collect();
    static ref O_LICS: HashMap<u8, &'static str> = [
        (0x00, "none"),
        (0x01, "nintendo"),
        (0x08, "capcom"),
        (0x13, "electronic arts"),
        (0x18, "hudsonsoft"),
        (0x19, "b-ai"),
        (0x20, "kss"),
        (0x22, "pow"),
        (0x24, "pcm complete"),
        (0x25, "san-x"),
        (0x28, "kemco japan"),
        (0x29, "seta"),
        (0x30, "viacom"),
        (0x31, "nintendo"),
        (0x32, "bandia"),
        (0x33, "ocean/acclaim"),
        (0x34, "konami"),
        (0x35, "hector"),
        (0x37, "taito"),
        (0x38, "hudson"),
        (0x39, "banpresto"),
        (0x41, "ubi soft"),
        (0x42, "atlus"),
        (0x44, "malibu"),
        (0x46, "angel"),
        (0x47, "pullet-proof"),
        (0x49, "irem"),
        (0x50, "absolute"),
        (0x51, "acclaim"),
        (0x52, "activision"),
        (0x53, "american sammy"),
        (0x54, "konami"),
        (0x55, "hi tech entertainment"),
        (0x56, "ljn"),
        (0x57, "matchbox"),
        (0x58, "mattel"),
        (0x59, "milton bradley"),
        (0x60, "titus"),
        (0x61, "virgin"),
        (0x64, "lucasarts"),
        (0x67, "ocean"),
        (0x69, "electronic arts"),
        (0x70, "infogrames"),
        (0x71, "interplay"),
        (0x72, "broderbund"),
        (0x73, "sculptured"),
        (0x75, "sci"),
        (0x78, "t*hq"),
        (0x79, "accolade"),
        (0x80, "misawa"),
        (0x83, "lozc"),
        (0x86, "tokuma shoten i*"),
        (0x87, "tsukuda ori*"),
        (0x91, "chun soft"),
        (0x92, "video system"),
        (0x93, "ocean/acclaim"),
        (0x95, "varie"),
        (0x96, "yonezawa/s'pal"),
        (0x97, "kaneko"),
        (0x99, "pack in soft"),
    ]
    .iter()
    .cloned()
    .collect();
    static ref RAM_SZ: Vec<u8> = vec![0, 2, 8, 32, 128, 64,];
}

enum LIC {
    Old((u8, &'static HashMap<u8, &'static str>)),
    New((String, &'static HashMap<&'static str, &'static str>)),
}

enum CGB {
    NoCgb,
    Cgb,
    CgbOnly,
}

pub struct Header {
    title: String,
    manu: Option<String>,
    cgb: CGB,
    lic: LIC,
    sgb: bool,
    mbc: u8,
    rom_sz: usize,
    ram_sz: usize,
    jap: bool,
    vers: u8,
}

impl Header {
    pub fn new(rom: &Vec<u8>) -> Header {
        let mut result = Header {
            title: String::from_utf8(
                rom[0x134..=0x143]
                    .iter()
                    .cloned()
                    .filter(|x| x.is_ascii_graphic() || *x == 0x20)
                    .collect::<Vec<u8>>(),
            )
            .unwrap_or_else(|_| fatal_err("Can't read rom title", 46)),
            manu: String::from_utf8(rom[0x13f..=0x142].to_vec()).ok(),
            cgb: match rom[0x143] {
                0x80 => CGB::Cgb,
                0xc0 => CGB::CgbOnly,
                _ => CGB::NoCgb,
            },
            lic: if rom[0x14b] == 0x33 {
                LIC::New((
                    String::from_utf8(rom[0x144..=0x145].to_vec())
                        .unwrap_or_else(|_| fatal_err("Can't read rom licence", 47)),
                    &N_LICS,
                ))
            } else {
                LIC::Old((rom[0x14b], &O_LICS))
            },
            sgb: rom[0x146] == 0x3,
            mbc: rom[0x147],
            rom_sz: rom[0x148] as usize,
            ram_sz: rom[0x149] as usize,
            jap: rom[0x14a] == 0,
            vers: rom[0x14c],
        };
        match result.cgb {
            CGB::Cgb | CGB::CgbOnly => {
                result.title = String::from(&result.title[..result.title.len() - 4]);
            }
            _ => result.manu = None,
        }
        result
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "-------------------------------------------------------\n\
            Title: {}\n\
            \n\
            Manufacturer: {}\n\
            Licence: {}\n\
            Japan: {}\n\
            Version: {}\n\
            \n\
            CGB: {}\n\
            SGB: {}\n\
            MBC: {}\n\
            \n\
            Rom: {}\n\
            Ram: {}\n\
            -------------------------------------------------------",
            self.title,
            if let Some(man) = &self.manu {
                &man
            } else {
                "."
            },
            if let Some(lic) = match &self.lic {
                LIC::New(li) => li.1.get(&li.0[..]),
                LIC::Old(li) => li.1.get(&li.0),
            } {
                lic
            } else {
                "."
            },
            if self.jap { "yes" } else { "no" },
            self.vers,
            match self.cgb {
                CGB::NoCgb => "not supported",
                CGB::Cgb => "supported",
                CGB::CgbOnly => "only",
            },
            if self.sgb {
                "supported"
            } else {
                "not supported"
            },
            MBCS.get(&self.mbc).unwrap_or(&"."),
            format!(
                "{} KB, {} bank(s)",
                32 << self.rom_sz,
                2_u32.pow(self.rom_sz as u32 + 1),
            ),
            {
                if let Some(tmp) = RAM_SZ.get(self.ram_sz) {
                    format!("{} KB, {} bank(s)", tmp, tmp / 8)
                } else {
                    String::from(".")
                }
            }
        )
    }
}
