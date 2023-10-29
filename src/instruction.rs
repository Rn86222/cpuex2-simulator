use std::collections::HashMap;

use crate::core::*;
use crate::decoder::*;
use crate::instruction_memory::InstructionMemory;
use crate::types::*;
use crate::utils::*;

fn println_inst(text: &str) {
    println!("{}", text);
    // colorized_println(text, RED);
}

type IInstructionMap = HashMap<(u8, u8), IInstructionExecutor>;
type RInstructionMap = HashMap<(u8, u8, u8), RInstructionExecutor>;
type SInstructionMap = HashMap<(u8, u8), SInstructionExecutor>;
type BInstructionMap = HashMap<(u8, u8), BInstructionExecutor>;
type JInstructionMap = HashMap<u8, JInstructionExecutor>;
type UInstructionMap = HashMap<u8, UInstructionExecutor>;
type R4InstructionMap = HashMap<u8, R4InstructionExecutor>;

pub struct InstructionMaps {
    i_instruction_map: IInstructionMap,
    r_instruction_map: RInstructionMap,
    s_instruction_map: SInstructionMap,
    b_instruction_map: BInstructionMap,
    j_instruction_map: JInstructionMap,
    u_instruction_map: UInstructionMap,
    r4_instruction_map: R4InstructionMap,
}

#[derive(Clone)]
struct IInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: String,
}

#[derive(Clone)]
struct RInstructionExecutor {
    exec: fn(&mut Core, u8, u8, u8, bool),
    name: String,
}

#[derive(Clone)]
struct SInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: String,
}

#[derive(Clone)]
struct BInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: String,
}

#[derive(Clone)]
struct UInstructionExecutor {
    exec: fn(&mut Core, i32, u8, bool),
    name: String,
}

#[derive(Clone)]
struct JInstructionExecutor {
    exec: fn(&mut Core, i32, u8, bool),
    name: String,
}

#[derive(Clone)]
struct R4InstructionExecutor {
    exec: fn(&mut Core, u8, u8, u8, u8, bool),
    name: String,
}

pub struct InstructionExecutor {
    exec: Box<dyn Fn(&mut Core, bool)>,
    name: String,
}

impl InstructionExecutor {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_exec(&self) -> &dyn Fn(&mut Core, bool) {
        &self.exec
    }
}

