mod core;
mod decoder;
mod instruction;
mod memory;
mod register;
mod types;
use crate::core::*;
use instruction::*;
use std::{
    fs::File,
    io::{self, stdout, BufRead, BufReader, Write},
};
use types::*;

fn main() {
    let mut core = Core::new();
    core.set_int_register(2, 5000);
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop().unwrap();
        let input: &str = &input;
        match input {
            "sr" => {
                core.show_registers();
            }
            "sm" => {
                core.show_memory();
            }
            "lf" => {
                print!("file name: ");
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
                            let input: String = input.chars().collect();
                            if input.len() != 32 {
                                eprintln!("invalid instruction");
                                continue;
                            }
                            let inst = u32::from_str_radix(&input, 2).unwrap() as i32;
                            core.store_word(inst_count * 4, inst);
                            inst_count += 1;
                        }
                        core.show_memory();
                        core.run();
                    }
                }
            }
            _ => {
                let input: Vec<char> = input.chars().collect();
                if input.len() != 32 {
                    eprintln!("invalid instruction");
                    continue;
                }
                let mut inst: [MemoryValue; 4] = [0; 4];
                for i in 0..4 {
                    let byte: String = input[(32 - (i + 1) * 8)..(32 - i * 8)].iter().collect();
                    inst[i] = MemoryValue::from_str_radix(&byte, 2).unwrap();
                }
                exec_instruction(&mut core, inst);
            }
        }
    }
}
