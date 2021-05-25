use crate::mem::*;
use crate::reg::api::*;

#[cfg(test)]
mod ops_impl_tst;

///////////////////////// 8 BITS LOADS ///////////////////////////

pub fn ld_r_n(r: MR, n: u8) -> bool {
    sr(r, n);
    true
}

pub fn ld_arr_n(m: MMy, rr: RR, n: u8) -> bool {
    m.set(grr(rr), n);
    true
}

pub fn ld_ann_r(m: MMy, nn: u16, r: R) -> bool {
    m.set(nn, gr(r));
    true
}

pub fn ld_r_ann(m: My, r: MR, nn: u16) -> bool {
    sr(r, m.get(nn));
    true
}

pub fn ldh_an_r(m: MMy, n: u8, r: R) -> bool {
    m.set(n as u16 | 0xff00, gr(r));
    true
}

pub fn ldh_ar_r(m: MMy, r1: R, r2: R) -> bool {
    m.set(gr(r1) as u16 | 0xff00, gr(r2));
    true
}

pub fn ldh_r_an(m: My, r: MR, n: u8) -> bool {
    sr(r, m.get(n as u16 | 0xff00));
    true
}

pub fn ld_arri_r(m: MMy, rr: MRR, r: R) -> bool {
    ld_arr_n(m, rr, gr(r));
    srr(rr, grr(rr).wrapping_add(1));
    true
}

pub fn ld_arrd_r(m: MMy, rr: MRR, r: R) -> bool {
    ld_arr_n(m, rr, gr(r));
    srr(rr, grr(rr).wrapping_sub(1));
    true
}

pub fn ld_r_arri(m: My, r: MR, rr: MRR) -> bool {
    let tmp = grr(rr);

    ld_r_ann(m, r, tmp);
    srr(rr, tmp.wrapping_add(1));
    true
}

pub fn ld_r_arrd(m: My, r: MR, rr: MRR) -> bool {
    let tmp = grr(rr);

    ld_r_ann(m, r, tmp);
    srr(rr, tmp.wrapping_sub(1));
    true
}

///////////////////////// 16 BITS LOADS ///////////////////////////

pub fn ld_rr_nn(rr: MRR, nn: u16) -> bool {
    srr(rr, nn);
    true
}

pub fn ld_ann_rr(m: MMy, nn: u16, rr: RR) -> bool {
    m.set(nn, gr((rr, D)));
    m.set(nn.wrapping_add(1), gr((rr, U)));
    true
}

pub fn ld_rr_rrpsn(f: MRR, rr1: MRR, rr2: RR, sn: i8) -> bool {
    let tmp = grr(rr2);

    sf((f, Z), false);
    sf((f, N), false);
    sf((f, H), (tmp & 0xf) + (sn & 0xf) as u16 > 0xf);
    sf((f, CY), (tmp & 0xff) + sn as u8 as u16 > 0xff);
    srr(rr1, tmp.wrapping_add(sn as u16));
    true
}

pub fn pop_rr_arr(m: My, rr1: MRR, rr2: MRR) -> bool {
    sr((rr1, D), m.get(grr(rr2)));
    srr(rr2, grr(rr2).wrapping_add(1));
    sr((rr1, U), m.get(grr(rr2)));
    srr(rr2, grr(rr2).wrapping_add(1));
    true
}

pub fn push_arr_rr(m: MMy, rr1: MRR, rr2: RR) -> bool {
    srr(rr1, grr(rr1).wrapping_sub(1));
    m.set(grr(rr1), gr((rr2, U)));
    srr(rr1, grr(rr1).wrapping_sub(1));
    m.set(grr(rr1), gr((rr2, D)));
    true
}

////////////////////// 8 BITS ARITHMETIC ///////////////////////

