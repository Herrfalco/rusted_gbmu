mod debug;
mod disp;
mod mem;
mod ops;
mod reg;
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

fn handl_int(m: &mut Mem, r: &mut Regs) -> usize {
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
    4
}

fn main() {
    let mut dbg = Debugger::new(DEBUG);

    'restart: loop {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            fatal_err("Need a rom file as argument", 1);
        }

        let mut mem = Mem::new();
        if let Err(msg) = mem.load_rom(0x8000, Path::new(&args[0])) {
            fatal_err(msg, 2);
        }
        mem.init_spe_reg();

        let mut regs = Regs::new();
        regs.init();

        let ops = Ops::new();
        let mut disp = Display::new();

        let mut opcode: (u8, u8);
        let mut op: &Op;
        let mut param: u16;
        let mut tmp: u16;
        let mut cycles: usize = 0;
        loop {
            if cycles == 0 {
                if grr(&regs.ime) == 1 && ((mem.get(IE) & 0x1f) & (mem.get(IF) & 0x1f)) != 0 {
                    cycles = handl_int(&mut mem, &mut regs);
                } else {
                    opcode = read_opcode(&mem, &mut regs.pc);
                    op = &ops.get(opcode).unwrap_or_else(|| {
                        fatal_err(&format!("Opcode 0x{:02x} not implemented", opcode.0), 3)
                    });
                    param = read_param(&mem, &mut regs.pc, op.len());
                    if !dbg.run(&mut mem, &mut regs, op, param) {
                        continue 'restart;
                    }
                    tmp = grr(&regs.pc).wrapping_add(op.len() as u16);
                    srr(&mut regs.pc, tmp);
                    cycles = if op.exec(&mut regs, &mut mem, param) {
                        op.cycles.0
                    } else {
                        op.cycles.1
                    } / 4
                        - 1;
                    if grr(&regs.ime) > 1 {
                        dec_rr(&mut regs.ime);
                    }
                }
            } else {
                cycles -= 1;
            }
            disp.update(&mut mem);
        }
    }
}
