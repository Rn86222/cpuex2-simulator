use std::collections::HashMap;

use crate::core::*;
use crate::decoder::*;
use crate::types::*;
use crate::utils::*;

pub fn sign_extention_i16(value: i16, before_bit: usize) -> i16 {
    if (value >> (before_bit - 1)) & 1 == 0 {
        value
    } else {
        let mut extention: i16 = 0;
        for i in 0..16 - before_bit {
            extention += 1 << (before_bit + i);
        }
        value | extention
    }
}

pub fn sign_extention_i32(value: i32, before_bit: usize) -> i32 {
    if (value >> (before_bit - 1)) & 1 == 0 {
        value
    } else {
        let mut extention: i32 = 0;
        for i in 0..32 - before_bit {
            extention += 1 << (before_bit + i);
        }
        value | extention
    }
}

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

struct IInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: &'static str,
}
struct RInstructionExecutor {
    exec: fn(&mut Core, u8, u8, u8, bool),
    name: &'static str,
}
struct SInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: &'static str,
}
struct BInstructionExecutor {
    exec: fn(&mut Core, i16, u8, u8, bool),
    name: &'static str,
}
struct UInstructionExecutor {
    exec: fn(&mut Core, i32, u8, bool),
    name: &'static str,
}
struct JInstructionExecutor {
    exec: fn(&mut Core, i32, u8, bool),
    name: &'static str,
}
struct R4InstructionExecutor {
    exec: fn(&mut Core, u8, u8, u8, u8, bool),
    name: &'static str,
}