pub fn add_n(af: MRR, n: u8) -> bool {
    let tmp = gr((af, U));
    let result = tmp.wrapping_add(n);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), (n & 0xf) + (tmp & 0xf) > 0xf);
    sf((af, CY), (n as u16) + (tmp as u16) > 0xff);
    true
}

pub fn adc_n(af: MRR, n: u8) -> bool {
    let tmp = gr((af, U));
    let c: u8 = if gf((af, CY)) { 1 } else { 0 };
    let result = tmp.wrapping_add(n).wrapping_add(c);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), (n & 0xf) + (tmp & 0xf) + c > 0xf);
    sf((af, CY), (n as u16) + (tmp as u16) + (c as u16) > 0xff);
    true
}

pub fn sub_n(af: MRR, n: u8) -> bool {
    let tmp = gr((af, U));
    let result = tmp.wrapping_sub(n);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), true);
    sf((af, H), (n & 0xf) > (tmp & 0xf));
    sf((af, CY), n > tmp);
    true
}

pub fn sbc_n(af: MRR, n: u8) -> bool {
    let tmp = gr((af, U));
    let c: u8 = if gf((af, CY)) { 1 } else { 0 };
    let result = tmp.wrapping_sub(n).wrapping_sub(c);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), true);
    sf((af, H), (n & 0xf) + c > tmp & 0xf);
    sf((af, CY), (n as u16) + (c as u16) > tmp as u16);
    true
}

pub fn and_n(af: MRR, n: u8) -> bool {
    let result = gr((af, U)) & n;

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), true);
    sf((af, CY), false);
    true
}

pub fn xor_n(af: MRR, n: u8) -> bool {
    let result = gr((af, U)) ^ n;

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), false);
    true
}

pub fn or_n(af: MRR, n: u8) -> bool {
    let result = gr((af, U)) | n;

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), false);
    true
}

pub fn cp_n(af: MRR, n: u8) -> bool {
    let tmp = gr((af, U));

    sf((af, Z), tmp.wrapping_sub(n) == 0);
    sf((af, N), true);
    sf((af, H), (n & 0xf) > (tmp & 0xf));
    sf((af, CY), n > tmp);
    true
}

pub fn inc_r(f: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = tmp.wrapping_add(1);

    sr(r, result);
    sf((f, Z), result == 0);
    sf((f, N), false);
    sf((f, H), (tmp & 0xf) + 1 > 0xf);
    true
}

pub fn inc(af: MRR) -> bool {
    let tmp = gr((af, U));
    let result = tmp.wrapping_add(1);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), (tmp & 0xf) + 1 > 0xf);
    true
}

pub fn dec_r(f: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = tmp.wrapping_sub(1);

    sr(r, result);
    sf((f, Z), result == 0);
    sf((f, N), true);
    sf((f, H), (tmp & 0xf) == 0);
    true
}

pub fn dec(af: MRR) -> bool {
    let tmp = gr((af, U));
    let result = tmp.wrapping_sub(1);

    sr((af, U), result);
    sf((af, Z), result == 0);
    sf((af, N), true);
    sf((af, H), (tmp & 0xf) == 0);
    true
}

pub fn inc_arr(f: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = tmp.wrapping_add(1);

    m.set(grr(rr), result);
    sf((f, Z), result == 0);
    sf((f, N), false);
    sf((f, H), (tmp & 0xf) + 1 > 0xf);
    true
}

pub fn dec_arr(f: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = tmp.wrapping_sub(1);

    m.set(grr(rr), result);
    sf((f, Z), result == 0);
    sf((f, N), true);
    sf((f, H), (tmp & 0xf) == 0);
    true
}

pub fn cpl(af: MRR) -> bool {
    let result = !gr((af, U));

    sr((af, U), result);
    sf((af, N), true);
    sf((af, H), true);
    true
}

pub fn scf(f: MRR) -> bool {
    sf((f, N), false);
    sf((f, H), false);
    sf((f, CY), true);
    true
}

