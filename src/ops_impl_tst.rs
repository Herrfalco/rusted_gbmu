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
fn misc_8() {
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

#[test]
fn arit_16() {
    let mut af = Reg::new();
    let mut hl = Reg::new();

    inc_rr(&mut hl);
    assert_eq!(grr(&hl), 1);
    dec_rr(&mut hl);
    dec_rr(&mut hl);
    assert_eq!(grr(&hl), 0xffff);
    add_rr_nn(&mut af, &mut hl, 1);
    assert!(gf((&af, H)));
    assert!(gf((&af, CY)));
    inc_rr(&mut hl);
    assert_eq!(grr(&hl), 1);
    add_rr_nn(&mut af, &mut hl, 0xfff);
    assert!(gf((&af, H)));
    assert!(!gf((&af, CY)));
    add_rr_nn(&mut af, &mut hl, 0xf000);
    assert!(!gf((&af, H)));
    assert!(gf((&af, CY)));
    add_rr_sn(&mut af, &mut hl, -1);
    assert_eq!(grr(&hl), 0xffff);
    assert!(!gf((&af, H)));
    assert!(!gf((&af, CY)));
    assert!(!gf((&af, N)));
    add_rr_sn(&mut af, &mut hl, 0x11);
    assert_eq!(grr(&hl), 0x10);
}

#[test]
fn rot_sh() {
    let mut af = Reg::new();

    sr((&mut af, U), 0x0);
    rlca(&mut af, false);
    assert!(gf((&af, Z)));
    sr((&mut af, U), 0xaa);
    rlca(&mut af, true);
    assert_eq!(gr((&af, U)), 0x55);
    assert!(gf((&af, CY)));
    rlca(&mut af, true);
    assert_eq!(gr((&af, U)), 0xaa);
    assert!(!gf((&af, CY)));
    rrca(&mut af, true);
    assert_eq!(gr((&af, U)), 0x55);
    assert!(!gf((&af, CY)));
    rrca(&mut af, true);
    assert_eq!(gr((&af, U)), 0xaa);
    assert!(gf((&af, CY)));
    rra(&mut af, true);
    assert_eq!(gr((&af, U)), 0xd5);
    assert!(!gf((&af, CY)));
    rla(&mut af, true);
    assert_eq!(gr((&af, U)), 0xaa);
    assert!(gf((&af, CY)));
    rla(&mut af, true);
    assert_eq!(gr((&af, U)), 0x55);
    assert!(gf((&af, CY)));
}

#[test]
fn cb_rot() {
    let mut mem = Mem::new();
    let mut hl = Reg::new();
    let mut af = Reg::new();
    let mut af2 = Reg::new();
    let mut bc = Reg::new();

    sr((&mut bc, U), 0xaa);
    mem.set(grr(&hl), 0xaa);
    rlc_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0x55);
    assert!(gf((&af, CY)));
    rlc_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0x55);
    assert!(gf((&af2, CY)));
    rlc_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0xaa);
    assert!(!gf((&af, CY)));
    rlc_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0xaa);
    assert!(!gf((&af2, CY)));
    rrc_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0x55);
    assert!(!gf((&af, CY)));
    rrc_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0x55);
    assert!(!gf((&af2, CY)));
    rrc_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0xaa);
    assert!(gf((&af, CY)));
    rrc_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0xaa);
    assert!(gf((&af2, CY)));
    rr_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0xd5);
    assert!(!gf((&af, CY)));
    rr_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0xd5);
    assert!(!gf((&af2, CY)));
    rl_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0xaa);
    assert!(gf((&af, CY)));
    rl_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0xaa);
    assert!(gf((&af2, CY)));
    rl_r(&mut af, (&mut bc, U));
    assert_eq!(gr((&bc, U)), 0x55);
    assert!(gf((&af, CY)));
    rl_arr(&mut af2, &mut mem, &hl);
    assert_eq!(mem.get(grr(&hl)), 0x55);
    assert!(gf((&af2, CY)));
}

#[test]
fn sh_bit() {
    let mut mem = Mem::new();
    let mut hl = Reg::new();
    let mut af = Reg::new();
    let mut af2 = Reg::new();
    let mut bc = Reg::new();

    sr((&mut bc, U), 0x55);
    mem.set(grr(&hl), 0x55);
    sla_r(&mut af, (&mut bc, U));
    sla_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0xaa);
    assert!(!gf((&af, CY)));
    assert_eq!(mem.get(grr(&hl)), 0xaa);
    assert!(!gf((&af2, CY)));
    sla_r(&mut af, (&mut bc, U));
    sla_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0x54);
    assert!(gf((&af, CY)));
    assert_eq!(mem.get(grr(&hl)), 0x54);
    assert!(gf((&af2, CY)));
    sla_r(&mut af, (&mut bc, U));
    sla_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0xa8);
    sra_r(&mut af, (&mut bc, U));
    sra_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0xd4);
    assert!(!gf((&af, CY)));
    assert_eq!(mem.get(grr(&hl)), 0xd4);
    assert!(!gf((&af2, CY)));
    srl_r(&mut af, (&mut bc, U));
    srl_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0x6a);
    assert!(!gf((&af, CY)));
    assert_eq!(mem.get(grr(&hl)), 0x6a);
    assert!(!gf((&af2, CY)));
    swap_r(&mut af, (&mut bc, U));
    swap_arr(&mut af2, &mut mem, &hl);
    assert_eq!(gr((&bc, U)), 0xa6);
    assert_eq!(mem.get(grr(&hl)), 0xa6);
    bit_msk_n(&mut af, 0x80, gr((&bc, U)));
    assert!(!gf((&af, Z)));
    bit_msk_n(&mut af, 0x1, gr((&bc, U)));
    assert!(gf((&af, Z)));
    res_msk_r(0x80, (&mut bc, U));
    res_msk_arr(&mut mem, 0x80, &hl);
    bit_msk_n(&mut af, 0x80, gr((&bc, U)));
    assert!(gf((&af, Z)));
    bit_msk_n(&mut af, 0x80, mem.get(grr(&hl)));
    assert!(gf((&af, Z)));
    set_msk_r(0x80, (&mut bc, U));
    set_msk_arr(&mut mem, 0x80, &hl);
    bit_msk_n(&mut af, 0x80, gr((&bc, U)));
    assert!(!gf((&af, Z)));
    bit_msk_n(&mut af, 0x80, mem.get(grr(&hl)));
    assert!(!gf((&af, Z)));
}
