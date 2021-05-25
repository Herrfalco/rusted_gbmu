mod mem;
mod ops;
mod reg;

use mem::*;
use ops::ops::*;
use reg::{api::*, *};
use std::env;
use std::io::Write;
use std::{path::Path, process::exit};
use text_io::read;

fn fatal_err(msg: &str, status: i32) -> ! {
    println!("Error: {}", msg);
    exit(status)
}

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

fn pflush(msg: &str) {
    print!("{}", msg);
    std::io::stdout()
        .flush()
        .unwrap_or_else(|_| fatal_err("Can't flush stdout", 5));
}

fn main() {
    /*
    let val = "bonjour # love";
    println!("{}", val.replace("#", &format!("0x{:06x}", 143)));
    */
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

    let mut saved_pc: u16;
    let mut entry: String;
    let mut opcode: u8;
    let mut op: &Op;
    let mut param: u16;
    let mut fm_par: String;
    let mut tmp: u16;
    loop {
        saved_pc = grr(&regs.pc);
        opcode = read_opcode(&mem, &mut regs.pc);
        op = &ops
            .get(opcode as usize)
            .unwrap_or_else(|| fatal_err(&format!("Opcode 0x{:02x} not implemented", opcode), 3));
        param = read_param(&mem, &mut regs.pc, op.len());
        fm_par = match op.len() {
            1 => String::from(""),
            2 => format!("0x{:02x}", param),
            3 => format!("0x{:04x}", param),
            _ => fatal_err("Wrong operation length", 4),
        };
        println!(
            "0x{:04x}:  {}",
            saved_pc,
            format!("{}", op).replace("#", &fm_par)
        );
        loop {
            pflush("> ");
            entry = read!("{}\n");
            match &entry[..] {
                "" => break,
                "r" => println!("{}", regs),
                _ => println!("Error: Unknown command"),
            }
        }
        tmp = grr(&regs.pc) + op.len() as u16;
        srr(&mut regs.pc, tmp);
        op.exec(&mut regs, &mut mem, param);
    }
}