pub fn create_instruction_executors(
    inst_memory: &InstructionMemory,
    inst_maps: &InstructionMaps,
) -> Vec<InstructionExecutor> {
    let mut executors = Vec::new();
    let mut count = 0;
    loop {
        let inst = inst_memory.load(count as Address);
        match decode_instruction(inst) {
            Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
                let i_inst_executor = inst_maps.get_i_instruction_map().get(&(op, funct3));
                if i_inst_executor.is_none() {
                    println!("unexpected op: {}, funct3: {}", op, funct3);
                    panic!();
                }
                let i_inst_executor = i_inst_executor.unwrap().clone();
                let name = i_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (i_inst_executor.exec)(core, imm, rs1, rd, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
                let r_inst_executor = inst_maps.get_r_instruction_map().get(&(op, funct3, funct7));
                if r_inst_executor.is_none() {
                    println!(
                        "unexpected op: {}, funct3: {}, funct7: {}",
                        op, funct3, funct7
                    );
                    panic!();
                }
                let r_inst_executor = r_inst_executor.unwrap().clone();
                let name = r_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (r_inst_executor.exec)(core, rs2, rs1, rd, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
                let s_inst_executor = inst_maps.get_s_instruction_map().get(&(op, funct3));
                if s_inst_executor.is_none() {
                    println!("unexpected op: {}, funct3: {}", op, funct3);
                    panic!();
                }
                let s_inst_executor = s_inst_executor.unwrap().clone();
                let name = s_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (s_inst_executor.exec)(core, imm, rs2, rs1, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::BInstruction(imm, rs2, rs1, funct3, op) => {
                let b_inst_executor = inst_maps.get_b_instruction_map().get(&(op, funct3));
                if b_inst_executor.is_none() {
                    println!("unexpected op: {}, funct3: {}", op, funct3);
                    panic!();
                }
                let b_inst_executor = b_inst_executor.unwrap().clone();
                let name = b_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (b_inst_executor.exec)(core, imm, rs2, rs1, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::JInstruction(imm, rd, op) => {
                let j_inst_executor = inst_maps.get_j_instruction_map().get(&(op));
                if j_inst_executor.is_none() {
                    println!("unexpected op: {}", op);
                    panic!();
                }
                let j_inst_executor = j_inst_executor.unwrap().clone();
                let name = j_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (j_inst_executor.exec)(core, imm, rd, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::UInstruction(imm, rd, op) => {
                let u_inst_executor = inst_maps.get_u_instruction_map().get(&(op));
                if u_inst_executor.is_none() {
                    println!("unexpected op: {}", op);
                    panic!();
                }
                let u_inst_executor = u_inst_executor.unwrap().clone();
                let name = u_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (u_inst_executor.exec)(core, imm, rd, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            Instruction::R4Instruction(fs3, _, fs2, fs1, _, fd, op) => {
                let r4_inst_executor = inst_maps.get_r4_instruction_map().get(&(op));
                if r4_inst_executor.is_none() {
                    println!("unexpected op: {}", op);
                    panic!();
                }
                let r4_inst_executor = r4_inst_executor.unwrap().clone();
                let name = r4_inst_executor.name;
                let executor = InstructionExecutor {
                    exec: Box::new(move |core: &mut Core, verbose: bool| {
                        (r4_inst_executor.exec)(core, fs3, fs2, fs1, fd, verbose);
                    }),
                    name,
                };
                executors.push(executor);
            }
            _ => {
                break;
            }
        }
        count += 4;
    }
    println!("{} instructions are loaded.", count / 4);
    executors
}

impl InstructionMaps {
    pub fn new() -> Self {
        InstructionMaps {
            i_instruction_map: create_i_instruction_map(),
            r_instruction_map: create_r_instruction_map(),
            s_instruction_map: create_s_instruction_map(),
            b_instruction_map: create_b_instruction_map(),
            j_instruction_map: create_j_instruction_map(),
            u_instruction_map: create_u_instruction_map(),
            r4_instruction_map: create_r4_instruction_map(),
        }
    }

    fn get_i_instruction_map(&self) -> &IInstructionMap {
        &self.i_instruction_map
    }

    fn get_r_instruction_map(&self) -> &RInstructionMap {
        &self.r_instruction_map
    }

    fn get_s_instruction_map(&self) -> &SInstructionMap {
        &self.s_instruction_map
    }

    fn get_b_instruction_map(&self) -> &BInstructionMap {
        &self.b_instruction_map
    }

    fn get_j_instruction_map(&self) -> &JInstructionMap {
        &self.j_instruction_map
    }

    fn get_u_instruction_map(&self) -> &UInstructionMap {
        &self.u_instruction_map
    }

    fn get_r4_instruction_map(&self) -> &R4InstructionMap {
        &self.r4_instruction_map
    }
}

fn create_i_instruction_map() -> IInstructionMap {
    let mut map = IInstructionMap::new();
    let lb = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("lb x{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_byte((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_int_register(rd as usize, value as Int);
            core.increment_pc();
        },
        name: "lb".to_string(),
    };
    map.insert((3, 0b000), lb);
    let lh = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("lh x{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_half((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_int_register(rd as usize, value as Int);
            core.increment_pc();
        },
        name: "lh".to_string(),
    };
    map.insert((3, 0b001), lh);
    let lw = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("lw x{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_word((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_int_register(rd as usize, value as Int);
            core.increment_pc();
        },
        name: "lw".to_string(),
    };
    map.insert((3, 0b010), lw);
    let lbu = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("lbu x{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_ubyte((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_int_register(rd as usize, value as Int);
            core.increment_pc();
        },
        name: "lbu".to_string(),
    };
    map.insert((3, 0b100), lbu);
    let lhu = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("lhu x{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_uhalf((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_int_register(rd as usize, value as Int);
            core.increment_pc();
        },
        name: "lhu".to_string(),
    };
    map.insert((3, 0b101), lhu);
    let addi = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("addi x{}, x{}, {}", rd, rs1, imm));
            }
            let value = core.get_int_register(rs1 as usize) + imm as i32;
            core.set_int_register(rd as usize, value);
            core.increment_pc();
        },
        name: "addi".to_string(),
    };
    map.insert((19, 0b000), addi);
    let slli = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = imm & 0b11111;
            let funct7 = (imm >> 5) & 0b1111111;
            assert_eq!(funct7, 0);
            if verbose {
                println_inst(&format!("slli x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value << imm);
            core.increment_pc();
        },
        name: "slli".to_string(),
    };
    map.insert((19, 0b001), slli);
    let slti = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("slti x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value < imm as i32 {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "slti".to_string(),
    };
    map.insert((19, 0b010), slti);
    let sltiu = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("sltiu x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
            if (rs1_value as i64) < (imm as i64) {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "sltiu".to_string(),
    };
    map.insert((19, 0b011), sltiu);
    let xori = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("xori x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value ^ imm as i32);
            core.increment_pc();
        },
        name: "xori".to_string(),
    };
    map.insert((19, 0b100), xori);
    let srli_srai = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let funct7 = (imm >> 5) & 0b1111111;
            let imm = imm & 0b11111;
            if funct7 == 0b0000000 {
                // srli
                if verbose {
                    println_inst(&format!("srli x{}, x{}, {}", rd, rs1, imm));
                }
                let rs1_value = core.get_int_register(rs1 as usize);
                core.set_int_register(rd as usize, u32_to_i32(i32_to_u32(rs1_value) >> imm));
                core.increment_pc();
            } else if funct7 == 0b0100000 {
                // srai
                if verbose {
                    println_inst(&format!("srai x{}, x{}, {}", rd, rs1, imm));
                }
                let rs1_value = core.get_int_register(rs1 as usize);
                core.set_int_register(rd as usize, rs1_value >> imm);
                core.increment_pc();
            } else {
                println!("unexpected funct7: {}", funct7);
            }
        },
        name: "srli_srai".to_string(),
    };
    map.insert((19, 0b101), srli_srai);
    let ori = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("ori x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value | imm as i32);
            core.increment_pc();
        },
        name: "ori".to_string(),
    };
    map.insert((19, 0b110), ori);
    let andi = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("andi x{}, x{}, {}", rd, rs1, imm));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value & imm as i32);
            core.increment_pc();
        },
        name: "andi".to_string(),
    };
    map.insert((19, 0b111), andi);
    let jalr = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("jalr x{}, x{}, {}", rd, rs1, imm));
            }
            let new_pc = core.get_int_register(rs1 as usize) + imm as i32;
            core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
            core.set_pc(new_pc as Address);
        },
        name: "jalr".to_string(),
    };
    map.insert((103, 0b000), jalr);
    let flw = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("flw f{}, {}(x{})", rd, imm, rs1));
            }
            let value = core
                .load_word((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
            core.set_float_register(rd as usize, f32::from_bits(i32_to_u32(value)));
            core.increment_pc();
        },
        name: "flw".to_string(),
    };
    map.insert((7, 0b010), flw);
    let in_ = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            if verbose {
                println_inst(&format!("in x{}", rs1));
            }
            core.increment_pc();
        },
        name: "in".to_string(),
    };
    map.insert((115, 0b000), in_);
    let outuart = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            if verbose {
                println_inst(&format!("outuart x{}", rs1));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>08x}", rs1_value);
            core.increment_pc();
        },
        name: "outuart".to_string(),
    };
    map.insert((115, 0b100), outuart);
    let out7seg8 = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>08x}", rs1_value);
            core.increment_pc();
        },
        name: "out7seg8".to_string(),
    };
    map.insert((115, 0b101), out7seg8);
    let out7seg1 = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>01x}", i32_to_u32(rs1_value) & 0b1111);
            core.increment_pc();
        },
        name: "out7seg1".to_string(),
    };
    map.insert((115, 0b110), out7seg1);
    let outled = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>016b}", i32_to_u32(rs1_value) & 65535);
            core.increment_pc();
        },
        name: "outled".to_string(),
    };
    map.insert((115, 0b111), outled);
    map
}

