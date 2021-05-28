use std::process::exit;

pub const COLORS: [u32; 5] = [0xd6ecd2, 0x99d18f, 0x61bd4f, 0x519839, 0x3f6f21];

pub fn fatal_err(msg: &str, status: i32) -> ! {
    println!("Error: {}", msg);
    exit(status)
}
