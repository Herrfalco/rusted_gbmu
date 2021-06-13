//pub const COLORS: [u32; 5] = [0xd9ffdb, 0x8fff94, 0x00cf0a, 0x006605, 0x002902];
pub const COLORS: [u32; 5] = [0x8fff94, 0x8fff94, 0x00cf0a, 0x006605, 0x002902];

pub fn fatal_err(msg: &str, status: i32) -> ! {
    eprintln!("Error: {}", msg);
    quit::with_code(status);
}
