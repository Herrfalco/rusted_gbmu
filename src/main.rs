mod debug;
mod mem;
mod ops;
mod reg;
mod utils;

use debug::*;
use mem::*;
use ops::ops::*;
use reg::{api::*, *};
use std::env;
use std::path::Path;
use utils::*;

const DEBUG: bool = true;

fn read_opcode(mem: My, pc: MRR) -> u8 {
    mem.get(grr(pc))
}

fn read_param(mem: My, pc: MRR, len: usize) -> u16 {
    let mut result: u16 = 0;

    for i in 1..len as u16 {
        result |= (mem.get(grr(pc) + i) as u16) << (4 * (i - 1));
    }
    result
}

fn main() {
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
    let mut dbg = Debugger::new(DEBUG);

    let mut opcode: u8;
    let mut op: &Op;
    let mut param: u16;
    let mut tmp: u16;
    loop {
        opcode = read_opcode(&mem, &mut regs.pc);
        op = &ops
            .get(opcode as usize)
            .unwrap_or_else(|| fatal_err(&format!("Opcode 0x{:02x} not implemented", opcode), 3));
        param = read_param(&mem, &mut regs.pc, op.len());
        dbg.run(&mut mem, &mut regs, op, param);
        tmp = grr(&regs.pc).wrapping_add(op.len() as u16);
        srr(&mut regs.pc, tmp);
        op.exec(&mut regs, &mut mem, param);
    }
}
