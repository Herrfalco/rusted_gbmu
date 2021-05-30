mod debug;
mod disp;
mod mem;
mod ops;
mod reg;
mod timer;
mod utils;

use debug::*;
use disp::*;
use mem::*;
use ops::imp::dec_rr;
use ops::imp::rst;
use ops::ops::*;
use reg::{api::*, *};
use std::env;
use std::path::Path;
use timer::*;
use utils::*;

const DEBUG: bool = true;

fn read_opcode(mem: My, pc: RR) -> (u8, u8) {
    (mem.get(grr(pc)), mem.get(grr(pc).wrapping_add(1)))
}

fn read_param(mem: My, pc: MRR, len: usize) -> u16 {
    let mut result: u16 = 0;

    for i in 1..len as u16 {
        result |= (mem.get(grr(pc).wrapping_add(i)) as u16) << (8 * (i - 1));
    }
    result
}

fn handl_int(m: &mut Mem, r: &mut Regs) {
    srr(&mut r.ime, 0);
    for i in 0..5 {
        if (0x1 << i) & (m.get(IE) & 0x1f) & (m.get(IF) & 0x1f) != 0 {
            m.set(IF, m.get(IF) & !(0x1 << i));
            match i {
                0 => rst(m, &mut r.sp, &mut r.pc, 0x40),
                1 => rst(m, &mut r.sp, &mut r.pc, 0x48),
                2 => rst(m, &mut r.sp, &mut r.pc, 0x50),
                3 => rst(m, &mut r.sp, &mut r.pc, 0x58),
                4 => rst(m, &mut r.sp, &mut r.pc, 0x60),
                _ => true,
            };
            break;
        }
    }
}

fn main() {
    let mut reset = false;
    let mut dbg = Debugger::new(DEBUG);
    let mut disp = Display::new();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        fatal_err("Need a rom file as argument", 1);
    }

    loop {
        let mut mem = Mem::new();
        if let Err(msg) = mem.load_rom(0x8000, Path::new(&args[0])) {
            fatal_err(msg, 2);
        }
        if let Err(_) = mem.load_rom(0x100, Path::new("../roms/DMG_ROM.gb")) {
            fatal_err("Can't load bootrom", 11);
        }
        mem.init_spe_reg();

        if reset {
            if let Some(vram) = &mut dbg.vram {
                vram.update(&mut mem, true);
            }
            disp.reset();
        }

        let mut regs = Regs::new();
        regs.init();

        let ops = Ops::new();
        let mut timer = Timer::new(&mut mem);

        let mut opcode: (u8, u8);
        let mut op: &Op;
        let mut param: u16;
        let mut tmp: u16;
        let mut cycles: usize;
        let mut boot_rom = true;
        loop {
            if boot_rom && grr(&regs.pc) == 0x100 {
                if let Err(_) = mem.load_rom(0x100, Path::new(&args[0])) {
                    fatal_err("Can't disable bootrom", 20);
                }
                boot_rom = false;
            }
            if grr(&regs.ime) == 1 && ((mem.get(IE) & 0x1f) & (mem.get(IF) & 0x1f)) != 0 {
                handl_int(&mut mem, &mut regs);
                cycles = 20;
            } else {
                opcode = read_opcode(&mem, &mut regs.pc);
                op = &ops.get(opcode).unwrap_or_else(|| {
                    fatal_err(&format!("Opcode 0x{:02x} not implemented", opcode.0), 3)
                });
                param = read_param(&mem, &mut regs.pc, op.len());
                if !dbg.run(&mut mem, &mut regs, op, param) {
                    reset = true;
                    break;
                }
                tmp = grr(&regs.pc).wrapping_add(op.len() as u16);
                srr(&mut regs.pc, tmp);
                cycles = if op.exec(&mut regs, &mut mem, param) {
                    op.cycles.0
                } else {
                    op.cycles.1
                };
                if grr(&regs.ime) > 1 {
                    dec_rr(&mut regs.ime);
                }
            }
            timer.update(&mut mem, cycles);
            disp.update(&mut mem, cycles);
            //            println!("0x{:04x}", grr(&regs.pc));
        }
    }
}