fn create_r_instruction_map() -> RInstructionMap {
    let mut map = RInstructionMap::new();
    let add = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("add x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value + rs2_value);
            core.increment_pc();
        },
        name: "add".to_string(),
    };
    map.insert((51, 0b000, 0b0000000), add);
    let sub = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("sub x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value - rs2_value);
            core.increment_pc();
        },
        name: "sub".to_string(),
    };
    map.insert((51, 0b000, 0b0100000), sub);
    let mul = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("mul x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) as i64;
            let rs1_value = core.get_int_register(rs1 as usize) as i64;
            core.set_int_register(rd as usize, ((rs1_value * rs2_value) & 0xffffffff) as i32);
            core.increment_pc();
        },
        name: "mul".to_string(),
    };
    map.insert((51, 0b000, 0b0000001), mul);
    let absdiff = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("absdiff x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value > rs2_value {
                core.set_int_register(rd as usize, rs1_value - rs2_value);
            } else {
                core.set_int_register(rd as usize, rs2_value - rs1_value);
            }
            core.increment_pc();
        },
        name: "absdiff".to_string(),
    };
    map.insert((51, 0b000, 0b0110000), absdiff);
    let sll = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("sll x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(
                rd as usize,
                u32_to_i32(i32_to_u32(rs1_value) << (rs2_value & 0b11111)) as Int,
            );
            core.increment_pc();
        },
        name: "sll".to_string(),
    };
    map.insert((51, 0b001, 0b0000000), sll);
    let mulh = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("mulh x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) as i64;
            let rs1_value = core.get_int_register(rs1 as usize) as i64;
            core.set_int_register(
                rd as usize,
                (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
            );
            core.increment_pc();
        },
        name: "mulh".to_string(),
    };
    map.insert((51, 0b001, 0b0000001), mulh);
    let slt = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("slt x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value < rs2_value {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "slt".to_string(),
    };
    map.insert((51, 0b010, 0b0000000), slt);
    let mulhsu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("mulhsu x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as i64;
            let rs1_value = core.get_int_register(rs1 as usize) as i64;
            core.set_int_register(
                rd as usize,
                (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
            );
            core.increment_pc();
        },
        name: "mulhsu".to_string(),
    };
    map.insert((51, 0b010, 0b0000001), mulhsu);
    let sltu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("sltu x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
            if rs1_value < rs2_value {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "slty".to_string(),
    };
    map.insert((51, 0b011, 0b0000000), sltu);
    let mulhu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("mulhu x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as u64;
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize)) as u64;
            core.set_int_register(
                rd as usize,
                u32_to_i32((((rs1_value * rs2_value) >> 32) & 0xffffffff) as u32),
            );
            core.increment_pc();
        },
        name: "mulhu".to_string(),
    };
    map.insert((51, 0b011, 0b0000001), mulhu);
    let xor = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("xor x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value ^ rs2_value);
            core.increment_pc();
        },
        name: "xor".to_string(),
    };
    map.insert((51, 0b100, 0b0000000), xor);
    let div = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("div x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) as i64;
            let rs1_value = core.get_int_register(rs1 as usize) as i64;
            if rs2_value == 0 {
                core.set_int_register(rd as usize, -1);
            } else {
                core.set_int_register(rd as usize, ((rs1_value / rs2_value) & 0xffffffff) as i32);
            }
            core.increment_pc();
        },
        name: "div".to_string(),
    };
    map.insert((51, 0b100, 0b0000001), div);
    let srl = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("srl x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(
                rd as usize,
                u32_to_i32(i32_to_u32(rs1_value) >> (rs2_value & 0b11111)),
            );
            core.increment_pc();
        },
        name: "srl".to_string(),
    };
    map.insert((51, 0b101, 0b0000000), srl);
    let sra = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("sra x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value >> (rs2_value & 0b11111));
            core.increment_pc();
        },
        name: "sra".to_string(),
    };
    map.insert((51, 0b101, 0b0100000), sra);
    let divu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("divu x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
            if rs2_value == 0 {
                core.set_int_register(rd as usize, -1);
            } else {
                core.set_int_register(
                    rd as usize,
                    u32_to_i32((rs1_value / rs2_value) & 0xffffffff),
                );
            }
            core.increment_pc();
        },
        name: "divu".to_string(),
    };
    map.insert((51, 0b101, 0b0000001), divu);
    let or = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("or x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value | rs2_value);
            core.increment_pc();
        },
        name: "or".to_string(),
    };
    map.insert((51, 0b110, 0b0000000), or);
    let rem = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("rem x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) as i64;
            let rs1_value = core.get_int_register(rs1 as usize) as i64;
            if rs2_value == 0 {
                core.set_int_register(rd as usize, rs1_value as i32);
            } else {
                core.set_int_register(rd as usize, ((rs1_value % rs2_value) & 0xffffffff) as i32);
            }
            core.increment_pc();
        },
        name: "rem".to_string(),
    };
    map.insert((51, 0b110, 0b0000001), rem);
    let and = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("and x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value & rs2_value);
            core.increment_pc();
        },
        name: "and".to_string(),
    };
    map.insert((51, 0b111, 0b0000000), and);
    let remu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("remu x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
            if rs2_value == 0 {
                core.set_int_register(rd as usize, u32_to_i32(rs1_value));
            } else {
                core.set_int_register(
                    rd as usize,
                    u32_to_i32((rs1_value % rs2_value) & 0xffffffff),
                );
            }
            core.increment_pc();
        },
        name: "remu".to_string(),
    };
    map.insert((51, 0b111, 0b0000001), remu);
    let fadd = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fadd f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value + rs2_value);
            core.increment_pc();
        },
        name: "fadd".to_string(),
    };
    map.insert((83, 0b000, 0b0000000), fadd);
    let fsub = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsub f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value - rs2_value);
            core.increment_pc();
        },
        name: "fsub".to_string(),
    };
    map.insert((83, 0b000, 0b0000100), fsub);
    let fmul = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fmul f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value * rs2_value);
            core.increment_pc();
        },
        name: "fmul".to_string(),
    };
    map.insert((83, 0b000, 0b0001000), fmul);
    let fdiv = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fadd f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value / rs2_value);
            core.increment_pc();
        },
        name: "fdiv".to_string(),
    };
    map.insert((83, 0b000, 0b0001100), fdiv);
    let fsqrt = RInstructionExecutor {
        exec: |core: &mut Core, _: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsqrt f{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value.sqrt());
            core.increment_pc();
        },
        name: "fsqrt".to_string(),
    };
    map.insert((83, 0b000, 0b0101100), fsqrt);
    let fsgnj = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            let mut rd_value = rs1_value;
            if rs2_value.is_sign_negative() {
                rd_value = -rd_value;
            }
            core.set_float_register(rd as usize, rd_value);
            core.increment_pc();
        },
        name: "fsgnj".to_string(),
    };
    map.insert((83, 0b000, 0b0010000), fsgnj);
    let fsgnjn = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            let mut rd_value = rs1_value;
            if !rs2_value.is_sign_negative() {
                rd_value = -rd_value;
            }
            core.set_float_register(rd as usize, rd_value);
            core.increment_pc();
        },
        name: "fsgnjn".to_string(),
    };
    map.insert((83, 0b001, 0b0010000), fsgnjn);
    let fsgnjx = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            let mut rd_value = rs1_value;
            if rs2_value.is_sign_negative() ^ rs1_value.is_sign_negative() {
                rd_value = -rd_value;
            }
            core.set_float_register(rd as usize, rd_value);
            core.increment_pc();
        },
        name: "fsgnjx".to_string(),
    };
    map.insert((83, 0b010, 0b0010000), fsgnjx);
    let fmin = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fmin f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            if rs1_value.is_nan() {
                core.set_float_register(rd as usize, rs1_value);
            } else if rs2_value.is_nan() {
                core.set_float_register(rd as usize, rs2_value);
            } else if rs1_value < rs2_value {
                core.set_float_register(rd as usize, rs1_value);
            } else {
                core.set_float_register(rd as usize, rs2_value);
            }
            core.increment_pc();
        },
        name: "fmin".to_string(),
    };
    map.insert((83, 0b000, 0b0010100), fmin);
    let fmax = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fmax f{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            if rs1_value.is_nan() {
                core.set_float_register(rd as usize, rs1_value);
            } else if rs2_value.is_nan() {
                core.set_float_register(rd as usize, rs2_value);
            } else if rs1_value > rs2_value {
                core.set_float_register(rd as usize, rs1_value);
            } else {
                core.set_float_register(rd as usize, rs2_value);
            }
            core.increment_pc();
        },
        name: "fmax".to_string(),
    };
    map.insert((83, 0b001, 0b0010100), fmax);
    let feq = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            // feq
            if verbose {
                println_inst(&format!("feq x{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            if rs1_value == rs2_value {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "feq".to_string(),
    };
    map.insert((83, 0b010, 0b1010000), feq);
    let flt = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("flt x{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            if rs1_value < rs2_value {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "flt".to_string(),
    };
    map.insert((83, 0b001, 0b1010000), flt);
    let fle = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fle x{}, f{}, f{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_float_register(rs2 as usize);
            let rs1_value = core.get_float_register(rs1 as usize);
            if rs1_value <= rs2_value {
                core.set_int_register(rd as usize, 1);
            } else {
                core.set_int_register(rd as usize, 0);
            }
            core.increment_pc();
        },
        name: "fle".to_string(),
    };
    map.insert((83, 0b000, 0b1010000), fle);
    let fclass = RInstructionExecutor {
        exec: |core: &mut Core, _: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fclass x{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            let mut rd_value = 0;
            if rs1_value.is_nan() {
                if rd_value >> 30 & 1 == 0 {
                    rd_value |= 0b0100000000; // signaling nan
                } else {
                    rd_value |= 0b1000000000; // quiet nan
                }
            } else if rs1_value == 0. {
                if rs1_value.is_sign_negative() {
                    rd_value |= 0b0000001000;
                } else {
                    rd_value |= 0b0000010000;
                }
            } else if rs1_value.is_infinite() {
                if rs1_value.is_sign_negative() {
                    rd_value |= 0b0000000001;
                } else {
                    rd_value |= 0b0010000000;
                }
            } else if rs1_value.is_normal() {
                if rs1_value.is_sign_negative() {
                    rd_value |= 0b0000000010;
                } else {
                    rd_value |= 0b0001000000;
                }
            } else {
                if rs1_value.is_sign_negative() {
                    rd_value |= 0b0000000100;
                } else {
                    rd_value |= 0b0000100000;
                }
            }
            core.set_int_register(rd as usize, rd_value);
            core.increment_pc();
        },
        name: "fclass".to_string(),
    };
    map.insert((83, 0b001, 0b1110000), fclass);

    let fcvt_w_s = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fcvt.wu.s x{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value as i32);
            core.increment_pc();
        },
        name: "fcvt.w.s".to_string(),
    };
    map.insert((83, 0b000, 0b1100000), fcvt_w_s);

    let fcvt_wu_s = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fcvt.wu.s x{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value.abs() as i32);
            core.increment_pc();
        },
        name: "fcvt.wu.s".to_string(),
    };
    map.insert((83, 0b000, 0b1100001), fcvt_wu_s);
    let fcvt_s_w = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fcvt.s.w f{}, x{}", rd, rs1));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value as f32);
            core.increment_pc();
        },
        name: "fcvt.s.w".to_string(),
    };
    map.insert((83, 0b000, 0b1101000), fcvt_s_w);
    let fcvt_s_wu = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fcvt.s.wu f{}, x{}", rd, rs1));
            }
            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
            core.set_float_register(rd as usize, rs1_value as f32);
            core.increment_pc();
        },
        name: "fcvt.s.wu".to_string(),
    };
    map.insert((83, 0b000, 0b1101001), fcvt_s_wu);
    let fmv_x_w = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);

            if verbose {
                println_inst(&format!("fmvs.x.w f{}, x{}", rd, rs1));
            }
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value as f32);
            core.increment_pc();
        },
        name: "fmv.x.w".to_string(),
    };
    map.insert((83, 0b000, 0b1110000), fmv_x_w);
    let fmv_w_x = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fmv.w.x x{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value as i32);
            core.increment_pc();
        },
        name: "fmv.w.x".to_string(),
    };
    map.insert((83, 0b000, 0b1111000), fmv_w_x);
    let swapw = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("swapw x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            core.set_int_register(rd as usize, rs2_value);
            core.set_int_register(rs2 as usize, rs1_value);
            core.set_int_register(rs1 as usize, rs2_value);
            core.increment_pc();
        },
        name: "swapw".to_string(),
    };
    map.insert((52, 0b000, 0b0000000), swapw);
    let swaph = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("swaph x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) & 0xffff;
            let rs1_value = core.get_int_register(rs1 as usize) & 0xffff;
            core.set_int_register(rd as usize, rs2_value);
            core.set_int_register(rs2 as usize, rs1_value);
            core.set_int_register(rs1 as usize, rs2_value);
            core.increment_pc();
        },
        name: "swaph".to_string(),
    };
    map.insert((52, 0b001, 0b0000000), swaph);
    let swapb = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("swapb x{}, x{}, x{}", rd, rs1, rs2));
            }
            let rs2_value = core.get_int_register(rs2 as usize) & 0xff;
            let rs1_value = core.get_int_register(rs1 as usize) & 0xff;
            core.set_int_register(rd as usize, rs2_value);
            core.set_int_register(rs2 as usize, rs1_value);
            core.set_int_register(rs1 as usize, rs2_value);
            core.increment_pc();
        },
        name: "swapb".to_string(),
    };
    map.insert((52, 0b010, 0b0000000), swapb);
    map
}

