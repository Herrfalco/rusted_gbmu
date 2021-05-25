use super::imp::*;
use crate::mem::Mem;
use crate::reg::*;
use std::fmt;

pub struct Op {
    label: &'static str,
    len: usize,
    cycles: (usize, usize),
    func: fn(&mut Regs, &mut Mem, u16) -> bool,
}

impl Op {
    fn new(
        label: &'static str,
        len: usize,
        cycles: (usize, usize),
        func: fn(&mut Regs, &mut Mem, u16) -> bool,
    ) -> Op {
        Op {
            label,
            len,
            cycles,
            func,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn exec(&self, r: &mut Regs, m: &mut Mem, p: u16) -> bool {
        (self.func)(r, m, p)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub struct Ops(Vec<Option<Op>>);

impl Ops {
    pub fn get(&self, idx: usize) -> Option<&Op> {
        match &self.0[idx] {
            Some(op) => Some(&op),
            None => None,
        }
    }

    pub fn new() -> Ops {
        let mut ops: Vec<Option<Op>> = (0..0x200).map(|_| None).collect();

        ///////////////////////// 8 BITS LOADS ///////////////////////////
        ops[0x02] = Some(Op::new("LD (BC), A", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &mut _r.bc, gr((&_r.af, U)))
        }));
        ops[0x12] = Some(Op::new("LD (DE), A", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &mut _r.de, gr((&_r.af, U)))
        }));
        ops[0x22] = Some(Op::new("LD (HL+), A", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arri_r(_m, &mut _r.hl, (&_r.af, U))
        }));
        ops[0x32] = Some(Op::new("LD (HL-), A", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arrd_r(_m, &mut _r.hl, (&_r.af, U))
        }));

        ops[0x06] = Some(Op::new("LD B, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), _p as u8)
        }));
        ops[0x16] = Some(Op::new("LD D, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), _p as u8)
        }));
        ops[0x26] = Some(Op::new("LD H, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), _p as u8)
        }));
        ops[0x36] = Some(Op::new("LD (HL), #", 2, (12, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, _p as u8)
        }));

        ops[0x0a] = Some(Op::new("LD A, (BC)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.af, U), grr(&_r.bc))
        }));
        ops[0x1a] = Some(Op::new("LD A, (DE)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.af, U), grr(&_r.de))
        }));
        ops[0x2a] = Some(Op::new("LD A, (HL+)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_arri(_m, (&mut _r.af, U), &mut _r.hl)
        }));
        ops[0x3a] = Some(Op::new("LD A, (HL-)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_arrd(_m, (&mut _r.af, U), &mut _r.hl)
        }));

        ops[0x0e] = Some(Op::new("LD C, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), _p as u8)
        }));
        ops[0x1e] = Some(Op::new("LD E, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), _p as u8)
        }));
        ops[0x2e] = Some(Op::new("LD L, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), _p as u8)
        }));
        ops[0x3e] = Some(Op::new("LD A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), _p as u8)
        }));

        ops[0x40] = Some(Op::new("LD B, B", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x41] = Some(Op::new("LD B, C", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.bc, D));
            ld_r_n((&mut _r.bc, U), tmp)
        }));
        ops[0x42] = Some(Op::new("LD B, D", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), gr((&_r.de, U)))
        }));
        ops[0x43] = Some(Op::new("LD B, E", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), gr((&_r.de, D)))
        }));
        ops[0x44] = Some(Op::new("LD B, H", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), gr((&_r.hl, U)))
        }));
        ops[0x45] = Some(Op::new("LD B, L", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), gr((&_r.hl, D)))
        }));
        ops[0x46] = Some(Op::new("LD B, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.bc, U), grr(&_r.hl))
        }));
        ops[0x47] = Some(Op::new("LD B, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, U), gr((&_r.af, U)))
        }));

        ops[0x48] = Some(Op::new("LD C, B", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.bc, U));
            ld_r_n((&mut _r.bc, D), tmp)
        }));
        ops[0x49] = Some(Op::new("LD C, C", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x4a] = Some(Op::new("LD C, D", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), gr((&_r.de, U)))
        }));
        ops[0x4b] = Some(Op::new("LD C, E", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), gr((&_r.de, D)))
        }));
        ops[0x4c] = Some(Op::new("LD C, H", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), gr((&_r.hl, U)))
        }));
        ops[0x4d] = Some(Op::new("LD C, L", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), gr((&_r.hl, D)))
        }));
        ops[0x4e] = Some(Op::new("LD C, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.bc, D), grr(&_r.hl))
        }));
        ops[0x4f] = Some(Op::new("LD C, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.bc, D), gr((&_r.af, U)))
        }));

        ops[0x50] = Some(Op::new("LD D, B", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), gr((&_r.bc, U)))
        }));
        ops[0x51] = Some(Op::new("LD D, C", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), gr((&_r.bc, D)))
        }));
        ops[0x52] = Some(Op::new("LD D, D", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x53] = Some(Op::new("LD D, E", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.de, D));
            ld_r_n((&mut _r.de, U), tmp)
        }));
        ops[0x54] = Some(Op::new("LD D, H", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), gr((&_r.hl, U)))
        }));
        ops[0x55] = Some(Op::new("LD D, L", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), gr((&_r.hl, D)))
        }));
        ops[0x56] = Some(Op::new("LD D, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.de, U), grr(&_r.hl))
        }));
        ops[0x57] = Some(Op::new("LD D, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, U), gr((&_r.af, U)))
        }));

        ops[0x58] = Some(Op::new("LD E, B", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), gr((&_r.bc, U)))
        }));
        ops[0x59] = Some(Op::new("LD E, C", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), gr((&_r.bc, D)))
        }));
        ops[0x5a] = Some(Op::new("LD E, D", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.de, U));
            ld_r_n((&mut _r.de, D), tmp)
        }));
        ops[0x5b] = Some(Op::new("LD E, E", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x5c] = Some(Op::new("LD E, H", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), gr((&_r.hl, U)))
        }));
        ops[0x5d] = Some(Op::new("LD E, L", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), gr((&_r.hl, D)))
        }));
        ops[0x5e] = Some(Op::new("LD E, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.de, D), grr(&_r.hl))
        }));
        ops[0x5f] = Some(Op::new("LD E, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.de, D), gr((&_r.af, U)))
        }));

        ops[0x60] = Some(Op::new("LD H, B", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), gr((&_r.bc, U)))
        }));
        ops[0x61] = Some(Op::new("LD H, C", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), gr((&_r.bc, D)))
        }));
        ops[0x62] = Some(Op::new("LD H, D", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), gr((&_r.de, U)))
        }));
        ops[0x63] = Some(Op::new("LD H, E", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), gr((&_r.de, D)))
        }));
        ops[0x64] = Some(Op::new("LD H, H", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x65] = Some(Op::new("LD H, L", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.hl, D));
            ld_r_n((&mut _r.hl, U), tmp)
        }));
        ops[0x66] = Some(Op::new("LD H, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            let tmp = grr(&_r.hl);
            ld_r_ann(_m, (&mut _r.hl, U), tmp)
        }));
        ops[0x67] = Some(Op::new("LD H, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, U), gr((&_r.af, U)))
        }));

        ops[0x68] = Some(Op::new("LD L, B", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), gr((&_r.bc, U)))
        }));
        ops[0x69] = Some(Op::new("LD L, C", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), gr((&_r.bc, D)))
        }));
        ops[0x6a] = Some(Op::new("LD L, D", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), gr((&_r.de, U)))
        }));
        ops[0x6b] = Some(Op::new("LD L, E", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), gr((&_r.de, D)))
        }));
        ops[0x6c] = Some(Op::new("LD L, H", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.hl, U));
            ld_r_n((&mut _r.hl, D), tmp)
        }));
        ops[0x6d] = Some(Op::new("LD L, L", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x6e] = Some(Op::new("LD L, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            let tmp = grr(&_r.hl);
            ld_r_ann(_m, (&mut _r.hl, D), tmp)
        }));
        ops[0x6f] = Some(Op::new("LD L, A", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.hl, D), gr((&_r.af, U)))
        }));

        ops[0x70] = Some(Op::new("LD (HL), B", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.bc, U)))
        }));
        ops[0x71] = Some(Op::new("LD (HL), C", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.bc, D)))
        }));
        ops[0x72] = Some(Op::new("LD (HL), D", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.de, U)))
        }));
        ops[0x73] = Some(Op::new("LD (HL), E", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.de, D)))
        }));
        ops[0x74] = Some(Op::new("LD (HL), H", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.hl, U)))
        }));
        ops[0x75] = Some(Op::new("LD (HL), L", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.hl, D)))
        }));
        ops[0x77] = Some(Op::new("LD (HL), A", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_arr_n(_m, &_r.hl, gr((&_r.af, U)))
        }));

        ops[0x78] = Some(Op::new("LD A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.bc, U)))
        }));
        ops[0x79] = Some(Op::new("LD A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.bc, D)))
        }));
        ops[0x7a] = Some(Op::new("LD A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.de, U)))
        }));
        ops[0x7b] = Some(Op::new("LD A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.de, D)))
        }));
        ops[0x7c] = Some(Op::new("LD A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.hl, U)))
        }));
        ops[0x7d] = Some(Op::new("LD A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            ld_r_n((&mut _r.af, U), gr((&_r.hl, D)))
        }));
        ops[0x7e] = Some(Op::new("LD A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.af, U), grr(&_r.hl))
        }));
        ops[0x7f] = Some(Op::new("LD A, A", 1, (4, 0), |_r, _m, _p| -> bool { true }));

        ops[0xe0] = Some(Op::new("LDH (#), A", 2, (12, 0), |_r, _m, _p| -> bool {
            ldh_an_r(_m, _p as u8, (&_r.af, U))
        }));
        ops[0xf0] = Some(Op::new("LDH A, (#)", 2, (12, 0), |_r, _m, _p| -> bool {
            ldh_r_an(_m, (&mut _r.af, U), _p as u8)
        }));

        ops[0xe2] = Some(Op::new("LDH (C), A", 2, (8, 0), |_r, _m, _p| -> bool {
            ldh_ar_r(_m, (&_r.bc, D), (&_r.af, U))
        }));
        ops[0xf2] = Some(Op::new("LDH A, (C)", 2, (8, 0), |_r, _m, _p| -> bool {
            ldh_r_an(_m, (&mut _r.af, U), gr((&_r.bc, D)))
        }));

        ops[0xea] = Some(Op::new("LD (#), A", 3, (16, 0), |_r, _m, _p| -> bool {
            ld_ann_r(_m, _p, (&_r.af, U))
        }));
        ops[0xfa] = Some(Op::new("LD A, (#)", 3, (16, 0), |_r, _m, _p| -> bool {
            ld_r_ann(_m, (&mut _r.af, U), _p)
        }));

        //////////////////////// 16 BITS LOADS ///////////////////////////

        ops[0x01] = Some(Op::new("LD BC, #", 3, (12, 0), |_r, _m, _p| -> bool {
            ld_rr_nn(&mut _r.bc, _p)
        }));
        ops[0x11] = Some(Op::new("LD DE, #", 3, (12, 0), |_r, _m, _p| -> bool {
            ld_rr_nn(&mut _r.de, _p)
        }));
        ops[0x21] = Some(Op::new("LD HL, #", 3, (12, 0), |_r, _m, _p| -> bool {
            ld_rr_nn(&mut _r.hl, _p)
        }));
        ops[0x31] = Some(Op::new("LD SP, #", 3, (12, 0), |_r, _m, _p| -> bool {
            ld_rr_nn(&mut _r.sp, _p)
        }));

        ops[0x08] = Some(Op::new("LD (#), SP", 3, (20, 0), |_r, _m, _p| -> bool {
            ld_ann_rr(_m, _p, &_r.sp)
        }));
        ops[0xf8] = Some(Op::new("LD HL, SP+#", 2, (12, 0), |_r, _m, _p| -> bool {
            ld_rr_rrpsn(&mut _r.af, &mut _r.hl, &_r.sp, _p as i8)
        }));
        ops[0xf9] = Some(Op::new("LD SP, HL", 1, (8, 0), |_r, _m, _p| -> bool {
            ld_rr_nn(&mut _r.sp, grr(&_r.hl))
        }));

        ops[0xc1] = Some(Op::new("POP BC", 1, (12, 0), |_r, _m, _p| -> bool {
            pop_rr_arr(_m, &mut _r.bc, &mut _r.sp)
        }));
        ops[0xd1] = Some(Op::new("POP BC", 1, (12, 0), |_r, _m, _p| -> bool {
            pop_rr_arr(_m, &mut _r.de, &mut _r.sp)
        }));
        ops[0xe1] = Some(Op::new("POP BC", 1, (12, 0), |_r, _m, _p| -> bool {
            pop_rr_arr(_m, &mut _r.hl, &mut _r.sp)
        }));
        ops[0xf1] = Some(Op::new("POP BC", 1, (12, 0), |_r, _m, _p| -> bool {
            pop_rr_arr(_m, &mut _r.af, &mut _r.sp)
        }));

        ops[0xc5] = Some(Op::new("PUSH BC", 1, (16, 0), |_r, _m, _p| -> bool {
            push_arr_rr(_m, &mut _r.sp, &mut _r.bc)
        }));
        ops[0xd5] = Some(Op::new("PUSH DE", 1, (16, 0), |_r, _m, _p| -> bool {
            push_arr_rr(_m, &mut _r.sp, &mut _r.de)
        }));
        ops[0xe5] = Some(Op::new("PUSH HL", 1, (16, 0), |_r, _m, _p| -> bool {
            push_arr_rr(_m, &mut _r.sp, &mut _r.hl)
        }));
        ops[0xf5] = Some(Op::new("PUSH AF", 1, (16, 0), |_r, _m, _p| -> bool {
            push_arr_rr(_m, &mut _r.sp, &mut _r.af)
        }));

        ////////////////////// 8 BITS ARITHMETIC ///////////////////////

        ops[0x04] = Some(Op::new("INC B", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x14] = Some(Op::new("INC D", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x24] = Some(Op::new("INC H", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x34] = Some(Op::new("INC (HL)", 1, (12, 0), |_r, _m, _p| -> bool {
            inc_arr(&mut _r.af, _m, &_r.hl)
        }));

        ops[0x05] = Some(Op::new("DEC B", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x15] = Some(Op::new("DEC D", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x25] = Some(Op::new("DEC H", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x35] = Some(Op::new("DEC (HL)", 1, (12, 0), |_r, _m, _p| -> bool {
            dec_arr(&mut _r.af, _m, &_r.hl)
        }));

        ops[0x0c] = Some(Op::new("INC C", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x1c] = Some(Op::new("INC E", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x2c] = Some(Op::new("INC L", 1, (4, 0), |_r, _m, _p| -> bool {
            inc_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x3c] = Some(Op::new("INC A", 1, (4, 0), |_r, _m, _p| -> bool {
            inc(&mut _r.af)
        }));

        ops[0x0d] = Some(Op::new("DEC C", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x1d] = Some(Op::new("DEC E", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x2d] = Some(Op::new("DEC L", 1, (4, 0), |_r, _m, _p| -> bool {
            dec_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x3d] = Some(Op::new("DEC A", 1, (4, 0), |_r, _m, _p| -> bool {
            dec(&mut _r.af)
        }));

        ops[0x27] = Some(Op::new("DAA", 1, (4, 0), |_r, _m, _p| -> bool {
            daa(&mut _r.af)
        }));
        ops[0x37] = Some(Op::new("SCF", 1, (4, 0), |_r, _m, _p| -> bool {
            scf(&mut _r.af)
        }));
        ops[0x2f] = Some(Op::new("CPL", 1, (4, 0), |_r, _m, _p| -> bool {
            cpl(&mut _r.af)
        }));
        ops[0x3f] = Some(Op::new("CCF", 1, (4, 0), |_r, _m, _p| -> bool {
            ccf(&mut _r.af)
        }));

        ops[0xc6] = Some(Op::new("ADD A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, _p as u8)
        }));
        ops[0xd6] = Some(Op::new("SUB A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, _p as u8)
        }));
        ops[0xe6] = Some(Op::new("AND A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, _p as u8)
        }));
        ops[0xf6] = Some(Op::new("OR A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, _p as u8)
        }));

        ops[0xce] = Some(Op::new("ADC A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, _p as u8)
        }));
        ops[0xde] = Some(Op::new("SBC A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, _p as u8)
        }));
        ops[0xee] = Some(Op::new("XOR A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, _p as u8)
        }));
        ops[0xfe] = Some(Op::new("CP A, #", 2, (8, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, _p as u8)
        }));

        ops[0x80] = Some(Op::new("ADD A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0x81] = Some(Op::new("ADD A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0x82] = Some(Op::new("ADD A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0x83] = Some(Op::new("ADD A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0x84] = Some(Op::new("ADD A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0x85] = Some(Op::new("ADD A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0x86] = Some(Op::new("ADD A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            add_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0x87] = Some(Op::new("ADD A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            add_n(&mut _r.af, tmp)
        }));

        ops[0x88] = Some(Op::new("ADC A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0x89] = Some(Op::new("ADC A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0x8a] = Some(Op::new("ADC A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0x8b] = Some(Op::new("ADC A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0x8c] = Some(Op::new("ADC A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0x8d] = Some(Op::new("ADC A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0x8e] = Some(Op::new("ADC A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            adc_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0x8f] = Some(Op::new("ADC A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            adc_n(&mut _r.af, tmp)
        }));

        ops[0x90] = Some(Op::new("SUB A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0x91] = Some(Op::new("SUB A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0x92] = Some(Op::new("SUB A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0x93] = Some(Op::new("SUB A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0x94] = Some(Op::new("SUB A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0x95] = Some(Op::new("SUB A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0x96] = Some(Op::new("SUB A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            sub_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0x97] = Some(Op::new("SUB A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            sub_n(&mut _r.af, tmp)
        }));

        ops[0x98] = Some(Op::new("SBC A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0x99] = Some(Op::new("SBC A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0x9a] = Some(Op::new("SBC A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0x9b] = Some(Op::new("SBC A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0x9c] = Some(Op::new("SBC A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0x9d] = Some(Op::new("SBC A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0x9e] = Some(Op::new("SBC A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            sbc_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0x9f] = Some(Op::new("SBC A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            sbc_n(&mut _r.af, tmp)
        }));

        ops[0xa0] = Some(Op::new("AND A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0xa1] = Some(Op::new("AND A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0xa2] = Some(Op::new("AND A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0xa3] = Some(Op::new("AND A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0xa4] = Some(Op::new("AND A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0xa5] = Some(Op::new("AND A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0xa6] = Some(Op::new("AND A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            and_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0xa7] = Some(Op::new("AND A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            and_n(&mut _r.af, tmp)
        }));

        ops[0xa8] = Some(Op::new("XOR A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0xa9] = Some(Op::new("XOR A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0xaa] = Some(Op::new("XOR A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0xab] = Some(Op::new("XOR A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0xac] = Some(Op::new("XOR A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0xad] = Some(Op::new("XOR A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0xae] = Some(Op::new("XOR A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            xor_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0xaf] = Some(Op::new("XOR A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            xor_n(&mut _r.af, tmp)
        }));

        ops[0xb0] = Some(Op::new("OR A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0xb1] = Some(Op::new("OR A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0xb2] = Some(Op::new("OR A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0xb3] = Some(Op::new("OR A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0xb4] = Some(Op::new("OR A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0xb5] = Some(Op::new("OR A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0xb6] = Some(Op::new("OR A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            or_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0xb7] = Some(Op::new("OR A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            or_n(&mut _r.af, tmp)
        }));

        ops[0xb8] = Some(Op::new("CP A, B", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.bc, U)))
        }));
        ops[0xb9] = Some(Op::new("CP A, C", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.bc, D)))
        }));
        ops[0xba] = Some(Op::new("CP A, D", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.de, U)))
        }));
        ops[0xbb] = Some(Op::new("CP A, E", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.de, D)))
        }));
        ops[0xbc] = Some(Op::new("CP A, H", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.hl, U)))
        }));
        ops[0xbd] = Some(Op::new("CP A, L", 1, (4, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, gr((&_r.hl, D)))
        }));
        ops[0xbe] = Some(Op::new("CP A, (HL)", 1, (8, 0), |_r, _m, _p| -> bool {
            cp_n(&mut _r.af, _m.get(grr(&_r.hl)))
        }));
        ops[0xbf] = Some(Op::new("CP A, A", 1, (4, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            cp_n(&mut _r.af, tmp)
        }));

        ///////////////////////// JUMP/CALLS //////////////////////////

        ops[0x20] = Some(Op::new("JR NZ, #", 2, (12, 8), |_r, _m, _p| -> bool {
            jr_cc_sn(&mut _r.pc, !gf((&_r.af, Z)), _p as i8)
        }));
        ops[0x30] = Some(Op::new("JR NC, #", 2, (12, 8), |_r, _m, _p| -> bool {
            jr_cc_sn(&mut _r.pc, !gf((&_r.af, CY)), _p as i8)
        }));
        ops[0x18] = Some(Op::new("JR #", 2, (12, 0), |_r, _m, _p| -> bool {
            jr_cc_sn(&mut _r.pc, true, _p as i8)
        }));
        ops[0x28] = Some(Op::new("JR Z, #", 2, (12, 8), |_r, _m, _p| -> bool {
            jr_cc_sn(&mut _r.pc, gf((&_r.af, Z)), _p as i8)
        }));
        ops[0x38] = Some(Op::new("JR C, #", 2, (12, 8), |_r, _m, _p| -> bool {
            jr_cc_sn(&mut _r.pc, gf((&_r.af, CY)), _p as i8)
        }));

        ops[0xc2] = Some(Op::new("JP NZ, #", 3, (16, 12), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, !gf((&_r.af, Z)), _p)
        }));
        ops[0xd2] = Some(Op::new("JP NC, #", 3, (16, 12), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, !gf((&_r.af, CY)), _p)
        }));
        ops[0xc3] = Some(Op::new("JP #", 3, (12, 0), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, true, _p)
        }));
        ops[0xca] = Some(Op::new("JP Z, #", 3, (16, 12), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, gf((&_r.af, Z)), _p)
        }));
        ops[0xda] = Some(Op::new("JP C, #", 3, (16, 12), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, gf((&_r.af, CY)), _p)
        }));
        ops[0xe9] = Some(Op::new("JP HL", 1, (4, 0), |_r, _m, _p| -> bool {
            jp_cc_nn(&mut _r.pc, true, grr(&mut _r.hl))
        }));

        ops[0xc0] = Some(Op::new("RET NZ", 1, (20, 8), |_r, _m, _p| -> bool {
            ret_cc(_m, &mut _r.pc, &mut _r.sp, !gf((&_r.af, Z)))
        }));
        ops[0xd0] = Some(Op::new("RET NC", 1, (20, 8), |_r, _m, _p| -> bool {
            ret_cc(_m, &mut _r.pc, &mut _r.sp, !gf((&_r.af, CY)))
        }));
        ops[0xc8] = Some(Op::new("RET Z", 1, (20, 8), |_r, _m, _p| -> bool {
            ret_cc(_m, &mut _r.pc, &mut _r.sp, gf((&_r.af, Z)))
        }));
        ops[0xd8] = Some(Op::new("RET C", 1, (20, 8), |_r, _m, _p| -> bool {
            ret_cc(_m, &mut _r.pc, &mut _r.sp, gf((&_r.af, CY)))
        }));
        ops[0xc9] = Some(Op::new("RET", 1, (16, 0), |_r, _m, _p| -> bool {
            ret_cc(_m, &mut _r.pc, &mut _r.sp, true)
        }));
        ops[0xd9] = Some(Op::new("RETI", 1, (16, 0), |_r, _m, _p| -> bool {
            reti(_m, &mut _r.ime, &mut _r.pc, &mut _r.sp)
        }));

        ops[0xc4] = Some(Op::new("CALL NZ, #", 3, (24, 12), |_r, _m, _p| -> bool {
            call_cc_nn(_m, &mut _r.sp, &mut _r.pc, !gf((&_r.af, Z)), _p)
        }));
        ops[0xd4] = Some(Op::new("CALL NC, #", 3, (24, 12), |_r, _m, _p| -> bool {
            call_cc_nn(_m, &mut _r.sp, &mut _r.pc, !gf((&_r.af, CY)), _p)
        }));
        ops[0xcc] = Some(Op::new("CALL Z, #", 3, (24, 12), |_r, _m, _p| -> bool {
            call_cc_nn(_m, &mut _r.sp, &mut _r.pc, gf((&_r.af, Z)), _p)
        }));
        ops[0xdc] = Some(Op::new("CALL C, #", 3, (24, 12), |_r, _m, _p| -> bool {
            call_cc_nn(_m, &mut _r.sp, &mut _r.pc, gf((&_r.af, CY)), _p)
        }));
        ops[0xcd] = Some(Op::new("CALL #", 3, (24, 0), |_r, _m, _p| -> bool {
            call_cc_nn(_m, &mut _r.sp, &mut _r.pc, true, _p)
        }));

        ops[0xc7] = Some(Op::new("RST 0x00", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x00)
        }));
        ops[0xd7] = Some(Op::new("RST 0x10", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x10)
        }));
        ops[0xe7] = Some(Op::new("RST 0x20", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x20)
        }));
        ops[0xf7] = Some(Op::new("RST 0x30", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x30)
        }));

        ops[0xcf] = Some(Op::new("RST 0x08", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x08)
        }));
        ops[0xdf] = Some(Op::new("RST 0x18", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x18)
        }));
        ops[0xef] = Some(Op::new("RST 0x28", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x28)
        }));
        ops[0xff] = Some(Op::new("RST 0x38", 1, (16, 0), |_r, _m, _p| -> bool {
            rst(_m, &mut _r.sp, &mut _r.pc, 0x38)
        }));

        ////////////////////// 16 BITS ARITHMETIC ///////////////////////

        ops[0x03] = Some(Op::new("INC BC", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.bc)
        }));
        ops[0x13] = Some(Op::new("INC DE", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.de)
        }));
        ops[0x23] = Some(Op::new("INC HL", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.hl)
        }));
        ops[0x33] = Some(Op::new("INC SP", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.sp)
        }));

        ops[0x09] = Some(Op::new("ADD HL, BC", 1, (8, 0), |_r, _m, _p| -> bool {
            add_rr_nn(&mut _r.af, &mut _r.hl, grr(&_r.bc))
        }));
        ops[0x19] = Some(Op::new("ADD HL, DE", 1, (8, 0), |_r, _m, _p| -> bool {
            add_rr_nn(&mut _r.af, &mut _r.hl, grr(&_r.de))
        }));
        ops[0x29] = Some(Op::new("ADD HL, HL", 1, (8, 0), |_r, _m, _p| -> bool {
            let tmp = grr(&_r.hl);
            add_rr_nn(&mut _r.af, &mut _r.hl, tmp)
        }));
        ops[0x39] = Some(Op::new("ADD HL, SP", 1, (8, 0), |_r, _m, _p| -> bool {
            add_rr_nn(&mut _r.af, &mut _r.hl, grr(&_r.sp))
        }));
        ops[0xe8] = Some(Op::new("ADD SP, #", 2, (16, 0), |_r, _m, _p| -> bool {
            add_rr_sn(&mut _r.af, &mut _r.sp, _p as i8)
        }));

        ops[0x0b] = Some(Op::new("DEC BC", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.bc)
        }));
        ops[0x1b] = Some(Op::new("DEC DE", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.de)
        }));
        ops[0x2b] = Some(Op::new("DEC HL", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.hl)
        }));
        ops[0x3b] = Some(Op::new("DEC SP", 1, (8, 0), |_r, _m, _p| -> bool {
            inc_rr(&mut _r.sp)
        }));

        ////////////////////// ROTATE/SHIFT ///////////////////////

        ops[0x07] = Some(Op::new("RLCA", 1, (4, 0), |_r, _m, _p| -> bool {
            rlca(&mut _r.af, true)
        }));
        ops[0x17] = Some(Op::new("RLA", 1, (4, 0), |_r, _m, _p| -> bool {
            rla(&mut _r.af, true)
        }));
        ops[0x0f] = Some(Op::new("RRCA", 1, (4, 0), |_r, _m, _p| -> bool {
            rrca(&mut _r.af, true)
        }));
        ops[0x1f] = Some(Op::new("RRA", 1, (4, 0), |_r, _m, _p| -> bool {
            rra(&mut _r.af, true)
        }));

        ////////////////////// MISC/CONTROL ///////////////////////

        ops[0x00] = Some(Op::new("NOP", 1, (4, 0), |_r, _m, _p| -> bool { true }));
        ops[0x10] = Some(Op::new("STOP", 2, (4, 0), |_r, _m, _p| -> bool { stop() }));
        ops[0x76] = Some(Op::new("HALT", 1, (4, 0), |_r, _m, _p| -> bool { halt() }));
        ops[0xf3] = Some(Op::new("DI", 1, (4, 0), |_r, _m, _p| -> bool {
            di(&mut _r.ime)
        }));
        ops[0xfb] = Some(Op::new("EI", 1, (4, 0), |_r, _m, _p| -> bool {
            ei(&mut _r.ime)
        }));

        ///////////////////////// CB OPS //////////////////////////

        ops[0x100] = Some(Op::new("RLC B", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x101] = Some(Op::new("RLC C", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x102] = Some(Op::new("RLC D", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x103] = Some(Op::new("RLC E", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x104] = Some(Op::new("RLC H", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x105] = Some(Op::new("RLC L", 2, (8, 0), |_r, _m, _p| -> bool {
            rlc_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x106] = Some(Op::new("RLC (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            rlc_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x107] = Some(Op::new("RLC A", 2, (8, 0), |_r, _m, _p| -> bool {
            rlca(&mut _r.af, false)
        }));

        ops[0x108] = Some(Op::new("RRC B", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x109] = Some(Op::new("RRC C", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x10a] = Some(Op::new("RRC D", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x10b] = Some(Op::new("RRC E", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x10c] = Some(Op::new("RRC H", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x10d] = Some(Op::new("RRC L", 2, (8, 0), |_r, _m, _p| -> bool {
            rrc_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x10e] = Some(Op::new("RRC (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            rrc_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x10f] = Some(Op::new("RRC A", 2, (8, 0), |_r, _m, _p| -> bool {
            rrca(&mut _r.af, false)
        }));

        ops[0x110] = Some(Op::new("RL B", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x111] = Some(Op::new("RL C", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x112] = Some(Op::new("RL D", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x113] = Some(Op::new("RL E", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x114] = Some(Op::new("RL H", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x115] = Some(Op::new("RL L", 2, (8, 0), |_r, _m, _p| -> bool {
            rl_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x116] = Some(Op::new("RL (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            rl_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x117] = Some(Op::new("RL A", 2, (8, 0), |_r, _m, _p| -> bool {
            rla(&mut _r.af, false)
        }));

        ops[0x118] = Some(Op::new("RR B", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x119] = Some(Op::new("RR C", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x11a] = Some(Op::new("RR D", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x11b] = Some(Op::new("RR E", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x11c] = Some(Op::new("RR H", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x11d] = Some(Op::new("RR L", 2, (8, 0), |_r, _m, _p| -> bool {
            rr_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x11e] = Some(Op::new("RR (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            rr_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x11f] = Some(Op::new("RR A", 2, (8, 0), |_r, _m, _p| -> bool {
            rra(&mut _r.af, false)
        }));

        ops[0x120] = Some(Op::new("SLA B", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x121] = Some(Op::new("SLA C", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x122] = Some(Op::new("SLA D", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x123] = Some(Op::new("SLA E", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x124] = Some(Op::new("SLA H", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x125] = Some(Op::new("SLA L", 2, (8, 0), |_r, _m, _p| -> bool {
            sla_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x126] = Some(Op::new("SLA (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            sla_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x127] = Some(Op::new("SLA A", 2, (8, 0), |_r, _m, _p| -> bool {
            sla(&mut _r.af)
        }));

        ops[0x128] = Some(Op::new("SRA B", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x129] = Some(Op::new("SRA C", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x12a] = Some(Op::new("SRA D", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x12b] = Some(Op::new("SRA E", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x12c] = Some(Op::new("SRA H", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x12d] = Some(Op::new("SRA L", 2, (8, 0), |_r, _m, _p| -> bool {
            sra_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x12e] = Some(Op::new("SRA (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            sra_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x12f] = Some(Op::new("SRA A", 2, (8, 0), |_r, _m, _p| -> bool {
            sra(&mut _r.af)
        }));

        ops[0x130] = Some(Op::new("SWAP B", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x131] = Some(Op::new("SWAP C", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x132] = Some(Op::new("SWAP D", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x133] = Some(Op::new("SWAP E", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x134] = Some(Op::new("SWAP H", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x135] = Some(Op::new("SWAP L", 2, (8, 0), |_r, _m, _p| -> bool {
            swap_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x136] = Some(Op::new("SWAP (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            swap_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x137] = Some(Op::new("SWAP A", 2, (8, 0), |_r, _m, _p| -> bool {
            swap(&mut _r.af)
        }));

        ops[0x138] = Some(Op::new("SRL B", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.bc, U))
        }));
        ops[0x139] = Some(Op::new("SRL C", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.bc, D))
        }));
        ops[0x13a] = Some(Op::new("SRL D", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.de, U))
        }));
        ops[0x13b] = Some(Op::new("SRL E", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.de, D))
        }));
        ops[0x13c] = Some(Op::new("SRL H", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.hl, U))
        }));
        ops[0x13d] = Some(Op::new("SRL L", 2, (8, 0), |_r, _m, _p| -> bool {
            srl_r(&mut _r.af, (&mut _r.hl, D))
        }));
        ops[0x13e] = Some(Op::new("SRL (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            srl_arr(&mut _r.af, _m, &mut _r.hl)
        }));
        ops[0x13f] = Some(Op::new("SRL A", 2, (8, 0), |_r, _m, _p| -> bool {
            srl(&mut _r.af)
        }));

        ops[0x140] = Some(Op::new("BIT 0, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.bc, U)))
        }));
        ops[0x141] = Some(Op::new("BIT 0, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.bc, D)))
        }));
        ops[0x142] = Some(Op::new("BIT 0, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.de, U)))
        }));
        ops[0x143] = Some(Op::new("BIT 0, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.de, D)))
        }));
        ops[0x144] = Some(Op::new("BIT 0, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.hl, U)))
        }));
        ops[0x145] = Some(Op::new("BIT 0, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, gr((&_r.hl, D)))
        }));
        ops[0x146] = Some(Op::new("BIT 0, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x1, _m.get(grr(&_r.hl)))
        }));
        ops[0x147] = Some(Op::new("BIT 0, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x1, tmp)
        }));

        ops[0x148] = Some(Op::new("BIT 1, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.bc, U)))
        }));
        ops[0x149] = Some(Op::new("BIT 1, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.bc, D)))
        }));
        ops[0x14a] = Some(Op::new("BIT 1, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.de, U)))
        }));
        ops[0x14b] = Some(Op::new("BIT 1, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.de, D)))
        }));
        ops[0x14c] = Some(Op::new("BIT 1, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.hl, U)))
        }));
        ops[0x14d] = Some(Op::new("BIT 1, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, gr((&_r.hl, D)))
        }));
        ops[0x14e] = Some(Op::new("BIT 1, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x2, _m.get(grr(&_r.hl)))
        }));
        ops[0x14f] = Some(Op::new("BIT 1, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x2, tmp)
        }));

        ops[0x150] = Some(Op::new("BIT 2, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.bc, U)))
        }));
        ops[0x151] = Some(Op::new("BIT 2, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.bc, D)))
        }));
        ops[0x152] = Some(Op::new("BIT 2, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.de, U)))
        }));
        ops[0x153] = Some(Op::new("BIT 2, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.de, D)))
        }));
        ops[0x154] = Some(Op::new("BIT 2, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.hl, U)))
        }));
        ops[0x155] = Some(Op::new("BIT 2, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, gr((&_r.hl, D)))
        }));
        ops[0x156] = Some(Op::new("BIT 2, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x4, _m.get(grr(&_r.hl)))
        }));
        ops[0x157] = Some(Op::new("BIT 2, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x4, tmp)
        }));

        ops[0x158] = Some(Op::new("BIT 3, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.bc, U)))
        }));
        ops[0x159] = Some(Op::new("BIT 3, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.bc, D)))
        }));
        ops[0x15a] = Some(Op::new("BIT 3, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.de, U)))
        }));
        ops[0x15b] = Some(Op::new("BIT 3, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.de, D)))
        }));
        ops[0x15c] = Some(Op::new("BIT 3, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.hl, U)))
        }));
        ops[0x15d] = Some(Op::new("BIT 3, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, gr((&_r.hl, D)))
        }));
        ops[0x15e] = Some(Op::new("BIT 3, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x8, _m.get(grr(&_r.hl)))
        }));
        ops[0x15f] = Some(Op::new("BIT 3, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x8, tmp)
        }));

        ops[0x160] = Some(Op::new("BIT 4, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.bc, U)))
        }));
        ops[0x161] = Some(Op::new("BIT 4, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.bc, D)))
        }));
        ops[0x162] = Some(Op::new("BIT 4, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.de, U)))
        }));
        ops[0x163] = Some(Op::new("BIT 4, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.de, D)))
        }));
        ops[0x164] = Some(Op::new("BIT 4, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.hl, U)))
        }));
        ops[0x165] = Some(Op::new("BIT 4, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, gr((&_r.hl, D)))
        }));
        ops[0x166] = Some(Op::new("BIT 4, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x10, _m.get(grr(&_r.hl)))
        }));
        ops[0x167] = Some(Op::new("BIT 4, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x10, tmp)
        }));

        ops[0x168] = Some(Op::new("BIT 5, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.bc, U)))
        }));
        ops[0x169] = Some(Op::new("BIT 5, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.bc, D)))
        }));
        ops[0x16a] = Some(Op::new("BIT 5, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.de, U)))
        }));
        ops[0x16b] = Some(Op::new("BIT 5, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.de, D)))
        }));
        ops[0x16c] = Some(Op::new("BIT 5, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.hl, U)))
        }));
        ops[0x16d] = Some(Op::new("BIT 5, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, gr((&_r.hl, D)))
        }));
        ops[0x16e] = Some(Op::new("BIT 5, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x20, _m.get(grr(&_r.hl)))
        }));
        ops[0x16f] = Some(Op::new("BIT 5, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x20, tmp)
        }));

        ops[0x170] = Some(Op::new("BIT 6, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.bc, U)))
        }));
        ops[0x171] = Some(Op::new("BIT 6, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.bc, D)))
        }));
        ops[0x172] = Some(Op::new("BIT 6, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.de, U)))
        }));
        ops[0x173] = Some(Op::new("BIT 6, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.de, D)))
        }));
        ops[0x174] = Some(Op::new("BIT 6, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.hl, U)))
        }));
        ops[0x175] = Some(Op::new("BIT 6, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, gr((&_r.hl, D)))
        }));
        ops[0x176] = Some(Op::new("BIT 6, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x40, _m.get(grr(&_r.hl)))
        }));
        ops[0x177] = Some(Op::new("BIT 6, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x40, tmp)
        }));

        ops[0x178] = Some(Op::new("BIT 7, B", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.bc, U)))
        }));
        ops[0x179] = Some(Op::new("BIT 7, C", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.bc, D)))
        }));
        ops[0x17a] = Some(Op::new("BIT 7, D", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.de, U)))
        }));
        ops[0x17b] = Some(Op::new("BIT 7, E", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.de, D)))
        }));
        ops[0x17c] = Some(Op::new("BIT 7, H", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.hl, U)))
        }));
        ops[0x17d] = Some(Op::new("BIT 7, L", 2, (8, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, gr((&_r.hl, D)))
        }));
        ops[0x17e] = Some(Op::new("BIT 7, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            bit_msk_n(&mut _r.af, 0x80, _m.get(grr(&_r.hl)))
        }));
        ops[0x17f] = Some(Op::new("BIT 7, A", 2, (8, 0), |_r, _m, _p| -> bool {
            let tmp = gr((&_r.af, U));
            bit_msk_n(&mut _r.af, 0x80, tmp)
        }));

        ops[0x180] = Some(Op::new("RES 0, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.bc, U))
        }));
        ops[0x181] = Some(Op::new("RES 0, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.bc, D))
        }));
        ops[0x182] = Some(Op::new("RES 0, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.de, U))
        }));
        ops[0x183] = Some(Op::new("RES 0, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.de, D))
        }));
        ops[0x184] = Some(Op::new("RES 0, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.hl, U))
        }));
        ops[0x185] = Some(Op::new("RES 0, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.hl, D))
        }));
        ops[0x186] = Some(Op::new("RES 0, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x1, &_r.hl)
        }));
        ops[0x187] = Some(Op::new("RES 0, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x1, (&mut _r.af, U))
        }));

        ops[0x188] = Some(Op::new("RES 1, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.bc, U))
        }));
        ops[0x189] = Some(Op::new("RES 1, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.bc, D))
        }));
        ops[0x18a] = Some(Op::new("RES 1, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.de, U))
        }));
        ops[0x18b] = Some(Op::new("RES 1, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.de, D))
        }));
        ops[0x18c] = Some(Op::new("RES 1, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.hl, U))
        }));
        ops[0x18d] = Some(Op::new("RES 1, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.hl, D))
        }));
        ops[0x18e] = Some(Op::new("RES 1, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x2, &_r.hl)
        }));
        ops[0x18f] = Some(Op::new("RES 1, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x2, (&mut _r.af, U))
        }));

        ops[0x190] = Some(Op::new("RES 2, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.bc, U))
        }));
        ops[0x191] = Some(Op::new("RES 2, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.bc, D))
        }));
        ops[0x192] = Some(Op::new("RES 2, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.de, U))
        }));
        ops[0x193] = Some(Op::new("RES 2, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.de, D))
        }));
        ops[0x194] = Some(Op::new("RES 2, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.hl, U))
        }));
        ops[0x195] = Some(Op::new("RES 2, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.hl, D))
        }));
        ops[0x196] = Some(Op::new("RES 2, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x4, &_r.hl)
        }));
        ops[0x197] = Some(Op::new("RES 2, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x4, (&mut _r.af, U))
        }));

        ops[0x198] = Some(Op::new("RES 3, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.bc, U))
        }));
        ops[0x199] = Some(Op::new("RES 3, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.bc, D))
        }));
        ops[0x19a] = Some(Op::new("RES 3, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.de, U))
        }));
        ops[0x19b] = Some(Op::new("RES 3, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.de, D))
        }));
        ops[0x19c] = Some(Op::new("RES 3, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.hl, U))
        }));
        ops[0x19d] = Some(Op::new("RES 3, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.hl, D))
        }));
        ops[0x19e] = Some(Op::new("RES 3, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x8, &_r.hl)
        }));
        ops[0x19f] = Some(Op::new("RES 3, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x8, (&mut _r.af, U))
        }));

        ops[0x1a0] = Some(Op::new("RES 4, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.bc, U))
        }));
        ops[0x1a1] = Some(Op::new("RES 4, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.bc, D))
        }));
        ops[0x1a2] = Some(Op::new("RES 4, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.de, U))
        }));
        ops[0x1a3] = Some(Op::new("RES 4, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.de, D))
        }));
        ops[0x1a4] = Some(Op::new("RES 4, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.hl, U))
        }));
        ops[0x1a5] = Some(Op::new("RES 4, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.hl, D))
        }));
        ops[0x1a6] = Some(Op::new("RES 4, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x10, &_r.hl)
        }));
        ops[0x1a7] = Some(Op::new("RES 4, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x10, (&mut _r.af, U))
        }));

        ops[0x1a8] = Some(Op::new("RES 5, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.bc, U))
        }));
        ops[0x1a9] = Some(Op::new("RES 5, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.bc, D))
        }));
        ops[0x1aa] = Some(Op::new("RES 5, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.de, U))
        }));
        ops[0x1ab] = Some(Op::new("RES 5, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.de, D))
        }));
        ops[0x1ac] = Some(Op::new("RES 5, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.hl, U))
        }));
        ops[0x1ad] = Some(Op::new("RES 5, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.hl, D))
        }));
        ops[0x1ae] = Some(Op::new("RES 5, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x20, &_r.hl)
        }));
        ops[0x1af] = Some(Op::new("RES 5, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x20, (&mut _r.af, U))
        }));

        ops[0x1b0] = Some(Op::new("RES 6, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.bc, U))
        }));
        ops[0x1b1] = Some(Op::new("RES 6, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.bc, D))
        }));
        ops[0x1b2] = Some(Op::new("RES 6, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.de, U))
        }));
        ops[0x1b3] = Some(Op::new("RES 6, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.de, D))
        }));
        ops[0x1b4] = Some(Op::new("RES 6, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.hl, U))
        }));
        ops[0x1b5] = Some(Op::new("RES 6, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.hl, D))
        }));
        ops[0x1b6] = Some(Op::new("RES 6, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x40, &_r.hl)
        }));
        ops[0x1b7] = Some(Op::new("RES 6, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x40, (&mut _r.af, U))
        }));

        ops[0x1b8] = Some(Op::new("RES 7, B", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.bc, U))
        }));
        ops[0x1b9] = Some(Op::new("RES 7, C", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.bc, D))
        }));
        ops[0x1ba] = Some(Op::new("RES 7, D", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.de, U))
        }));
        ops[0x1bb] = Some(Op::new("RES 7, E", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.de, D))
        }));
        ops[0x1bc] = Some(Op::new("RES 7, H", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.hl, U))
        }));
        ops[0x1bd] = Some(Op::new("RES 7, L", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.hl, D))
        }));
        ops[0x1be] = Some(Op::new("RES 7, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            res_msk_arr(_m, 0x80, &_r.hl)
        }));
        ops[0x1bf] = Some(Op::new("RES 7, A", 2, (8, 0), |_r, _m, _p| -> bool {
            res_msk_r(0x80, (&mut _r.af, U))
        }));

        ops[0x1c0] = Some(Op::new("SET 0, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.bc, U))
        }));
        ops[0x1c1] = Some(Op::new("SET 0, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.bc, D))
        }));
        ops[0x1c2] = Some(Op::new("SET 0, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.de, U))
        }));
        ops[0x1c3] = Some(Op::new("SET 0, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.de, D))
        }));
        ops[0x1c4] = Some(Op::new("SET 0, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.hl, U))
        }));
        ops[0x1c5] = Some(Op::new("SET 0, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.hl, D))
        }));
        ops[0x1c6] = Some(Op::new("SET 0, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x1, &_r.hl)
        }));
        ops[0x1c7] = Some(Op::new("SET 0, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x1, (&mut _r.af, U))
        }));

        ops[0x1c8] = Some(Op::new("SET 1, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.bc, U))
        }));
        ops[0x1c9] = Some(Op::new("SET 1, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.bc, D))
        }));
        ops[0x1ca] = Some(Op::new("SET 1, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.de, U))
        }));
        ops[0x1cb] = Some(Op::new("SET 1, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.de, D))
        }));
        ops[0x1cc] = Some(Op::new("SET 1, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.hl, U))
        }));
        ops[0x1cd] = Some(Op::new("SET 1, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.hl, D))
        }));
        ops[0x1ce] = Some(Op::new("SET 1, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x2, &_r.hl)
        }));
        ops[0x1cf] = Some(Op::new("SET 1, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x2, (&mut _r.af, U))
        }));

        ops[0x1d0] = Some(Op::new("SET 2, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.bc, U))
        }));
        ops[0x1d1] = Some(Op::new("SET 2, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.bc, D))
        }));
        ops[0x1d2] = Some(Op::new("SET 2, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.de, U))
        }));
        ops[0x1d3] = Some(Op::new("SET 2, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.de, D))
        }));
        ops[0x1d4] = Some(Op::new("SET 2, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.hl, U))
        }));
        ops[0x1d5] = Some(Op::new("SET 2, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.hl, D))
        }));
        ops[0x1d6] = Some(Op::new("SET 2, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x4, &_r.hl)
        }));
        ops[0x1d7] = Some(Op::new("SET 2, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x4, (&mut _r.af, U))
        }));

        ops[0x1d8] = Some(Op::new("SET 3, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.bc, U))
        }));
        ops[0x1d9] = Some(Op::new("SET 3, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.bc, D))
        }));
        ops[0x1da] = Some(Op::new("SET 3, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.de, U))
        }));
        ops[0x1db] = Some(Op::new("SET 3, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.de, D))
        }));
        ops[0x1dc] = Some(Op::new("SET 3, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.hl, U))
        }));
        ops[0x1dd] = Some(Op::new("SET 3, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.hl, D))
        }));
        ops[0x1de] = Some(Op::new("SET 3, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x8, &_r.hl)
        }));
        ops[0x1df] = Some(Op::new("SET 3, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x8, (&mut _r.af, U))
        }));

        ops[0x1e0] = Some(Op::new("SET 4, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.bc, U))
        }));
        ops[0x1e1] = Some(Op::new("SET 4, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.bc, D))
        }));
        ops[0x1e2] = Some(Op::new("SET 4, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.de, U))
        }));
        ops[0x1e3] = Some(Op::new("SET 4, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.de, D))
        }));
        ops[0x1e4] = Some(Op::new("SET 4, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.hl, U))
        }));
        ops[0x1e5] = Some(Op::new("SET 4, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.hl, D))
        }));
        ops[0x1e6] = Some(Op::new("SET 4, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x10, &_r.hl)
        }));
        ops[0x1e7] = Some(Op::new("SET 4, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x10, (&mut _r.af, U))
        }));

        ops[0x1e8] = Some(Op::new("SET 5, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.bc, U))
        }));
        ops[0x1e9] = Some(Op::new("SET 5, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.bc, D))
        }));
        ops[0x1ea] = Some(Op::new("SET 5, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.de, U))
        }));
        ops[0x1eb] = Some(Op::new("SET 5, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.de, D))
        }));
        ops[0x1ec] = Some(Op::new("SET 5, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.hl, U))
        }));
        ops[0x1ed] = Some(Op::new("SET 5, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.hl, D))
        }));
        ops[0x1ee] = Some(Op::new("SET 5, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x20, &_r.hl)
        }));
        ops[0x1ef] = Some(Op::new("SET 5, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x20, (&mut _r.af, U))
        }));

        ops[0x1f0] = Some(Op::new("SET 6, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.bc, U))
        }));
        ops[0x1f1] = Some(Op::new("SET 6, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.bc, D))
        }));
        ops[0x1f2] = Some(Op::new("SET 6, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.de, U))
        }));
        ops[0x1f3] = Some(Op::new("SET 6, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.de, D))
        }));
        ops[0x1f4] = Some(Op::new("SET 6, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.hl, U))
        }));
        ops[0x1f5] = Some(Op::new("SET 6, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.hl, D))
        }));
        ops[0x1f6] = Some(Op::new("SET 6, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x40, &_r.hl)
        }));
        ops[0x1f7] = Some(Op::new("SET 6, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x40, (&mut _r.af, U))
        }));

        ops[0x1f8] = Some(Op::new("SET 7, B", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.bc, U))
        }));
        ops[0x1f9] = Some(Op::new("SET 7, C", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.bc, D))
        }));
        ops[0x1fa] = Some(Op::new("SET 7, D", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.de, U))
        }));
        ops[0x1fb] = Some(Op::new("SET 7, E", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.de, D))
        }));
        ops[0x1fc] = Some(Op::new("SET 7, H", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.hl, U))
        }));
        ops[0x1fd] = Some(Op::new("SET 7, L", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.hl, D))
        }));
        ops[0x1fe] = Some(Op::new("SET 7, (HL)", 2, (16, 0), |_r, _m, _p| -> bool {
            set_msk_arr(_m, 0x80, &_r.hl)
        }));
        ops[0x1ff] = Some(Op::new("SET 7, A", 2, (8, 0), |_r, _m, _p| -> bool {
            set_msk_r(0x80, (&mut _r.af, U))
        }));

        Ops(ops)
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
            Op::new("TEST1", 1, (1, 0), |_r, _m, _p| -> bool {
                srr(&mut _r.af, 0xff);
                true
            }),
            Op::new("TEST2", 1, (1, 0), |_r, _m, _p| -> bool {
                sr((&mut _r.af, U), 0xff);
                true
            }),
            Op::new("TEST3", 1, (1, 0), |_r, _m, _p| -> bool {
                sf((&mut _r.af, Z), true);
                true
            }),
            Op::new("TEST4", 1, (1, 0), |_r, _m, _p| -> bool {
                sf((&mut _r.af, Z), false);
                true
            }),
            Op::new("TEST5", 1, (1, 0), |_r, _m, _p| -> bool {
                sf((&mut _r.af, CY), false);
                _m.set(0xff, 0xf);
                true
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

    #[test]
    fn count() {
        let ops = Ops::new();
        let empty: [usize; 12] = [
            0xcb, 0xd3, 0xdb, 0xdd, 0xe3, 0xe4, 0xeb, 0xec, 0xed, 0xf4, 0xfc, 0xfd,
        ];

        for (i, op) in ops.0.iter().enumerate() {
            if let None = op {
                if !empty.contains(&i) {
                    panic!("panic at: {:x}", i);
                }
            }
        }
    }
}