pub fn ccf(f: MRR) -> bool {
    let result = if gf((f, CY)) { false } else { true };

    sf((f, N), false);
    sf((f, H), false);
    sf((f, CY), result);
    true
}

pub fn daa(af: MRR) -> bool {
    let mut tmp = gr((af, U));

    if gf((af, N)) {
        if gf((af, CY)) {
            sr((af, U), tmp.wrapping_sub(0x60));
        }
        tmp = gr((af, U));
        if gf((af, H)) {
            sr((af, U), tmp.wrapping_sub(0x6));
        }
    } else {
        if gf((af, CY)) || tmp > 0x99 {
            sr((af, U), tmp.wrapping_add(0x60));
            sf((af, CY), true);
        }
        tmp = gr((af, U));
        if gf((af, H)) || ((tmp & 0xf) > 0x09) {
            sr((af, U), tmp.wrapping_add(0x6));
        }
    }
    true
}

///////////////////////// JUMP/CALLS //////////////////////////

pub fn jp_cc_nn(pc: MRR, cc: bool, nn: u16) -> bool {
    if cc {
        srr(pc, nn);
        return true;
    }
    false
}

pub fn jr_cc_sn(pc: MRR, cc: bool, sn: i8) -> bool {
    if cc {
        srr(pc, grr(pc).wrapping_add(sn as u16));
        return true;
    }
    false
}

pub fn call_cc_nn(m: MMy, sp: MRR, pc: MRR, cc: bool, nn: u16) -> bool {
    if cc {
        push_arr_rr(m, sp, pc);
        srr(pc, nn);
        return true;
    }
    false
}

pub fn ret_cc(m: MMy, pc: MRR, sp: MRR, cc: bool) -> bool {
    if cc {
        pop_rr_arr(m, pc, sp);
        return true;
    }
    false
}

pub fn reti(m: MMy, ime: MRR, pc: MRR, sp: MRR) -> bool {
    pop_rr_arr(m, pc, sp);
    srr(ime, 1);
    true
}

pub fn rst(m: MMy, sp: MRR, pc: MRR, nn: u16) -> bool {
    push_arr_rr(m, sp, pc);
    srr(pc, nn);
    true
}

////////////////////// 16 BITS ARITHMETIC ///////////////////////

pub fn inc_rr(rr: MRR) -> bool {
    srr(rr, grr(rr).wrapping_add(1));
    true
}

pub fn dec_rr(rr: MRR) -> bool {
    srr(rr, grr(rr).wrapping_sub(1));
    true
}

pub fn add_rr_nn(af: MRR, rr: MRR, nn: u16) -> bool {
    let tmp = grr(rr);

    srr(rr, tmp.wrapping_add(nn));
    sf((af, N), false);
    sf((af, H), (nn & 0xfff) + (tmp & 0xfff) > 0xfff);
    sf((af, CY), nn as u32 + tmp as u32 > 0xffff);
    true
}

pub fn add_rr_sn(af: MRR, rr: MRR, sn: i8) -> bool {
    let tmp = grr(rr);

    srr(rr, tmp.wrapping_add(sn as u16));
    sf((af, Z), false);
    sf((af, N), false);
    sf((af, H), ((tmp & 0xf) + ((sn & 0xf) as u16)) > 0xf);
    sf((af, CY), ((tmp & 0xff) + (sn as u8 as u16)) > 0xff);
    true
}

////////////////////// ROTATE/SHIFT ///////////////////////

pub fn rlca(af: MRR, res_z: bool) -> bool {
    let tmp = gr((af, U));
    let result = (tmp << 1) | (tmp >> 7);

    sf((af, Z), if res_z { false } else { result == 0 });
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    sr((af, U), result);
    true
}

pub fn rla(af: MRR, res_z: bool) -> bool {
    let tmp = gr((af, U));
    let cy = if gf((af, CY)) { 1 } else { 0 };
    let result = (tmp << 1) | cy;

    sf((af, Z), if res_z { false } else { result == 0 });
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    sr((af, U), result);
    true
}

