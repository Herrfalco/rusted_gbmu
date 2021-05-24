use crate::mem::*;
use crate::reg::api::*;

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
    ld_r_ann(m, r, grr(rr));
    srr(rr, grr(rr).wrapping_add(1));
    true
}

pub fn ld_r_arrd(m: My, r: MR, rr: MRR) -> bool {
    ld_r_ann(m, r, grr(rr));
    srr(rr, grr(rr).wrapping_sub(1));
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
    sf((f, Z), false);
    sf((f, N), false);
    sf((f, H), ((grr(rr2) & 0xf) + ((sn & 0xf) as u16)) > 0xf);
    sf((f, CY), ((grr(rr2) & 0xff) + (sn as u8 as u16)) > 0xff);
    srr(rr1, grr(rr2).wrapping_add(sn as u16));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reg::*;

    #[test]
    fn ld_8() {
        let mut mem = Mem::new();
        let mut af = Reg::new();
        let mut hl = Reg::new();
        let mut bc = Reg::new();

        srr(&mut af, 0x1010);
        srr(&mut hl, 0x2020);
        srr(&mut bc, 0x3030);
        assert_eq!(grr(&af), 0x1010);
        assert_eq!(grr(&hl), 0x2020);
        assert_eq!(grr(&bc), 0x3030);
        ld_r_n((&mut af, U), 0xff);
        assert_eq!(grr(&af), 0xff10);
        srr(&mut af, 0x4040);
        ld_r_n((&mut bc, D), gr((&af, U)));
        assert_eq!(grr(&bc), 0x3040);
        ld_arr_n(&mut mem, &hl, 0x66);
        ld_r_ann(&mut mem, (&mut bc, D), grr(&hl));
        assert_eq!(grr(&bc), 0x3066);
        mem.set(0x33, 0x33);
        ld_r_ann(&mut mem, (&mut bc, U), 0x33);
        assert_eq!(grr(&bc), 0x3366);
        ld_arr_n(&mut mem, &bc, gr((&af, U)));
        assert_eq!(mem.get(grr(&bc)), 0x40);
        ld_ann_r(&mut mem, 0xfff, (&af, U));
        assert_eq!(mem.get(0xfff), 0x40);
        ldh_an_r(&mut mem, 0x10, (&af, U));
        assert_eq!(mem.get(0xff10), 0x40);
        ldh_an_r(&mut mem, 0x77, (&af, U));
        assert_eq!(mem.get(0xff77), 0x40);
        ldh_r_an(&mut mem, (&mut bc, D), 0x77);
        assert_eq!(gr((&bc, D)), 0x40);
        mem.set(0xff40, 0x42);
        ldh_r_an(&mut mem, (&mut hl, U), gr((&bc, D)));
        assert_eq!(gr((&hl, U)), 0x42);
        assert_eq!(grr(&hl), 0x4220);
        ld_arri_r(&mut mem, &mut hl, (&af, U));
        ld_r_arri(&mut mem, (&mut af, U), &mut hl);
        assert_eq!(grr(&hl), 0x4222);
        ld_arrd_r(&mut mem, &mut hl, (&af, U));
        ld_r_arrd(&mut mem, (&mut af, U), &mut hl);
        assert_eq!(grr(&hl), 0x4220);
    }

    #[test]
    fn ld_16() {
        let mut mem = Mem::new();
        let mut af = Reg::new();
        let mut hl = Reg::new();
        let mut sp = Reg::new();
        let mut bc = Reg::new();

        srr(&mut af, 0x1122);
        srr(&mut hl, 0x2020);
        srr(&mut sp, 0x3030);
        ld_rr_nn(&mut sp, 0xf0f0);
        assert_eq!(grr(&sp), 0xf0f0);
        ld_ann_rr(&mut mem, 0xf0f0, &af);
        pop_rr_arr(&mem, &mut hl, &mut sp);
        assert_eq!(grr(&hl), 0x1122);
        assert_eq!(grr(&sp), 0xf0f2);
        ld_rr_nn(&mut sp, grr(&hl));
        ld_rr_rrpsn(&mut bc, &mut af, &sp, -2);
        assert_eq!(grr(&af), 0x1120);
        push_arr_rr(&mut mem, &mut af, &mut hl);
        assert_eq!(grr(&af), 0x111e);
        assert_eq!(mem.get(0x111f), 0x11);
        assert_eq!(mem.get(0x111e), 0x22);
        pop_rr_arr(&mem, &mut sp, &mut af);
        assert_eq!(grr(&sp), 0x1122);
        srr(&mut sp, 0);
        ld_rr_rrpsn(&mut af, &mut bc, &sp, 1);
        assert_eq!(gr((&af, D)), 0);
        srr(&mut sp, 1);
        ld_rr_rrpsn(&mut af, &mut bc, &sp, -1);
        assert!(!gf((&af, Z)));
        assert!(!gf((&af, N)));
        assert!(gf((&af, H)));
        assert!(gf((&af, CY)));
        ld_rr_rrpsn(&mut af, &mut bc, &sp, 15);
        assert_eq!(gr((&af, D)), 0x20);
        srr(&mut sp, 0x10);
        ld_rr_rrpsn(&mut af, &mut bc, &sp, -16);
        assert_eq!(gr((&af, D)), 0x10);
    }

    #[test]
    fn acc_8() {
        let mut af = Reg::new();

        add_n(&mut af, 0x10);
        assert_eq!(gr((&af, U)), 0x10);
        assert!(!gf((&af, CY)));
        add_n(&mut af, 0xf0);
        assert_eq!(gr((&af, U)), 0);
        assert!(gf((&af, Z)));
        assert!(gf((&af, CY)));
        assert!(!gf((&af, H)));
        adc_n(&mut af, 0x10);
        assert_eq!(gr((&af, U)), 0x11);
        adc_n(&mut af, 0x10);
        assert_eq!(gr((&af, U)), 0x21);
        sub_n(&mut af, 0x22);
        assert_eq!(gr((&af, U)), 0xff);
        assert!(gf((&af, CY)));
        assert!(gf((&af, H)));
        sbc_n(&mut af, 0x10);
        assert_eq!(gr((&af, U)), 0xee);
        sbc_n(&mut af, 0x10);
        assert_eq!(gr((&af, U)), 0xde);
        and_n(&mut af, 0xf);
        assert_eq!(gr((&af, U)), 0xe);
        assert!(gf((&af, H)));
        assert!(!gf((&af, CY)));
        xor_n(&mut af, 0xfa);
        assert_eq!(gr((&af, U)), 0xf4);
        assert_eq!(gr((&af, D)), 0);
        or_n(&mut af, 0xff);
        assert_eq!(gr((&af, U)), 0xff);
        cp_n(&mut af, 0xff);
        assert!(gf((&af, Z)));
        sr((&mut af, U), 0x80);
        cp_n(&mut af, 0xff);
        assert!(gf((&af, CY)));
        cp_n(&mut af, 0x4f);
        assert!(!gf((&af, CY)));
        inc(&mut af);
        assert_eq!(gr((&af, U)), 0x81);
        dec(&mut af);
        assert_eq!(gr((&af, U)), 0x80);
    }

    #[test]
    fn var_8() {
        let mut mem = Mem::new();
        let mut af = Reg::new();
        let mut bc = Reg::new();

        srr(&mut bc, 0xf0f0);
        inc_r(&mut af, (&mut bc, U));
        assert_eq!(gr((&bc, U)), 0xf1);
        dec_r(&mut af, (&mut bc, U));
        assert_eq!(gr((&bc, U)), 0xf0);
        dec_arr(&mut af, &mut mem, &bc);
        assert_eq!(mem.get(grr(&bc)), 0xff);
        inc_arr(&mut af, &mut mem, &bc);
        assert_eq!(mem.get(grr(&bc)), 0);
        ld_r_n((&mut af, U), 0xaa);
        cpl(&mut af);
        assert_eq!(gr((&af, U)), 0x55);
        assert!(!gf((&af, CY)));
        scf(&mut af);
        assert!(gf((&af, CY)));
        ccf(&mut af);
        assert!(!gf((&af, CY)));
        ccf(&mut af);
        assert!(gf((&af, CY)));
    }

    #[test]
    fn daa_8() {
        let mut af = Reg::new();

        ld_r_n((&mut af, U), 0);
        add_n(&mut af, 0x10);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x10);
        ld_r_n((&mut af, U), 0x10);
        add_n(&mut af, 0xf0);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x60);
        ld_r_n((&mut af, U), 1);
        add_n(&mut af, 0xf);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x16);
        ld_r_n((&mut af, U), 1);
        add_n(&mut af, 0xff);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x66);
        ld_r_n((&mut af, U), 1);
        add_n(&mut af, 0xa5);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x6);
        ld_r_n((&mut af, U), 1);
        add_n(&mut af, 0x8a);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x91);
        ld_r_n((&mut af, U), 1);
        add_n(&mut af, 0xaa);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x11);
        ld_r_n((&mut af, U), 0);
        sub_n(&mut af, 0x10);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x90);
        ld_r_n((&mut af, U), 0x20);
        sub_n(&mut af, 0x12);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x08);
        ld_r_n((&mut af, U), 0);
        sub_n(&mut af, 0x1);
        daa(&mut af);
        assert_eq!(gr((&af, U)), 0x99);
    }

    #[test]
    fn jp_call() {
        let mut mem = Mem::new();
        let mut sp = Reg::new();
        let mut pc = Reg::new();
        let mut ime = Reg::new();

        ld_rr_nn(&mut sp, 0xffff);
        ld_rr_nn(&mut pc, 0x4242);
        ret_cc(&mut mem, &mut pc, &mut sp, false);
        jp_cc_nn(&mut pc, false, 0x2424);
        jr_cc_sn(&mut pc, false, -1);
        call_cc_nn(&mut mem, &mut sp, &mut pc, false, 0x2442);
        assert_eq!(grr(&pc), 0x4242);
        jp_cc_nn(&mut pc, true, 0x2424);
        assert_eq!(grr(&pc), 0x2424);
        jr_cc_sn(&mut pc, true, -1);
        assert_eq!(grr(&pc), 0x2423);
        ld_rr_nn(&mut pc, 0);
        jr_cc_sn(&mut pc, true, -1);
        assert_eq!(grr(&pc), 0xffff);
        jr_cc_sn(&mut pc, true, 10);
        assert_eq!(grr(&pc), 0x9);
        ld_rr_nn(&mut pc, 0x4224);
        call_cc_nn(&mut mem, &mut sp, &mut pc, true, 0x2442);
        assert_eq!(grr(&sp), 0xfffd);
        assert_eq!(grr(&pc), 0x2442);
        ret_cc(&mut mem, &mut pc, &mut sp, true);
        assert_eq!(grr(&sp), 0xffff);
        assert_eq!(grr(&pc), 0x4224);
        rst(&mut mem, &mut sp, &mut pc, 0x80);
        assert_eq!(grr(&sp), 0xfffd);
        assert_eq!(grr(&pc), 0x80);
        assert_eq!(grr(&ime), 0);
        reti(&mut mem, &mut ime, &mut pc, &mut sp);
        assert_eq!(grr(&sp), 0xffff);
        assert_eq!(grr(&pc), 0x4224);
        assert_eq!(grr(&ime), 1);
    }
}
