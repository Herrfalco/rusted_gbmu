use crate::mem::*;
use std::fmt;

pub struct Reg {
    val: u16,
}

impl Reg {
    pub fn new() -> Reg {
        Reg { val: 0 }
    }

    fn get_16(&self) -> u16 {
        self.val
    }

    fn set_16(&mut self, val: u16) {
        self.val = val;
    }

    fn get_8(&self, up: bool) -> u8 {
        if up {
            ((self.val & 0xff00) >> 8) as u8
        } else {
            self.val as u8
        }
    }

    fn set_8(&mut self, up: bool, val: u8) {
        if up {
            self.val &= 0x00ff;
            self.val |= (val as u16) << 8;
        } else {
            self.val &= 0xff00;
            self.val |= val as u16;
        }
    }

    fn get_bit(&self, mask: u16) -> bool {
        self.val & mask != 0
    }

    fn set_bit(&mut self, mask: u16, val: bool) {
        self.val &= !mask;
        if val {
            self.val |= mask;
        }
    }
}

pub struct Regs {
    pub af: Reg,
    pub bc: Reg,
    pub de: Reg,
    pub hl: Reg,
    pub pc: Reg,
    pub sp: Reg,
    pub ime: Reg,
}

impl Regs {
    pub fn new() -> Regs {
        Regs {
            af: Reg::new(),
            bc: Reg::new(),
            de: Reg::new(),
            hl: Reg::new(),
            pc: Reg::new(),
            sp: Reg::new(),
            ime: Reg::new(),
        }
    }

    pub fn init(&mut self, debug: bool) {
        self.af.set_16(0x01b0);
        self.bc.set_16(0x0013);
        self.de.set_16(0x00d8);
        self.hl.set_16(0x014d);
        self.pc.set_16(if debug { 0x100 } else { 0x0 });
        self.sp.set_16(0xfffe);
    }

    pub fn spe_to_str(&self, m: My) -> String {
        format!(
            "-------------------------------------------------------\n  \
            LCD:                Timer:              Interrupt:\n\n  \
            
            LCDC = 0x{:02x}         DIV  = 0x{:02x}         IME = {}\n  \
            STAT = 0x{:02x}         TIMA = 0x{:02x}         IE  = {:02x}\n  \
            SCY  = 0x{:02x}         TMA  = 0x{:02x}         IF  = {:02x}\n  \
            SCX  = 0x{:02x}         TAC  = 0x{:02x}\n  \
            LY   = 0x{:02x}\n  \
            LYC  = 0x{:02x}\n  \
            DMA  = 0x{:02x}\n  \
            BGP  = 0x{:02x}\n  \
            OBP0 = 0x{:02x}\n  \
            OBP1 = 0x{:02x}\n  \
            WY   = 0x{:02x}\n  \
            WX   = 0x{:02x}\n\
            -------------------------------------------------------",
            m.su_get(api::LCDC),
            m.su_get(api::DIV),
            api::grr(&self.ime) == 1,
            m.su_get(api::STAT),
            m.su_get(api::TIMA),
            m.su_get(api::IE),
            m.su_get(api::SCY),
            m.su_get(api::TMA),
            m.su_get(api::IF),
            m.su_get(api::SCX),
            m.su_get(api::TAC),
            m.su_get(api::LY),
            m.su_get(api::LYC),
            m.su_get(api::DMA),
            m.su_get(api::BGP),
            m.su_get(api::OBP0),
            m.su_get(api::OBP1),
            m.su_get(api::WY),
            m.su_get(api::WX),
        )
    }
}

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "-------------------------------------------------------\n \
            A=0x{:02x}     Z={:5}    N={:5}    H={:5}    CY={:5}\n \
            BC=0x{:04x}  DE=0x{:04x}  HL=0x{:04x}  SP=0x{:04x}  PC=0x{:04x}\n\
            -------------------------------------------------------",
            api::gr((&self.af, api::U)),
            api::gf((&self.af, api::Z)),
            api::gf((&self.af, api::N)),
            api::gf((&self.af, api::H)),
            api::gf((&self.af, api::CY)),
            api::grr(&self.bc),
            api::grr(&self.de),
            api::grr(&self.hl),
            api::grr(&self.sp),
            api::grr(&self.pc),
        )
    }
}

pub mod api {
    use super::*;

    pub const Z: u16 = 0x80;
    pub const N: u16 = 0x40;
    pub const H: u16 = 0x20;
    pub const CY: u16 = 0x10;

    pub const P1: u16 = 0xff00;
    pub const SB: u16 = 0xff01;
    pub const SC: u16 = 0xff02;
    pub const DIV: u16 = 0xff04;
    pub const TIMA: u16 = 0xff05;
    pub const TMA: u16 = 0xff06;
    pub const TAC: u16 = 0xff07;
    pub const IF: u16 = 0xff0f;
    pub const NR10: u16 = 0xff10;
    pub const NR11: u16 = 0xff11;
    pub const NR12: u16 = 0xff12;
    pub const NR13: u16 = 0xff13;
    pub const NR14: u16 = 0xff14;
    pub const NR21: u16 = 0xff16;
    pub const NR22: u16 = 0xff17;
    pub const NR23: u16 = 0xff18;
    pub const NR24: u16 = 0xff19;
    pub const NR30: u16 = 0xff1a;
    pub const NR31: u16 = 0xff1b;
    pub const NR32: u16 = 0xff1c;
    pub const NR33: u16 = 0xff1d;
    pub const NR34: u16 = 0xff1e;
    pub const NR41: u16 = 0xff20;
    pub const NR42: u16 = 0xff21;
    pub const NR43: u16 = 0xff22;
    pub const NR44: u16 = 0xff23;
    pub const NR50: u16 = 0xff24;
    pub const NR51: u16 = 0xff25;
    pub const NR52: u16 = 0xff26;
    pub const WPRAM: u16 = 0xff30;
    pub const LCDC: u16 = 0xff40;
    pub const STAT: u16 = 0xff41;
    pub const SCY: u16 = 0xff42;
    pub const SCX: u16 = 0xff43;
    pub const LY: u16 = 0xff44;
    pub const LYC: u16 = 0xff45;
    pub const DMA: u16 = 0xff46;
    pub const BGP: u16 = 0xff47;
    pub const OBP0: u16 = 0xff48;
    pub const OBP1: u16 = 0xff49;
    pub const WY: u16 = 0xff4a;
    pub const WX: u16 = 0xff4b;
    pub const IE: u16 = 0xffff;

