mod mem;
mod ops;
mod reg;

use mem::*;
use ops::ops::*;
use reg::*;
use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let val = "bonjour # love";
    println!("{}", val.replace("#", &format!("0x{:06x}", 143)));
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("Error: Need a rom file as argument");
        exit(1);
    }

    let mut mem = Mem::new();

    if let Err(msg) = mem.load_rom(0x8000, Path::new(&args[0])) {
        println!("Err: {}", msg);
    }
    let mut regs = Regs::new();

    let ops = Ops::new();
    loop {}
}
