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
        }
    }
}

pub mod api {
    use super::*;

    pub const Z: u16 = 0x80;
    pub const N: u16 = 0x40;
    pub const H: u16 = 0x20;
    pub const CY: u16 = 0x10;

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
