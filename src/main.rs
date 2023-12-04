mod cache;
mod core;
mod decoder;
mod fpu_emulator;
mod fpu_tester;
mod instruction;
mod instruction_memory;
mod memory;
mod register;
mod types;
mod utils;
use crate::core::*;
use crate::instruction_memory::*;
use clap::Parser;
use fpu_tester::*;
use std::fs::File;
use std::io::Read;
use types::*;

/// Simulator for CPUEX-Group2 computer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[arg(short, long, default_value = "main.bin")]
    file: Option<String>,

    /// Operation name for test of FPU (fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, or fcvtws)
    #[arg(short, long)]
    test_fpu: Option<String>,
}

fn main() {
    let args = Args::parse();
    let fpu = args.test_fpu;
    if fpu.is_some() {
        let operation: &str = &fpu.unwrap();
        test_fpu(operation);
        return;
    }
    let mut core = Core::new();
    core.set_int_register(1, INSTRUCTION_MEMORY_SIZE as Int);
    core.set_int_register(2, 10000000);
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
}
