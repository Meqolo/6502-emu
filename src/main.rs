// obelisk.me.uk/6502
mod cpu;
mod mem;
mod tests;

use mem::*;
use std::io;

fn main() {
    println!("Please enter a key depending on which you wish to run: ");
    println!("1) 6502 Test Suite");
    println!("2) 6502 Example Programs");
    let mut stdin_buffer = String::new();
    match io::stdin().read_line(&mut stdin_buffer) {
        Ok(_n) => {
            println!("\n\n");
            if stdin_buffer == "1" {
                tests::main::run();
            } else {
                tests::main::run_programs();
            }
        }
        Err(_error) => {}
    }
}