    pub const U: bool = true;
    pub const D: bool = false;

    pub type RR<'a> = &'a Reg;
    pub type MRR<'a> = &'a mut Reg;
    pub type R<'a> = (&'a Reg, bool);
    pub type MR<'a> = (&'a mut Reg, bool);
    pub type F<'a> = (&'a Reg, u16);
    pub type MF<'a> = (&'a mut Reg, u16);

    pub fn grr(r: RR) -> u16 {
        r.get_16()
    }

    pub fn srr(r: MRR, v: u16) {
        r.set_16(v);
    }

    pub fn gr(r: R) -> u8 {
        r.0.get_8(r.1)
    }

    pub fn sr(r: MR, v: u8) {
        r.0.set_8(r.1, v);
    }

    pub fn gf(r: F) -> bool {
        r.0.get_bit(r.1)
    }

    pub fn sf(r: MF, v: bool) {
        r.0.set_bit(r.1, v);
    }
}

#[cfg(test)]
mod tests {
    use super::api::*;
    use super::*;

    #[test]
    fn priv_get() {
        let mut reg = Reg::new();

        reg.set_16(0xc3a5);
        assert_eq!(reg.get_16(), 0xc3a5);
        assert_eq!(reg.get_8(U), 0xc3);
        assert_eq!(reg.get_8(D), 0xa5);
        assert_eq!(reg.get_bit(Z), true);
        assert_eq!(reg.get_bit(N), false);
        assert_eq!(reg.get_bit(H), true);
        assert_eq!(reg.get_bit(CY), false);
    }

    #[test]
    fn priv_set() {
        let mut reg = Reg::new();

        reg.set_16(0xc3a5);
        assert_eq!(reg.get_16(), 0xc3a5);
        reg.set_8(U, 0xaa);
        assert_eq!(reg.get_16(), 0xaaa5);
        reg.set_8(D, 0x55);
        assert_eq!(reg.get_16(), 0xaa55);
        reg.set_bit(Z, false);
        assert_eq!(reg.get_bit(Z), false);
        reg.set_bit(N, true);
        assert_eq!(reg.get_bit(N), true);
        reg.set_bit(H, false);
        assert_eq!(reg.get_bit(H), false);
        reg.set_bit(CY, true);
        assert_eq!(reg.get_bit(CY), true);
    }

    #[test]
    fn regs() {
        let mut regs = Regs::new();

        assert_eq!(regs.af.get_16(), 0);
        regs.af.set_8(U, 0xff);
        assert_eq!(regs.af.get_16(), 0xff00);
        assert_eq!(regs.af.get_8(U), 0xff);
        assert_eq!(regs.af.get_bit(Z), false);
        regs.af.set_bit(Z, true);
        assert_eq!(regs.af.get_bit(Z), true);
        regs.af.set_bit(Z, false);
        assert_eq!(regs.af.get_bit(Z), false);
        assert_eq!(regs.sp.get_8(D), 0);
        regs.sp.set_8(D, 0xff);
        assert_eq!(regs.sp.get_16(), 0xff);
        assert_eq!(regs.sp.get_8(D), 0xff);
    }

    #[test]
    fn pub_get() {
        let mut reg = Reg::new();

        reg.set_16(0xc3a5);
        assert_eq!(grr(&reg), 0xc3a5);
        assert_eq!(grr(&mut reg), 0xc3a5);
        assert_eq!(gr((&reg, U)), 0xc3);
        assert_eq!(gr((&reg, D)), 0xa5);
        assert_eq!(gf((&reg, Z)), true);
        assert_eq!(gf((&reg, N)), false);
        assert_eq!(gf((&reg, H)), true);
        assert_eq!(gf((&reg, CY)), false);
    }

    #[test]
    fn pub_set() {
        let mut reg = Reg::new();

        srr(&mut reg, 0xc3a5);
        assert_eq!(grr(&reg), 0xc3a5);
        sr((&mut reg, U), 0xaa);
        assert_eq!(grr(&reg), 0xaaa5);
        sr((&mut reg, D), 0x55);
        assert_eq!(grr(&reg), 0xaa55);
        sf((&mut reg, Z), false);
        assert_eq!(gf((&reg, Z)), false);
        sf((&mut reg, N), true);
        assert_eq!(gf((&reg, N)), true);
        sf((&mut reg, H), false);
        assert_eq!(gf((&reg, H)), false);
        sf((&mut reg, CY), true);
        assert_eq!(gf((&reg, CY)), true);
    }
}
