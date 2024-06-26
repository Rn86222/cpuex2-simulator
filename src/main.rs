mod cache;
mod core;
mod decoder;
mod fpu_emulator;
mod fpu_tester;
mod instruction;
mod instruction_memory;
mod memory;
mod register;
mod sld_loader;
mod types;
mod utils;
use crate::core::*;
use crate::instruction_memory::*;
use clap::Parser;
use fpu_tester::*;
use memory::MEMORY_SIZE;
use std::fs::File;
use std::io::Read;
use types::*;
use utils::*;

/// Simulator for CPUEX-Group2 computer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input binary file
    #[arg(short, long, default_value = "main.bin")]
    bin: Option<String>,

    /// Name of sld file for raytracing
    #[arg(short, long, default_value = "./sld/contest.sld")]
    sld: Option<String>,

    /// Verbose mode
    /// If this flag is not set, the simulator won't print anything in each cycle
    /// If this flag is set to 1, the simulator will print the information about only pipeline in each cycle
    /// If this flag is set to 2, the simulator will print the information about pipeline and registers in each cycle, and save history of registers and pc
    #[arg(short, long)]
    verbose: Option<u32>,

    /// Operation name for test of FPU (fadd, fsub, fmul, fdiv, fsqrt, flt, fcvtsw, fcvtws, or all)
    #[arg(short, long)]
    test_fpu: Option<String>,

    /// Disassemble mode
    /// If this flag is set, the simulator will print the disassembled instructions
    /// The output file name is the same as the input binary file name, but the extension is changed to ".dasm"
    #[arg(short, long)]
    disassemble: bool,
}

fn main() {
    let args = Args::parse();
    let fpu = args.test_fpu;
    if let Some(operation) = fpu {
        test_fpu(&operation);
    } else {
        let mut core = Core::new();
        core.set_int_register(RA, INSTRUCTION_MEMORY_SIZE as Int);
        core.set_int_register(SP, MEMORY_SIZE as Int);
        let input = args.bin.unwrap();
        match File::open(input.clone()) {
            Err(e) => {
                println!("Failed in opening file ({}).", e);
            }
            Ok(mut file) => {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                let mut inst_count = 0;
                let mut inst = 0;
                for byte in &buf {
                    inst += (*byte as u32) << ((inst_count % 4) * 8);
                    inst_count += 1;
                    if inst_count % 4 == 0 {
                        core.store_instruction(inst_count - 4, inst);
                        inst = 0;
                    }
                }
                if inst_count % 4 != 0 {
                    panic!("Reading file failed.\nThe size of sum of instructions is not a multiple of 4. {}", inst_count);
                }
                if args.disassemble {
                    let disassemble_file_path = &input.replace(".bin", ".dasm");
                    disassemble(&buf, disassemble_file_path);
                }
                let verbose = args.verbose.unwrap_or(0);
                let interval = 0;
                // let data_file_path = &input.replace(".bin", ".data");
                let ppm_file_path = &input.replace(".bin", ".ppm");
                let sld_file_path = &args.sld.unwrap();
                let pc_file_path = &input.replace(".bin", ".pc");
                core.run(
                    verbose,
                    interval,
                    // data_file_path,
                    ppm_file_path,
                    sld_file_path,
                    pc_file_path,
                );
            }
        }
    }
}
