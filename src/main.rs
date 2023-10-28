mod cache;
mod core;
mod decoder;
mod instruction;
// mod instruction_cache;
mod instruction_memory;
mod memory;
mod register;
mod types;
mod utils;
use crate::types::*;
use crate::{core::*, instruction_memory::INSTRUCTION_MEMORY_SIZE};
// use instruction::*;
use std::{
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
};
// use types::*;
// use utils::*;

fn main() {
    let mut core = Core::new();
    core.set_int_register(1, INSTRUCTION_MEMORY_SIZE as Int);
    core.set_int_register(2, 10000000);
    print!("binary file name: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop().unwrap();
    match File::open(input) {
        Err(e) => {
            println!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut inst_count = 0;
            for line in reader.lines() {
                let input = line.unwrap();
                let input: Vec<char> = input.chars().collect();
                let mut inst = 0;
                for c in input {
                    inst += ((c as u32) << (inst_count % 4) * 8) as u32;
                    // core.store_byte(inst_count, u8_to_i8(inst));
                    inst_count += 1;
                    if inst_count % 4 == 0 {
                        core.store_instruction(inst_count - 4, inst);
                        inst = 0;
                    }
                }
            }
            if inst_count % 4 != 0 {
                eprintln!("Reading file failed.\nThe size of sum of instructions is not a multiple of 4. {}", inst_count);
            }
            core.run(true, 0);
        }
    }
    // loop {
    //     print!("> ");
    //     io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");
    //     input.pop().unwrap();
    //     let input: &str = &input;
    //     match input {
    //         "sr" => {
    //             core.show_registers();
    //         }
    //         "sm" => {
    //             core.show_memory();
    //         }
    //         "lf" => {
    //             print!("file name: ");
    //             stdout().flush().unwrap();
    //             let mut input = String::new();
    //             io::stdin()
    //                 .read_line(&mut input)
    //                 .expect("Failed to read line");
    //             input.pop().unwrap();
    //             match File::open(input) {
    //                 Err(e) => {
    //                     println!("Failed in opening file ({}).", e);
    //                 }
    //                 Ok(file) => {
    //                     let reader = BufReader::new(file);
    //                     let mut inst_count = 0;
    //                     for line in reader.lines() {
    //                         let input = line.unwrap();
    //                         let input: Vec<char> = input.chars().collect();
    //                         for c in input {
    //                             let inst = c as u8;
    //                             core.store_byte(inst_count, u8_to_i8(inst));
    //                             inst_count += 1;
    //                         }
    //                     }
    //                     if inst_count % 4 != 0 {
    //                         eprintln!("Reading file failed.\nThe size of sum of instructions is not a multiple of 4. {}", inst_count);
    //                     }
    //                     core.run(true, 100);
    //                 }
    //             }
    //         }
    //         _ => {
    //             let input: Vec<char> = input.chars().collect();
    //             if input.len() != 32 {
    //                 eprintln!("invalid instruction");
    //                 continue;
    //             }
    //             let mut inst: [MemoryValue; 4] = [0; 4];
    //             for i in 0..4 {
    //                 let byte: String = input[(32 - (i + 1) * 8)..(32 - i * 8)].iter().collect();
    //                 inst[i] = MemoryValue::from_str_radix(&byte, 2).unwrap();
    //             }
    //             exec_instruction(&mut core, inst, false);
    //         }
    //     }
    // }
}