pub struct InstructionMaps {
    i_instruction_map: IInstructionMap,
    r_instruction_map: RInstructionMap,
    s_instruction_map: SInstructionMap,
    b_instruction_map: BInstructionMap,
    j_instruction_map: JInstructionMap,
    u_instruction_map: UInstructionMap,
    r4_instruction_map: R4InstructionMap,
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

fn exec_i_instruction(
    core: &mut Core,
    imm: i16,
    rs1: u8,
    funct3: u8,
    rd: u8,
    op: u8,
    verbose: bool,
) {
    let executor = core
        .get_instruction_maps()
        .get_i_instruction_map()
        .get(&(op, funct3));
    if executor.is_none() {
        println!("unexpected op: {}, funct3: {}", op, funct3);
        return;
    }
    (executor.unwrap().exec)(core, imm, rs1, rd, verbose);
}

fn exec_r_instruction(
    core: &mut Core,
    funct7: u8,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    rd: u8,
    op: u8,
    verbose: bool,
) {
    let executor = core
        .get_instruction_maps()
        .get_r_instruction_map()
        .get(&(op, funct3, funct7));
    if executor.is_none() {
        println!(
            "unexpected op: {}, funct3: {}, funct7: {}",
            op, funct3, funct7
        );
        return;
    }
    (executor.unwrap().exec)(core, rs2, rs1, rd, verbose);
}

fn exec_s_instruction(
    core: &mut Core,
    imm: i16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    op: u8,
    verbose: bool,
) {
    let executor = core
        .get_instruction_maps()
        .get_s_instruction_map()
        .get(&(op, funct3));
    if executor.is_none() {
        println!("unexpected op: {}, funct3: {}", op, funct3);
        return;
    }
    (executor.unwrap().exec)(core, imm, rs2, rs1, verbose);
}

fn exec_b_instruction(
    core: &mut Core,
    imm: i16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    op: u8,
    verbose: bool,
) {
    let executor = core
        .get_instruction_maps()
        .get_b_instruction_map()
        .get(&(op, funct3));
    if executor.is_none() {
        println!("unexpected op: {}, funct3: {}", op, funct3);
        return;
    }
    (executor.unwrap().exec)(core, imm, rs2, rs1, verbose);
}

fn exec_j_instruction(core: &mut Core, imm: i32, rd: u8, op: u8, verbose: bool) {
    let executor = core
        .get_instruction_maps()
        .get_j_instruction_map()
        .get(&(op));
    if executor.is_none() {
        println!("unexpected op: {}", op);
        return;
    }
    (executor.unwrap().exec)(core, imm, rd, verbose);
}

fn exec_u_instruction(core: &mut Core, imm: i32, rd: u8, op: u8, verbose: bool) {
    let executor = core
        .get_instruction_maps()
        .get_u_instruction_map()
        .get(&(op));
    if executor.is_none() {
        println!("unexpected op: {}", op);
        return;
    }
    (executor.unwrap().exec)(core, imm, rd, verbose);
}

fn exec_r4_instruction(core: &mut Core, fs3: u8, fs2: u8, fs1: u8, fd: u8, op: u8, verbose: bool) {
    let executor = core
        .get_instruction_maps()
        .get_r4_instruction_map()
        .get(&(op));
    if executor.is_none() {
        println!("unexpected op: {}", op);
        return;
    }
    (executor.unwrap().exec)(core, fs3, fs2, fs1, fd, verbose);
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
        },
        name: "lb",
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
        },
        name: "lh",
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
        },
        name: "lw",
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
        },
        name: "lbu",
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
        },
        name: "lhu",
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
        },
        name: "addi",
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
        },
        name: "slli",
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
        },
        name: "slti",
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
        },
        name: "sltiu",
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
        },
        name: "xori",
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
            } else if funct7 == 0b0100000 {
                // srai
                if verbose {
                    println_inst(&format!("srai x{}, x{}, {}", rd, rs1, imm));
                }
                let rs1_value = core.get_int_register(rs1 as usize);
                core.set_int_register(rd as usize, rs1_value >> imm);
            } else {
                println!("unexpected funct7: {}", funct7);
            }
        },
        name: "srli_srai",
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
        },
        name: "ori",
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
        },
        name: "andi",
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
        name: "jalr",
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
        },
        name: "flw",
    };
    map.insert((7, 0b010), flw);
    let in_ = IInstructionExecutor {
        exec: |_: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            if verbose {
                println_inst(&format!("in x{}", rs1));
            }
        },
        name: "in",
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
        },
        name: "outuart",
    };
    map.insert((115, 0b100), outuart);
    let out7seg8 = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>08x}", rs1_value);
        },
        name: "out7seg8",
    };
    map.insert((115, 0b101), out7seg8);
    let out7seg1 = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>01x}", i32_to_u32(rs1_value) & 0b1111);
        },
        name: "out7seg16",
    };
    map.insert((115, 0b110), out7seg1);
    let outled = IInstructionExecutor {
        exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
            assert_eq!(imm, 0);
            assert_eq!(rd, 0);
            let rs1_value = core.get_int_register(rs1 as usize);
            eprintln!("{:>016b}", i32_to_u32(rs1_value) & 65535);
        },
        name: "outled",
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
        },
        name: "add",
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
        },
        name: "sub",
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
        },
        name: "mul",
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
        },
        name: "absdiff",
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
        },
        name: "sll",
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
        },
        name: "mulh",
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
        },
        name: "slt",
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
        },
        name: "mulhsu",
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
        },
        name: "slty",
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
        },
        name: "mulhu",
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
        },
        name: "xor",
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
        },
        name: "div",
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
        },
        name: "srl",
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
        },
        name: "sra",
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
        },
        name: "divu",
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
        },
        name: "or",
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
        },
        name: "rem",
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
        },
        name: "and",
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
        },
        name: "remu",
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
        },
        name: "fadd",
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
        },
        name: "fsub",
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
        },
        name: "fmul",
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
        },
        name: "fdiv",
    };
    map.insert((83, 0b000, 0b0001100), fdiv);
    let fsqrt = RInstructionExecutor {
        exec: |core: &mut Core, _: u8, rs1: u8, rd: u8, verbose: bool| {
            if verbose {
                println_inst(&format!("fsqrt f{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_float_register(rd as usize, rs1_value.sqrt());
        },
        name: "fsqrt",
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
        },
        name: "fsgnj",
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
        },
        name: "fsgnjn",
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
        },
        name: "fsgnjx",
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
        },
        name: "fmin",
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
        },
        name: "fmax",
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
        },
        name: "feq",
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
        },
        name: "flt",
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
        },
        name: "fle",
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
        },
        name: "fclass",
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
        },
        name: "fcvt.w.s",
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
        },
        name: "fcvt.wu.s",
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
        },
        name: "fcvt.s.w",
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
        },
        name: "fcvt.s.wu",
    };
    map.insert((83, 0b000, 0b1101001), fcvt_s_wu);
    let fmv_x_w = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| match rs2 {
            0b00000 => {
                if verbose {
                    println_inst(&format!("fmvs.x.w f{}, x{}", rd, rs1));
                }
                let rs1_value = core.get_int_register(rs1 as usize);
                core.set_float_register(rd as usize, rs1_value as f32);
            }
            _ => {
                println!("unexpected rs2: {}", rs2)
            }
        },
        name: "fmv.x.w",
    };
    map.insert((83, 0b000, 0b1110000), fmv_x_w);
    let fmv_x_w = RInstructionExecutor {
        exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
            assert_eq!(rs2, 0b00000);
            if verbose {
                println_inst(&format!("fmv.w.x x{}, f{}", rd, rs1));
            }
            let rs1_value = core.get_float_register(rs1 as usize);
            core.set_int_register(rd as usize, rs1_value as i32);
        },
        name: "fmv.x.w",
    };
    map.insert((83, 0b000, 0b1111000), fmv_x_w);
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
        },
        name: "swapw",
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
        },
        name: "swaph",
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
        },
        name: "swapb",
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
            )
        },
        name: "sb",
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
            )
        },
        name: "sh",
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
            )
        },
        name: "sw",
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
            )
        },
        name: "fsw",
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
        name: "beq",
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
        name: "bne",
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
        name: "blt",
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
        name: "bge",
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
        name: "bltu",
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
        name: "bgeu",
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
        name: "jal",
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
        },
        name: "auipc",
    };
    map.insert(23, auipc);
    let lui = UInstructionExecutor {
        exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
            let imm = sign_extention_i32(imm, 20);
            if verbose {
                println_inst(&format!("lui x{}, {}", rd, imm));
            }
            core.set_int_register(rd as usize, (imm as Int) << 12);
        },
        name: "lui",
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
        },
        name: "fmadd",
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
        },
        name: "fmsub",
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
        },
        name: "fnmsub",
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
        },
        name: "fnmadd",
    };
    map.insert(79, fnmadd);
    map
}

