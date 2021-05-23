use crate::mem::*;
use crate::reg::api::*;

///////////////////////// 8 BITS LOADS ///////////////////////////

pub fn ld_r_r(r1: MR, r2: R) {
    sr(r1, gr(r2));
}

pub fn ld_r_n(r: MR, n: u8) {
    sr(r, n);
}

pub fn ld_arr_r(m: MMy, rr: RR, r: R) {
    m.set(grr(rr), gr(r));
}

pub fn ld_arr_n(m: MMy, rr: RR, n: u8) {
    m.set(grr(rr), n);
}

pub fn ld_r_arr(m: My, r: MR, rr: RR) {
    sr(r, m.get(grr(rr)));
}

pub fn ld_ann_r(m: MMy, nn: u16, r: R) {
    m.set(nn, gr(r));
}

pub fn ld_r_ann(m: My, r: MR, nn: u16) {
    sr(r, m.get(nn));
}

pub fn ldh_an_r(m: MMy, n: u8, r: R) {
    m.set(n as u16 | 0xff00, gr(r));
}

pub fn ldh_ar_r(m: MMy, r1: R, r2: R) {
    m.set(gr(r1) as u16 | 0xff00, gr(r2));
}

pub fn ldh_r_an(m: My, r: MR, n: u8) {
    sr(r, m.get(n as u16 | 0xff00));
}

pub fn ldh_r_ar(m: My, r1: MR, r2: R) {
    sr(r1, m.get(gr(r2) as u16 | 0xff00));
}

pub fn ld_arri_r(m: MMy, rr: MRR, r: R) {
    ld_arr_r(m, rr, r);
    srr(rr, grr(rr).wrapping_add(1));
}

pub fn ld_arrd_r(m: MMy, rr: MRR, r: R) {
    ld_arr_r(m, rr, r);
    srr(rr, grr(rr).wrapping_sub(1));
}

pub fn ld_r_arri(m: My, r: MR, rr: MRR) {
    ld_r_arr(m, r, rr);
    srr(rr, grr(rr).wrapping_add(1));
}

pub fn ld_r_arrd(m: My, r: MR, rr: MRR) {
    ld_r_arr(m, r, rr);
    srr(rr, grr(rr).wrapping_sub(1));
}

pub fn ld_pass() {}

pub fn ld_to_u(rr: MRR) {
    srr(rr, (grr(rr) & 0xff) | (grr(rr) << 8));
}

pub fn ld_to_d(rr: MRR) {
    srr(rr, (grr(rr) & 0xff00) | (grr(rr) >> 8));
}

///////////////////////// 16 BITS LOADS ///////////////////////////

pub fn ld_rr_nn(rr: MRR, nn: u16) {
    srr(rr, nn);
}

pub fn ld_ann_rr(m: MMy, nn: u16, rr: RR) {
    m.set(nn, gr((rr, D)));
    m.set(nn.wrapping_add(1), gr((rr, U)));
}

pub fn ld_rr_rrpsn(f: MRR, rr1: MRR, rr2: RR, sn: i8) {
    sr((f, D), 0);
    sf((f, H), ((grr(rr2) & 0xf) + ((sn & 0xf) as u16)) > 0xf);
    sf((f, CY), ((grr(rr2) & 0xff) + (sn as u8 as u16)) > 0xff);
    srr(rr1, grr(rr2).wrapping_add(sn as u16));
}

pub fn ld_rr_rr(rr1: MRR, rr2: RR) {
    srr(rr1, grr(rr2));
}

pub fn pop_rr_arr(m: My, rr1: MRR, rr2: MRR) {
    sr((rr1, D), m.get(grr(rr2)));
    srr(rr2, grr(rr2).wrapping_add(1));
    sr((rr1, U), m.get(grr(rr2)));
    srr(rr2, grr(rr2).wrapping_add(1));
}

pub fn push_arr_rr(m: MMy, rr1: MRR, rr2: RR) {
    srr(rr1, grr(rr1).wrapping_sub(1));
    m.set(grr(rr1), gr((rr2, U)));
    srr(rr1, grr(rr1).wrapping_sub(1));
    m.set(grr(rr1), gr((rr2, D)));
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
        ld_r_r((&mut bc, D), (&af, U));
        assert_eq!(grr(&bc), 0x3040);
        ld_arr_n(&mut mem, &hl, 0x66);
        ld_r_arr(&mut mem, (&mut bc, D), &hl);
        assert_eq!(grr(&bc), 0x3066);
        mem.set(0x33, 0x33);
        ld_r_ann(&mut mem, (&mut bc, U), 0x33);
        assert_eq!(grr(&bc), 0x3366);
        ld_arr_r(&mut mem, &bc, (&af, U));
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
        ldh_r_ar(&mut mem, (&mut hl, U), (&bc, D));
        assert_eq!(gr((&hl, U)), 0x42);
        assert_eq!(grr(&hl), 0x4220);
        ld_arri_r(&mut mem, &mut hl, (&af, U));
        ld_r_arri(&mut mem, (&mut af, U), &mut hl);
        assert_eq!(grr(&hl), 0x4222);
        ld_arrd_r(&mut mem, &mut hl, (&af, U));
        ld_r_arrd(&mut mem, (&mut af, U), &mut hl);
        assert_eq!(grr(&hl), 0x4220);
        ld_to_u(&mut hl);
        assert_eq!(grr(&hl), 0x2020);
        ld_r_n((&mut hl, U), 0x66);
        ld_to_d(&mut hl);
        assert_eq!(grr(&hl), 0x6666);
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
        ld_rr_rr(&mut sp, &hl);
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
}