fn create_s_instruction_map() -> SInstructionMap {
    let mut map = SInstructionMap::new();
    let sb = SInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("sb x{}, {}(x{})", rs2, imm, rs1));
            }
            let value = core.get_int_register(rs2 as usize);
            core.store_byte(
                (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
                (value & 255) as Byte,
            );
            core.increment_pc();
        },
        name: "sb".to_string(),
    };
    map.insert((35, 0b000), sb);
    let sh = SInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("sh x{}, {}(x{})", rs2, imm, rs1));
            }
            let value = core.get_int_register(rs2 as usize);
            core.store_half(
                (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
                (value & 65535) as Half,
            );
            core.increment_pc();
        },
        name: "sh".to_string(),
    };
    map.insert((35, 0b001), sh);
    let sw = SInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("sw x{}, {}(x{})", rs2, imm, rs1));
            }
            let value = core.get_int_register(rs2 as usize);
            core.store_word(
                (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
                value as Word,
            );
            core.increment_pc();
        },
        name: "sw".to_string(),
    };
    map.insert((35, 0b010), sw);
    let fsw = SInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!("fsw f{}, {}(x{})", rs2, imm, rs1));
            }
            let value = core.get_float_register(rs2 as usize);
            core.store_word(
                (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
                value.to_bits() as Word,
            );
            core.increment_pc();
        },
        name: "fsw".to_string(),
    };
    map.insert((39, 0b010), fsw);
    map
}

