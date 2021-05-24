use crate::mem::*;
use crate::ops_impl::*;
use crate::reg::api::*;
use crate::reg::*;

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
}

pub struct Ops(Vec<Option<Op>>);

impl Ops {
    pub fn new() -> Ops {
        let mut ops: Vec<Option<Op>> = (0..0x100).map(|_| None).collect();

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
            add_n(&mut _r.af, gr((&_r.bc, U)))
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
            adc_n(&mut _r.af, gr((&_r.bc, U)))
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
            sub_n(&mut _r.af, gr((&_r.bc, U)))
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
            sbc_n(&mut _r.af, gr((&_r.bc, U)))
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
            and_n(&mut _r.af, gr((&_r.bc, U)))
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
            xor_n(&mut _r.af, gr((&_r.bc, U)))
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
            or_n(&mut _r.af, gr((&_r.bc, U)))
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
            cp_n(&mut _r.af, gr((&_r.bc, U)))
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
}
