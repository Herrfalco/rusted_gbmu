use crate::mem::*;
use crate::reg::API::*;
use crate::reg::*;

pub fn ld_r_r(r1: MR, r2: R) {
    sr(r1, gr(r2));
}

pub fn ld_r_n(r: MR, n: u8) {
    sr(r, n);
}

pub fn ld_rr_nn(rr: MRR, nn: u16) {
    srr(rr, nn);
}

pub fn ld_arr_r(m: Mmy, rr: RR, r: R) {
    m.set(grr(rr), gr(r));
}

pub fn ld_arr_n(m: Mmy, rr: RR, n: u8) {
    m.set(grr(rr), n);
}

pub fn ld_r_arr(m: My, r: MR, rr: RR) {
    sr(r, m.get(grr(rr)));
}

pub fn ld_ann_r(m: Mmy, nn: u16, r: R) {
    m.set(nn, gr(r));
}

pub fn ld_r_ann(m: My, r: MR, nn: u16) {
    sr(r, m.get(nn));
}

pub fn ldh_an_r(m: Mmy, n: u8, r: R) {
    m.set(n as u16 | 0xff00, gr(r));
}

pub fn ldh_ar_r(m: Mmy, r1: R, r2: R) {
    m.set(gr(r1) as u16 | 0xff00, gr(r2));
}

pub fn ldh_r_an(m: My, r: MR, n: u8) {
    sr(r, m.get(n as u16 | 0xff00));
}

pub fn ldh_r_ar(m: My, r1: MR, r2: R) {
    sr(r1, m.get(gr(r2) as u16 | 0xff00));
}

pub fn ld_arri_r(m: Mmy, rr: MRR, r: R) {
    ld_arr_r(m, rr, r);
    srr(rr, grr(rr).wrapping_add(1));
}

pub fn ld_arrd_r(m: Mmy, rr: MRR, r: R) {
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

pub fn ld_to_U(rr: MRR) {
    srr(rr, (grr(rr) & 0xff) | (grr(rr) << 8));
}

pub fn ld_to_D(rr: MRR) {
    srr(rr, (grr(rr) & 0xff00) | (grr(rr) >> 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads() {
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
        ld_rr_nn(&mut af, 0x4040);
        assert_eq!(grr(&af), 0x4040);
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
        ld_to_U(&mut hl);
        assert_eq!(grr(&hl), 0x2020);
        ld_r_n((&mut hl, U), 0x66);
        ld_to_D(&mut hl);
        assert_eq!(grr(&hl), 0x6666);
    }
}