pub fn rrca(af: MRR, res_z: bool) -> bool {
    let tmp = gr((af, U));
    let result = (tmp >> 1) | (tmp << 7);

    sf((af, Z), if res_z { false } else { result == 0 });
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    sr((af, U), result);
    true
}

pub fn rra(af: MRR, res_z: bool) -> bool {
    let tmp = gr((af, U));
    let cy = if gf((af, CY)) { 0x80 } else { 0 };
    let result = (tmp >> 1) | cy;

    sf((af, Z), if res_z { false } else { result == 0 });
    sf((af, N), false);
    sf((af, H), false);
    sr((af, U), result);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    true
}

////////////////////// CB OPS ///////////////////////

pub fn rlc_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = (tmp << 1) | (tmp >> 7);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn rlc_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = (tmp << 1) | (tmp >> 7);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn rl_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let cy = if gf((af, CY)) { 1 } else { 0 };
    let result = (tmp << 1) | cy;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn rl_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let cy = if gf((af, CY)) { 1 } else { 0 };
    let result = (tmp << 1) | cy;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn rrc_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = (tmp >> 1) | (tmp << 7);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn rrc_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = (tmp >> 1) | (tmp << 7);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn rr_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let cy = if gf((af, CY)) { 0x80 } else { 0 };
    let result = (tmp >> 1) | cy;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn rr_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let cy = if gf((af, CY)) { 0x80 } else { 0 };
    let result = (tmp >> 1) | cy;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn sla_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = tmp << 1;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn sla_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = tmp << 1;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x80 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn sra_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = (tmp >> 1) | (tmp & 0x80);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn sra_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = (tmp >> 1) | (tmp & 0x80);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn srl_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = tmp >> 1;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    sr(r, result);
    true
}

pub fn srl_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = tmp >> 1;

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), if tmp & 0x1 == 0 { false } else { true });
    m.set(grr(rr), result);
    true
}

pub fn swap_r(af: MRR, r: MR) -> bool {
    let tmp = gr((r.0, r.1));
    let result = (tmp >> 4) | (tmp << 4);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), false);
    sr(r, result);
    true
}

pub fn swap_arr(af: MRR, m: MMy, rr: RR) -> bool {
    let tmp = m.get(grr(rr));
    let result = (tmp >> 4) | (tmp << 4);

    sf((af, Z), result == 0);
    sf((af, N), false);
    sf((af, H), false);
    sf((af, CY), false);
    m.set(grr(rr), result);
    true
}

pub fn bit_msk_n(af: MRR, msk: u8, n: u8) -> bool {
    sf((af, N), false);
    sf((af, H), true);
    sf((af, Z), n & msk == 0);
    true
}

pub fn res_msk_r(msk: u8, r: MR) -> bool {
    let tmp = gr((r.0, r.1)) & !msk;

    sr(r, tmp);
    true
}

pub fn res_msk_arr(m: MMy, msk: u8, rr: RR) -> bool {
    m.set(grr(rr), m.get(grr(rr)) & !msk);
    true
}

pub fn set_msk_r(msk: u8, r: MR) -> bool {
    let tmp = gr((r.0, r.1)) | msk;

    sr(r, tmp);
    true
}

pub fn set_msk_arr(m: MMy, msk: u8, rr: RR) -> bool {
    m.set(grr(rr), m.get(grr(rr)) | msk);
    true
}

////////////////////// MISC/CONTROL ///////////////////////

/////ADD TESTS !
/////need to be implemented (low consumption)
pub fn stop() -> bool {
    true
}

/////need to be implemented (low consumption)
pub fn halt() -> bool {
    true
}

pub fn di(ime: MRR) -> bool {
    srr(ime, 0);
    true
}

/////need to be dec in main loop
pub fn ei(ime: MRR) -> bool {
    srr(ime, 3);
    true
}
