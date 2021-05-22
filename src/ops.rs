use crate::mem::*;
use crate::ops_impl::*;
use crate::reg::API::*;
use crate::reg::*;

pub struct Op {
    label: &'static str,
    len: usize,
    cycles: usize,
    func: fn(&mut Regs, &mut Mem, u16),
}

impl Op {
    fn new(
        label: &'static str,
        len: usize,
        cycles: usize,
        func: fn(&mut Regs, &mut Mem, u16),
    ) -> Op {
        Op {
            label,
            len,
            cycles,
            func,
        }
    }
}

pub struct Ops(Vec<Option<Op>>);

impl Ops {
    pub fn new() -> Ops {
        let mut ops: Vec<Option<Op>> = (0..0x100).map(|_| None).collect();

        ops[0x02] = Some(Op::new("LD (BC), A", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &mut _r.bc, (&_r.af, U));
        }));
        ops[0x12] = Some(Op::new("LD (DE), A", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &mut _r.de, (&_r.af, U));
        }));
        ops[0x22] = Some(Op::new("LD (HL+), A", 1, 8, |_r, _m, _p| {
            ld_arri_r(_m, &mut _r.hl, (&_r.af, U));
        }));
        ops[0x32] = Some(Op::new("LD (HL-), A", 1, 8, |_r, _m, _p| {
            ld_arri_r(_m, &mut _r.hl, (&_r.af, U));
        }));

        ops[0x06] = Some(Op::new("LD B, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.bc, U), _p as u8);
        }));
        ops[0x16] = Some(Op::new("LD D, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.de, U), _p as u8);
        }));
        ops[0x26] = Some(Op::new("LD H, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.hl, U), _p as u8);
        }));
        ops[0x36] = Some(Op::new("LD (HL), #", 2, 12, |_r, _m, _p| {
            ld_arr_n(_m, &_r.hl, _p as u8);
        }));

        ops[0x0a] = Some(Op::new("LD A, (BC)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.af, U), &_r.bc);
        }));
        ops[0x1a] = Some(Op::new("LD A, (DE)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.af, U), &_r.de);
        }));
        ops[0x2a] = Some(Op::new("LD A, (HL+)", 1, 8, |_r, _m, _p| {
            ld_r_arri(_m, (&mut _r.af, U), &mut _r.hl);
        }));
        ops[0x3a] = Some(Op::new("LD A, (HL-)", 1, 8, |_r, _m, _p| {
            ld_r_arrd(_m, (&mut _r.af, U), &mut _r.hl);
        }));

        ops[0x0e] = Some(Op::new("LD C, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.bc, D), _p as u8);
        }));
        ops[0x1e] = Some(Op::new("LD E, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.de, D), _p as u8);
        }));
        ops[0x2e] = Some(Op::new("LD L, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.hl, D), _p as u8);
        }));
        ops[0x3e] = Some(Op::new("LD A, #", 2, 8, |_r, _m, _p| {
            ld_r_n((&mut _r.af, U), _p as u8);
        }));

        ops[0x40] = Some(Op::new("LD B, B", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x41] = Some(Op::new("LD B, C", 1, 4, |_r, _m, _p| {
            ld_to_U(&mut _r.bc);
        }));
        ops[0x42] = Some(Op::new("LD B, D", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, U), (&_r.de, U));
        }));
        ops[0x43] = Some(Op::new("LD B, E", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, U), (&_r.de, D));
        }));
        ops[0x44] = Some(Op::new("LD B, H", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, U), (&_r.hl, U));
        }));
        ops[0x45] = Some(Op::new("LD B, L", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, U), (&_r.hl, D));
        }));
        ops[0x46] = Some(Op::new("LD B, (HL)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.bc, U), &_r.hl);
        }));
        ops[0x47] = Some(Op::new("LD B, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, U), (&_r.af, U));
        }));

        ops[0x48] = Some(Op::new("LD C, B", 1, 4, |_r, _m, _p| {
            ld_to_D(&mut _r.bc);
        }));
        ops[0x49] = Some(Op::new("LD C, C", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x4a] = Some(Op::new("LD C, D", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, D), (&_r.de, U));
        }));
        ops[0x4b] = Some(Op::new("LD C, E", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, D), (&_r.de, D));
        }));
        ops[0x4c] = Some(Op::new("LD C, H", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, D), (&_r.hl, U));
        }));
        ops[0x4d] = Some(Op::new("LD C, L", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, D), (&_r.hl, D));
        }));
        ops[0x4e] = Some(Op::new("LD C, (HL)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.bc, D), &_r.hl);
        }));
        ops[0x4f] = Some(Op::new("LD C, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.bc, D), (&_r.af, U));
        }));

        ops[0x50] = Some(Op::new("LD D, B", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, U), (&_r.bc, U));
        }));
        ops[0x51] = Some(Op::new("LD D, C", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, U), (&_r.bc, D));
        }));
        ops[0x52] = Some(Op::new("LD D, D", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x53] = Some(Op::new("LD D, E", 1, 4, |_r, _m, _p| {
            ld_to_U(&mut _r.de);
        }));
        ops[0x54] = Some(Op::new("LD D, H", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, U), (&_r.hl, U));
        }));
        ops[0x55] = Some(Op::new("LD D, L", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, U), (&_r.hl, D));
        }));
        ops[0x56] = Some(Op::new("LD D, (HL)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.de, U), &_r.hl);
        }));
        ops[0x57] = Some(Op::new("LD D, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, U), (&_r.af, U));
        }));

        ops[0x58] = Some(Op::new("LD E, B", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, D), (&_r.bc, U));
        }));
        ops[0x59] = Some(Op::new("LD E, C", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, D), (&_r.bc, D));
        }));
        ops[0x5a] = Some(Op::new("LD E, D", 1, 4, |_r, _m, _p| {
            ld_to_D(&mut _r.de);
        }));
        ops[0x5b] = Some(Op::new("LD E, E", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x5c] = Some(Op::new("LD E, H", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, D), (&_r.hl, U));
        }));
        ops[0x5d] = Some(Op::new("LD E, L", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, D), (&_r.hl, D));
        }));
        ops[0x5e] = Some(Op::new("LD E, (HL)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.de, D), &_r.hl);
        }));
        ops[0x5f] = Some(Op::new("LD E, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.de, D), (&_r.af, U));
        }));

        ops[0x60] = Some(Op::new("LD H, B", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, U), (&_r.bc, U));
        }));
        ops[0x61] = Some(Op::new("LD H, C", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, U), (&_r.bc, D));
        }));
        ops[0x62] = Some(Op::new("LD H, D", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, U), (&_r.de, U));
        }));
        ops[0x63] = Some(Op::new("LD H, E", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, U), (&_r.de, D));
        }));
        ops[0x64] = Some(Op::new("LD H, H", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x65] = Some(Op::new("LD H, L", 1, 4, |_r, _m, _p| {
            ld_to_U(&mut _r.hl);
        }));
        ops[0x66] = Some(Op::new("LD H, (HL)", 1, 8, |_r, _m, _p| {
            let tmp = grr(&_r.hl);
            ld_r_ann(_m, (&mut _r.hl, U), tmp);
        }));
        ops[0x67] = Some(Op::new("LD H, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, U), (&_r.af, U));
        }));

        ops[0x68] = Some(Op::new("LD L, B", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, D), (&_r.bc, U));
        }));
        ops[0x69] = Some(Op::new("LD L, C", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, D), (&_r.bc, D));
        }));
        ops[0x6a] = Some(Op::new("LD L, D", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, D), (&_r.de, U));
        }));
        ops[0x6b] = Some(Op::new("LD L, E", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, D), (&_r.de, D));
        }));
        ops[0x6c] = Some(Op::new("LD L, H", 1, 4, |_r, _m, _p| {
            ld_to_D(&mut _r.hl);
        }));
        ops[0x6d] = Some(Op::new("LD L, L", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));
        ops[0x6e] = Some(Op::new("LD L, (HL)", 1, 8, |_r, _m, _p| {
            let tmp = grr(&_r.hl);
            ld_r_ann(_m, (&mut _r.hl, D), tmp);
        }));
        ops[0x6f] = Some(Op::new("LD L, A", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.hl, D), (&_r.af, U));
        }));

        ops[0x70] = Some(Op::new("LD (HL), B", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.bc, U));
        }));
        ops[0x71] = Some(Op::new("LD (HL), C", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.bc, D));
        }));
        ops[0x72] = Some(Op::new("LD (HL), D", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.de, U));
        }));
        ops[0x73] = Some(Op::new("LD (HL), E", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.de, D));
        }));
        ops[0x74] = Some(Op::new("LD (HL), H", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.hl, U));
        }));
        ops[0x75] = Some(Op::new("LD (HL), L", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.hl, D));
        }));
        ops[0x77] = Some(Op::new("LD (HL), A", 1, 8, |_r, _m, _p| {
            ld_arr_r(_m, &_r.hl, (&_r.af, U));
        }));

        ops[0x78] = Some(Op::new("LD A, B", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.bc, U));
        }));
        ops[0x79] = Some(Op::new("LD A, C", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.bc, D));
        }));
        ops[0x7a] = Some(Op::new("LD A, D", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.de, U));
        }));
        ops[0x7b] = Some(Op::new("LD A, E", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.de, D));
        }));
        ops[0x7c] = Some(Op::new("LD A, H", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.hl, U));
        }));
        ops[0x7d] = Some(Op::new("LD A, L", 1, 4, |_r, _m, _p| {
            ld_r_r((&mut _r.af, U), (&_r.hl, D));
        }));
        ops[0x7e] = Some(Op::new("LD A, (HL)", 1, 8, |_r, _m, _p| {
            ld_r_arr(_m, (&mut _r.af, U), &_r.hl);
        }));
        ops[0x7f] = Some(Op::new("LD A, A", 1, 4, |_r, _m, _p| {
            ld_pass();
        }));

        Ops(ops)
    }

    pub fn get(&self, idx: usize) -> &Option<Op> {
        if idx > 0xff {
            &None
        } else {
            &self.0[idx]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system() {
        let mut mem = Mem::new();
        let mut regs = Regs::new();

        let ops = vec![
            Op::new("TEST1", 1, 1, |_r, _m, _p| srr(&mut _r.af, 0xff)),
            Op::new("TEST2", 1, 1, |_r, _m, _p| sr((&mut _r.af, U), 0xff)),
            Op::new("TEST3", 1, 1, |_r, _m, _p| sf((&mut _r.af, Z), true)),
            Op::new("TEST4", 1, 1, |_r, _m, _p| sf((&mut _r.af, Z), false)),
            Op::new("TEST5", 1, 1, |_r, _m, _p| {
                sf((&mut _r.af, CY), false);
                _m.set(0xff, 0xf);
            }),
        ];

        (ops[0].func)(&mut regs, &mut mem, 0);
        assert_eq!(grr(&regs.af), 0xff);
        (ops[1].func)(&mut regs, &mut mem, 0);
        assert_eq!(grr(&regs.af), 0xffff);
        (ops[2].func)(&mut regs, &mut mem, 0);
        assert_eq!(gf((&regs.af, Z)), true);
        (ops[3].func)(&mut regs, &mut mem, 0);
        assert_eq!(gf((&regs.af, Z)), false);
        assert_eq!(grr(&regs.af), 0xff7f);
        (ops[4].func)(&mut regs, &mut mem, 0);
        assert_eq!(gf((&regs.af, CY)), false);
        assert_eq!(mem.get(0xff), 0xf);
    }

    #[test]
    fn creation() {
        let ops = Ops::new();

        if let Some(op) = ops.get(2) {
            assert_eq!(op.label, "LD (BC), A");
        } else {
            panic!();
        }
    }
}
