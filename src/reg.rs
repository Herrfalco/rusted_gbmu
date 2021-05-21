pub const Z: u16 = 0x80;
pub const N: u16 = 0x40;
pub const H: u16 = 0x20;
pub const CY: u16 = 0x10;

pub struct Reg {
    val: u16,
}

impl Reg {
    fn new() -> Reg {
        Reg { val: 0 }
    }

    pub fn get(&self) -> u16 {
        self.val
    }

    pub fn set(&mut self, val: u16) {
        self.val = val;
    }

    pub fn get_h(&self) -> u8 {
        ((self.val & 0xff00) >> 8) as u8
    }

    pub fn get_l(&self) -> u8 {
        self.val as u8
    }

    pub fn set_h(&mut self, val: u8) {
        self.val &= 0x00ff;
        self.val |= (val as u16) << 8
    }

    pub fn set_l(&mut self, val: u8) {
        self.val &= 0xff00;
        self.val |= val as u16;
    }

    pub fn get_bit(&self, mask: u16) -> bool {
        self.val & mask != 0
    }

    pub fn set_bit(&mut self, mask: u16, val: bool) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut reg = Reg::new();

        reg.set(0xc3a5);
        assert_eq!(reg.get(), 0xc3a5);
        assert_eq!(reg.get_h(), 0xc3);
        assert_eq!(reg.get_l(), 0xa5);
        assert_eq!(reg.get_bit(Z), true);
        assert_eq!(reg.get_bit(N), false);
        assert_eq!(reg.get_bit(H), true);
        assert_eq!(reg.get_bit(CY), false);
    }

    #[test]
    fn set() {
        let mut reg = Reg::new();

        reg.set(0xc3a5);
        assert_eq!(reg.get(), 0xc3a5);
        reg.set_h(0xaa);
        assert_eq!(reg.get(), 0xaaa5);
        reg.set_l(0x55);
        assert_eq!(reg.get(), 0xaa55);
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

        assert_eq!(regs.af.get(), 0);
        regs.af.set_h(0xff);
        assert_eq!(regs.af.get(), 0xff00);
        assert_eq!(regs.af.get_h(), 0xff);

        assert_eq!(regs.af.get_bit(Z), false);
        regs.af.set_bit(Z, true);
        assert_eq!(regs.af.get_bit(Z), true);
        regs.af.set_bit(Z, false);
        assert_eq!(regs.af.get_bit(Z), false);

        assert_eq!(regs.sp.get_l(), 0);
        regs.sp.set_l(0xff);
        assert_eq!(regs.sp.get(), 0xff);
        assert_eq!(regs.sp.get_l(), 0xff);
    }
}
