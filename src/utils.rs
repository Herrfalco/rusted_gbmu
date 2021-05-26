use std::io::Write;
use std::process::exit;

pub fn fatal_err(msg: &str, status: i32) -> ! {
    println!("Error: {}", msg);
    exit(status)
}

pub fn pflush(msg: &str) {
    print!("{}", msg);
    std::io::stdout()
        .flush()
        .unwrap_or_else(|_| fatal_err("Can't flush stdout", 5));
}
