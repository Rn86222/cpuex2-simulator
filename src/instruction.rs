use crate::core::*;
use crate::decoder::*;
use crate::types::*;

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

pub fn exec_instruction(core: &mut Core, inst: [MemoryValue; 4], verbose: bool) {
    match decode_instruction(inst) {
        Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
            match op {
                3 => match funct3 {
                    0b000 => {
                        // lb
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("lb x{}, {}(x{})", rd, imm, rs1);
                        }
                        let value = core.load_byte(
                            (imm + core.get_int_register(rs1 as usize) as i16) as Address,
                        );
                        core.set_int_register(rd as usize, value as Int);
                    }
                    0b010 => {
                        // lw
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("lw x{}, {}(x{})", rd, imm, rs1);
                        }
                        let value = core.load_word(
                            (imm + core.get_int_register(rs1 as usize) as i16) as Address,
                        );
                        core.set_int_register(rd as usize, value as Int);
                    }
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                19 => match funct3 {
                    0b000 => {
                        // addi
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("addi x{}, x{}, {}", rd, rs1, imm);
                        }
                        let value = core.get_int_register(rs1 as usize) + imm as i32;
                        core.set_int_register(rd as usize, value);
                    }
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                103 => match funct3 {
                    0b000 => {
                        // jalr
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("jalr x{}, x{}, {}", rd, rs1, imm);
                        }
                        let new_pc = core.get_int_register(rs1 as usize) + imm as i32;
                        core.set_int_register(rd as usize, core.get_pc() as Int + 4);
                        core.set_pc(new_pc as Address);
                    }
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                _ => {
                    println!("unexpected op: {}", op);
                }
            }
            if op != 103 {
                core.increment_pc();
            }
        }
        Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
            match op {
                51 => match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => {
                            // add
                            if verbose {
                                println!("add x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value + rs2_value);
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                _ => {
                    println!("unexpected op: {}", op);
                }
            }
            core.increment_pc();
        }
        Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
            match op {
                35 => match funct3 {
                    0b000 => {
                        // sb
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("sb x{}, {}(x{})", rs2, imm, rs1);
                        }
                        let value = core.get_int_register(rs2 as usize);
                        core.store_byte(
                            (imm + core.get_int_register(rs1 as usize) as i16) as Address,
                            (value & 255) as Byte,
                        )
                    }
                    0b010 => {
                        // sw
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("sw x{}, {}(x{})", rs2, imm, rs1);
                        }
                        let value = core.get_int_register(rs2 as usize);
                        core.store_word(
                            (imm + core.get_int_register(rs1 as usize) as i16) as Address,
                            value as Word,
                        )
                    }
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                _ => {
                    println!("unexpected op: {}", op);
                }
            }
            core.increment_pc();
        }
        Instruction::BInstruction(imm, rs2, rs1, funct3, op) => match op {
            99 => match funct3 {
                0b000 => {
                    // beq
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("beq x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if rs1_value == rs2_value {
                        let new_pc = core.get_pc() as i32 + (imm << 1) as i32;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                0b100 => {
                    // blt
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("blt x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if rs1_value < rs2_value {
                        let new_pc = core.get_pc() as i32 + (imm << 1) as i32;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                _ => {
                    println!("unexpected funct3: {}", funct3)
                }
            },
            _ => {
                println!("unexpected op: {}", op);
            }
        },
        Instruction::JInstruction(imm, rd, op) => match op {
            111 => {
                // jal
                let imm = sign_extention_i32(imm, 20);
                if verbose {
                    println!("jal x{}, {} + {}", rd, core.get_pc(), imm << 1);
                }
                let new_pc = core.get_pc() as i32 + (imm << 1);
                core.set_int_register(rd as usize, (core.get_pc() + 4) as Int);
                core.set_pc(new_pc as Address);
            }
            _ => {
                println!("unexpected op: {}", op);
            }
        },
        Instruction::UInstruction(imm, rd, op) => {
            match op {
                55 => {
                    // lui
                    let imm = sign_extention_i32(imm, 20);
                    if verbose {
                        println!("lui x{}, {}", rd, imm << 12);
                    }
                    core.set_int_register(rd as usize, (imm as Int) << 12);
                }
                _ => {
                    println!("unexpected op: {}", op);
                }
            }
            core.increment_pc();
        }
        Instruction::OtherInstruction => {
            println!("other instruction");
        }
    }
}
