use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

use crate::cpu::processor::Functions;
use crate::tests::common::*;
use crate::Memory;

// Path to the functional test binary

fn read_file_to_memory(
    memory: &mut Memory,
    file_addr: String,
    memory_offset_option: Option<usize>,
) -> () {
    let memory_offset = memory_offset_option.or(Some(0)).unwrap();
    let mut test_binary = File::open(file_addr).expect("File not found at address");
    let mut reader = BufReader::new(test_binary);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    for index in 0..buffer.len() {
        let value = buffer[index];
        memory.data[memory_offset + index] = value;
    }
}

pub fn functional_program_test() -> () {
    let (mut memory, mut processor) = setup();
    let home_dir = env::home_dir().unwrap();
    let program_path = format!(
        "{}/Desktop/Projects/emu-6502/src/binaries/functional_test.bin",
        home_dir.to_str().unwrap()
    );
    read_file_to_memory(&mut memory, program_path, Some(0x0A));

    processor.program_counter = 0x0400; // Program starts at 0x0400 minus 0x000A;
    println!("{:#X}", memory.data[0x0A]);

    let mut clock = 85000;

    while clock > 0 {
        processor.cycles = 85000;
        clock -= processor.execute(&mut memory);
    }

    println!(
        "{:X} | {:X}",
        processor, memory.data[processor.program_counter as usize]
    );
}