pub fn exec_instruction(core: &mut Core, inst: InstructionValue, verbose: bool) {
    match decode_instruction(inst) {
        Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
            exec_i_instruction(core, imm, rs1, funct3, rd, op, verbose);
            if op != 103 {
                core.increment_pc();
            }
        }
        Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
            exec_r_instruction(core, funct7, rs2, rs1, funct3, rd, op, verbose);
            core.increment_pc();
        }
        Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
            exec_s_instruction(core, imm, rs2, rs1, funct3, op, verbose);
            core.increment_pc();
        }
        Instruction::BInstruction(imm, rs2, rs1, funct3, op) => {
            exec_b_instruction(core, imm, rs2, rs1, funct3, op, verbose);
        }
        Instruction::JInstruction(imm, rd, op) => {
            exec_j_instruction(core, imm, rd, op, verbose);
        }
        Instruction::UInstruction(imm, rd, op) => {
            exec_u_instruction(core, imm, rd, op, verbose);
            core.increment_pc();
        }
        Instruction::R4Instruction(fs1, _, fs2, fs3, _, fd, op) => {
            exec_r4_instruction(core, fs3, fs2, fs1, fd, op, verbose);
            core.increment_pc();
        }
        Instruction::OtherInstruction => {
            println!("other instruction {:>032b}", inst);
        }
    }
}
