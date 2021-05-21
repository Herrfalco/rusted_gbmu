use crate::mem::Mem;
use crate::reg::*;

pub enum Param {
    U8(u8),
    U16(u16),
    No,
}

pub struct Op {
    label: &'static str,
    len: usize,
    cycl: usize,
    func: fn(&mut Regs, &mut Mem, Param),
}

impl Op {
    fn new(
        label: &'static str,
        len: usize,
        cycl: usize,
        func: fn(&mut Regs, &mut Mem, Param),
    ) -> Op {
        Op {
            label,
            len,
            cycl,
            func,
        }
    }
}

pub struct Ops();

impl Ops {
    pub fn new() -> Vec<Op> {
        vec![Op::new("NOP", 1, 1, |_r, _m, _p| {
            _r.af.set(0xff);
        })]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn various() {
        let mut mem = Mem::new();
        let mut regs = Regs::new();

        let ops = vec![
            Op::new("TEST1", 1, 1, |_r, _m, _p| _r.af.set(0xff)),
            Op::new("TEST2", 1, 1, |_r, _m, _p| _r.af.set_h(0xff)),
            Op::new("TEST3", 1, 1, |_r, _m, _p| _r.af.set_bit(Z, true)),
            Op::new("TEST4", 1, 1, |_r, _m, _p| _r.af.set_bit(Z, false)),
            Op::new("TEST5", 1, 1, |_r, _m, _p| {
                _r.af.set_bit(CY, false);
                _m.set(0xff, 0xf);
            }),
        ];

        (ops[0].func)(&mut regs, &mut mem, Param::No);
        assert_eq!(regs.af.get(), 0xff);
        (ops[1].func)(&mut regs, &mut mem, Param::No);
        assert_eq!(regs.af.get(), 0xffff);
        (ops[2].func)(&mut regs, &mut mem, Param::No);
        assert_eq!(regs.af.get_bit(Z), true);
        (ops[3].func)(&mut regs, &mut mem, Param::No);
        assert_eq!(regs.af.get_bit(Z), false);
        assert_eq!(regs.af.get(), 0xff7f);
        (ops[4].func)(&mut regs, &mut mem, Param::No);
        assert_eq!(regs.af.get_bit(CY), false);
        assert_eq!(mem.get(0xff), 0xf);
    }
}