fn create_b_instruction_map() -> BInstructionMap {
    let mut map = BInstructionMap::new();
    let beq = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "beq x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value == rs2_value {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "beq".to_string(),
    };
    map.insert((99, 0b000), beq);
    let bne = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "bne x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value != rs2_value {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "bne".to_string(),
    };
    map.insert((99, 0b001), bne);
    let blt = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "blt x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value < rs2_value {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "blt".to_string(),
    };
    map.insert((99, 0b100), blt);
    let bge = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "bge x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if rs1_value >= rs2_value {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "bge".to_string(),
    };
    map.insert((99, 0b101), bge);
    let bltu = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "bltu x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if i32_to_u32(rs1_value) < i32_to_u32(rs2_value) {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "bltu".to_string(),
    };
    map.insert((99, 0b110), bltu);
    let bgeu = BInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
            let imm = sign_extention_i16(imm, 12);
            if verbose {
                println_inst(&format!(
                    "bgeu x{}, x{}, {} + {}",
                    rs2,
                    rs1,
                    core.get_pc(),
                    imm << 1,
                ));
            }
            let rs2_value = core.get_int_register(rs2 as usize);
            let rs1_value = core.get_int_register(rs1 as usize);
            if i32_to_u32(rs1_value) >= i32_to_u32(rs2_value) {
                let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                core.set_pc(new_pc as Address);
            } else {
                core.increment_pc();
            }
        },
        name: "bgeu".to_string(),
    };
    map.insert((99, 0b111), bgeu);
    map
}

