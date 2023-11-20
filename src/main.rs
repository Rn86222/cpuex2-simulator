mod cache;
mod core;
mod decoder;
mod instruction;
// mod instruction_cache;
mod fpu;
mod instruction_memory;
mod memory;
mod register;
mod types;
mod utils;
use crate::core::*;
use crate::instruction_memory::*;
use std::io::Read;
// use instruction::*;
use std::fs::File;
use types::*;
// use utils::*;
use clap::Parser;

/// Simulator for CPUEX-Group2 computer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long, default_value = "main.bin")]
    file: Option<String>,
}

fn main() {
    let mut core = Core::new();
    core.set_int_register(1, INSTRUCTION_MEMORY_SIZE as Int);
    core.set_int_register(2, 10000000);
    let args = Args::parse();
    let input = args.file.unwrap();
    match File::open(input) {
        Err(e) => {
            println!("Failed in opening file ({}).", e);
        }
        Ok(mut file) => {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let mut inst_count = 0;
            let mut inst = 0;
            for byte in buf {
                inst += ((byte as u32) << (inst_count % 4) * 8) as u32;
                inst_count += 1;
                if inst_count % 4 == 0 {
                    core.store_instruction(inst_count - 4, inst);
                    inst = 0;
                }
            }
            if inst_count % 4 != 0 {
                eprintln!("Reading file failed.\nThe size of sum of instructions is not a multiple of 4. {}", inst_count);
            }
            core.run(false, 0);
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