fn create_j_instruction_map() -> JInstructionMap {
    let mut map = JInstructionMap::new();
    let jal = JInstructionExecutor {
        exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
            let imm = sign_extention_i32(imm, 20);
            if verbose {
                println_inst(&format!("jal x{}, {} + {}", rd, core.get_pc(), imm << 1));
            }
            let new_pc = core.get_pc() as i32 + (imm << 1);
            core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
            core.set_pc(new_pc as Address);
        },
        name: "jal".to_string(),
    };
    map.insert(111, jal);
    map
}

fn create_u_instruction_map() -> UInstructionMap {
    let mut map = UInstructionMap::new();
    let auipc = UInstructionExecutor {
        exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
            let imm = sign_extention_i32(imm, 20);
            if verbose {
                println_inst(&format!("auipc x{}, {}", rd, imm << 12));
            }
            core.set_int_register(
                rd as usize,
                (core.get_pc() as i64 + (imm << 12) as i64) as Int,
            );
            core.increment_pc();
        },
        name: "auipc".to_string(),
    };
    map.insert(23, auipc);
    let lui = UInstructionExecutor {
        exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
            let imm = sign_extention_i32(imm, 20);
            if verbose {
                println_inst(&format!("lui x{}, {}", rd, imm));
            }
            core.set_int_register(rd as usize, (imm as Int) << 12);
            core.increment_pc();
        },
        name: "lui".to_string(),
    };
    map.insert(55, lui);
    map
}

fn create_r4_instruction_map() -> R4InstructionMap {
    let mut map = R4InstructionMap::new();
    let fmadd = R4InstructionExecutor {
        exec: |core: &mut Core, fs3: u8, fs2: u8, fs1: u8, fd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
            }
            let fs1_value = core.get_float_register(fs1 as usize);
            let fs2_value = core.get_float_register(fs2 as usize);
            let fs3_value = core.get_float_register(fs3 as usize);
            core.set_float_register(fd as usize, fs1_value * fs2_value + fs3_value);
            core.increment_pc();
        },
        name: "fmadd".to_string(),
    };
    map.insert(67, fmadd);
    let fmsub = R4InstructionExecutor {
        exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
            }
            let fs1_value = core.get_float_register(fs1 as usize);
            let fs2_value = core.get_float_register(fs2 as usize);
            let fs3_value = core.get_float_register(fs3 as usize);
            core.set_float_register(fd as usize, fs1_value * fs2_value - fs3_value);
            core.increment_pc();
        },
        name: "fmsub".to_string(),
    };
    map.insert(71, fmsub);
    let fnmsub = R4InstructionExecutor {
        exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fnmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
            }
            let fs1_value = core.get_float_register(fs1 as usize);
            let fs2_value = core.get_float_register(fs2 as usize);
            let fs3_value = core.get_float_register(fs3 as usize);
            core.set_float_register(fd as usize, -(fs1_value * fs2_value) - fs3_value);
            core.increment_pc();
        },
        name: "fnmsub".to_string(),
    };
    map.insert(75, fnmsub);
    let fnmadd = R4InstructionExecutor {
        exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fnmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
            }
            let fs1_value = core.get_float_register(fs1 as usize);
            let fs2_value = core.get_float_register(fs2 as usize);
            let fs3_value = core.get_float_register(fs3 as usize);
            core.set_float_register(fd as usize, -(fs1_value * fs2_value - fs3_value));
            core.increment_pc();
        },
        name: "fnmadd".to_string(),
    };
    map.insert(79, fnmadd);
    map
}
