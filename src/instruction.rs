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
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
                        );
                        core.set_int_register(rd as usize, value as Int);
                    }
                    0b001 => {
                        // lh
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("lh x{}, {}(x{})", rd, imm, rs1);
                        }
                        let value = core.load_half(
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
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
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
                        );
                        core.set_int_register(rd as usize, value as Int);
                    }
                    0b100 => {
                        // lbu
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("lbu x{}, {}(x{})", rd, imm, rs1);
                        }
                        let value = core.load_ubyte(
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
                        );
                        core.set_int_register(rd as usize, value as Int);
                    }
                    0b101 => {
                        // lhu
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("lhu x{}, {}(x{})", rd, imm, rs1);
                        }
                        let value = core.load_uhalf(
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
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
                    0b001 => {
                        // slli
                        let imm = imm & 0b11111;
                        let funct7 = (imm >> 5) & 0b1111111;
                        assert_eq!(funct7, 0);
                        if verbose {
                            println!("slli x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = core.get_int_register(rs1 as usize);
                        core.set_int_register(rd as usize, rs1_value << imm);
                    }
                    0b010 => {
                        // slti
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("slti x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = core.get_int_register(rs1 as usize);
                        if rs1_value < imm as i32 {
                            core.set_int_register(rd as usize, 1);
                        } else {
                            core.set_int_register(rd as usize, 0);
                        }
                    }
                    0b011 => {
                        // sltiu
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("sltiu x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
                        if (rs1_value as i64) < (imm as i64) {
                            core.set_int_register(rd as usize, 1);
                        } else {
                            core.set_int_register(rd as usize, 0);
                        }
                    }
                    0b100 => {
                        // xori
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("xori x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = core.get_int_register(rs1 as usize);
                        core.set_int_register(rd as usize, rs1_value ^ imm as i32);
                    }
                    0b101 => {
                        let funct7 = (imm >> 5) & 0b1111111;
                        match funct7 {
                            0b0000000 => {
                                // srli
                                let imm = imm & 0b11111;
                                if verbose {
                                    println!("srli x{}, x{}, {}", rd, rs1, imm);
                                }
                                let rs1_value = core.get_int_register(rs1 as usize);
                                core.set_int_register(
                                    rd as usize,
                                    u32_to_i32((i32_to_u32(rs1_value) >> imm)),
                                );
                            }
                            0b0100000 => {
                                // srai
                                let imm = imm & 0b11111;
                                if verbose {
                                    println!("srai x{}, x{}, {}", rd, rs1, imm);
                                }
                                let rs1_value = core.get_int_register(rs1 as usize);
                                core.set_int_register(rd as usize, rs1_value >> imm);
                            }
                            _ => {
                                println!("unexpected funct7: {}", funct7)
                            }
                        }
                    }
                    0b110 => {
                        // ori
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("ori x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = core.get_int_register(rs1 as usize);
                        core.set_int_register(rd as usize, rs1_value | imm as i32);
                    }
                    0b111 => {
                        // andi
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("andi x{}, x{}, {}", rd, rs1, imm);
                        }
                        let rs1_value = core.get_int_register(rs1 as usize);
                        core.set_int_register(rd as usize, rs1_value & imm as i32);
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
                        core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
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
                        0b0100000 => {
                            // sub
                            if verbose {
                                println!("sub x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value - rs2_value);
                        }
                        0b0000001 => {
                            // mul
                            if verbose {
                                println!("mul x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize) as i64;
                            let rs1_value = core.get_int_register(rs1 as usize) as i64;
                            core.set_int_register(
                                rd as usize,
                                ((rs1_value * rs2_value) & 0xffffffff) as i32,
                            );
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b001 => match funct7 {
                        0b0000000 => {
                            // sll
                            if verbose {
                                println!("sll x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(
                                rd as usize,
                                u32_to_i32(i32_to_u32(rs1_value) << (rs2_value & 0b11111)) as Int,
                            );
                        }
                        0b0000001 => {
                            // mulh
                            if verbose {
                                println!("mulh x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize) as i64;
                            let rs1_value = core.get_int_register(rs1 as usize) as i64;
                            core.set_int_register(
                                rd as usize,
                                (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
                            );
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b010 => match funct7 {
                        0b0000000 => {
                            // slt
                            if verbose {
                                println!("slt x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            if rs1_value < rs2_value {
                                core.set_int_register(rd as usize, 1);
                            } else {
                                core.set_int_register(rd as usize, 0);
                            }
                        }
                        0b0000001 => {
                            // mulhsu
                            if verbose {
                                println!("mulhsu x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as i64;
                            let rs1_value = core.get_int_register(rs1 as usize) as i64;
                            core.set_int_register(
                                rd as usize,
                                (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
                            );
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b011 => match funct7 {
                        0b0000000 => {
                            // sltu
                            if verbose {
                                println!("sltu x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
                            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
                            if rs1_value < rs2_value {
                                core.set_int_register(rd as usize, 1);
                            } else {
                                core.set_int_register(rd as usize, 0);
                            }
                        }
                        0b0000001 => {
                            // mulhu
                            if verbose {
                                println!("mulhu x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as u64;
                            let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize)) as u64;
                            core.set_int_register(
                                rd as usize,
                                u32_to_i32((((rs1_value * rs2_value) >> 32) & 0xffffffff) as u32),
                            );
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b100 => match funct7 {
                        0b0000000 => {
                            // xor
                            if verbose {
                                println!("xor x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value ^ rs2_value);
                        }
                        0b0000001 => {
                            // div
                            if verbose {
                                println!("div x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize) as i64;
                            let rs1_value = core.get_int_register(rs1 as usize) as i64;
                            if rs2_value == 0 {
                                core.set_int_register(rd as usize, -1);
                            } else {
                                core.set_int_register(
                                    rd as usize,
                                    ((rs1_value / rs2_value) & 0xffffffff) as i32,
                                );
                            }
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b101 => match funct7 {
                        0b0000000 => {
                            // srl
                            if verbose {
                                println!("srl x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(
                                rd as usize,
                                u32_to_i32(i32_to_u32(rs1_value) >> (rs2_value & 0b11111)),
                            );
                        }
                        0b0100000 => {
                            // sra
                            if verbose {
                                println!("sra x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value >> (rs2_value & 0b11111));
                        }
                        0b0000001 => {
                            // divu
                            if verbose {
                                println!("divu x{}, x{}, x{}", rd, rs1, rs2);
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
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b110 => match funct7 {
                        0b0000000 => {
                            // or
                            if verbose {
                                println!("or x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value | rs2_value);
                        }
                        0b0000001 => {
                            // rem
                            if verbose {
                                println!("rem x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize) as i64;
                            let rs1_value = core.get_int_register(rs1 as usize) as i64;
                            if rs2_value == 0 {
                                core.set_int_register(rd as usize, rs1_value as i32);
                            } else {
                                core.set_int_register(
                                    rd as usize,
                                    ((rs1_value % rs2_value) & 0xffffffff) as i32,
                                );
                            }
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    0b111 => match funct7 {
                        0b0000000 => {
                            // and
                            if verbose {
                                println!("and x{}, x{}, x{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_int_register(rs2 as usize);
                            let rs1_value = core.get_int_register(rs1 as usize);
                            core.set_int_register(rd as usize, rs1_value & rs2_value);
                        }
                        0b0000001 => {
                            // remu
                            if verbose {
                                println!("remu x{}, x{}, x{}", rd, rs1, rs2);
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
                        }
                        _ => {
                            println!("unexpected funct7: {}", funct7)
                        }
                    },
                    _ => {
                        println!("unexpected funct3: {}", funct3)
                    }
                },
                83 => match funct7 >> 2 {
                    0b00000 => {
                        // fadd
                        if verbose {
                            println!("fadd f{}, f{}, f{}", rd, rs1, rs2);
                        }
                        let rs2_value = core.get_float_register(rs2 as usize);
                        let rs1_value = core.get_float_register(rs1 as usize);
                        core.set_float_register(rd as usize, rs1_value + rs2_value);
                    }
                    0b00001 => {
                        // fsub
                        if verbose {
                            println!("fsub f{}, f{}, f{}", rd, rs1, rs2);
                        }
                        let rs2_value = core.get_float_register(rs2 as usize);
                        let rs1_value = core.get_float_register(rs1 as usize);
                        core.set_float_register(rd as usize, rs1_value - rs2_value);
                    }
                    0b00010 => {
                        // fmul
                        if verbose {
                            println!("fmul f{}, f{}, f{}", rd, rs1, rs2);
                        }
                        let rs2_value = core.get_float_register(rs2 as usize);
                        let rs1_value = core.get_float_register(rs1 as usize);
                        core.set_float_register(rd as usize, rs1_value * rs2_value);
                    }
                    0b00011 => {
                        // fdiv
                        if verbose {
                            println!("fadd f{}, f{}, f{}", rd, rs1, rs2);
                        }
                        let rs2_value = core.get_float_register(rs2 as usize);
                        let rs1_value = core.get_float_register(rs1 as usize);
                        core.set_float_register(rd as usize, rs1_value / rs2_value);
                    }
                    0b01011 => {
                        // fsqrt
                        if verbose {
                            println!("fsqrt f{}, f{}", rd, rs1);
                        }
                        let rs1_value = core.get_float_register(rs1 as usize);
                        core.set_float_register(rd as usize, rs1_value.sqrt());
                    }
                    0b00100 => {
                        match funct3 {
                            0b000 => {
                                // fsgnj
                                if verbose {
                                    println!("fsgnj f{}, f{}, f{}", rd, rs1, rs2);
                                }
                                let rs2_value = core.get_float_register(rs2 as usize);
                                let rs1_value = core.get_float_register(rs1 as usize);
                                let mut rd_value = rs1_value;
                                if rs2_value.is_sign_negative() {
                                    rd_value = -rd_value;
                                }
                                core.set_float_register(rd as usize, rd_value);
                            }
                            0b001 => {
                                // fsgnjn
                                if verbose {
                                    println!("fsgnj f{}, f{}, f{}", rd, rs1, rs2);
                                }
                                let rs2_value = core.get_float_register(rs2 as usize);
                                let rs1_value = core.get_float_register(rs1 as usize);
                                let mut rd_value = rs1_value;
                                if !rs2_value.is_sign_negative() {
                                    rd_value = -rd_value;
                                }
                                core.set_float_register(rd as usize, rd_value);
                            }
                            0b010 => {
                                // fsgnjx
                                if verbose {
                                    println!("fsgnj f{}, f{}, f{}", rd, rs1, rs2);
                                }
                                let rs2_value = core.get_float_register(rs2 as usize);
                                let rs1_value = core.get_float_register(rs1 as usize);
                                let mut rd_value = rs1_value;
                                if rs2_value.is_sign_negative() ^ rs1_value.is_sign_negative() {
                                    rd_value = -rd_value;
                                }
                                core.set_float_register(rd as usize, rd_value);
                            }
                            _ => {
                                println!("unexpected funct3: {}", funct3)
                            }
                        }
                    }
                    0b00101 => {
                        match funct3 {
                            0b000 => {
                                // fmin
                                if verbose {
                                    println!("fmin f{}, f{}, f{}", rd, rs1, rs2);
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
                            }
                            0b001 => {
                                // fmax
                                if verbose {
                                    println!("fmax f{}, f{}, f{}", rd, rs1, rs2);
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
                            }
                            _ => {
                                println!("unexpected funct3: {}", funct3)
                            }
                        }
                    }
                    0b10100 => match funct3 {
                        0b010 => {
                            // feq
                            if verbose {
                                println!("feq x{}, f{}, f{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_float_register(rs2 as usize);
                            let rs1_value = core.get_float_register(rs1 as usize);
                            if rs1_value == rs2_value {
                                core.set_int_register(rd as usize, 1);
                            } else {
                                core.set_int_register(rd as usize, 0);
                            }
                        }
                        0b001 => {
                            // flt
                            if verbose {
                                println!("flt x{}, f{}, f{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_float_register(rs2 as usize);
                            let rs1_value = core.get_float_register(rs1 as usize);
                            if rs1_value < rs2_value {
                                core.set_int_register(rd as usize, 1);
                            } else {
                                core.set_int_register(rd as usize, 0);
                            }
                        }
                        0b000 => {
                            // fle
                            if verbose {
                                println!("fle x{}, f{}, f{}", rd, rs1, rs2);
                            }
                            let rs2_value = core.get_float_register(rs2 as usize);
                            let rs1_value = core.get_float_register(rs1 as usize);
                            if rs1_value <= rs2_value {
                                core.set_int_register(rd as usize, 1);
                            } else {
                                core.set_int_register(rd as usize, 0);
                            }
                        }
                        _ => {
                            println!("unexpected funct3: {}", funct3)
                        }
                    },
                    0b11100 => match funct3 {
                        0b001 => {
                            // fclass
                            if verbose {
                                println!("fclass x{}, f{}", rd, rs1);
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
                        }
                        _ => {
                            println!("unexpected funct3: {}", funct3)
                        }
                    },
                    _ => {
                        println!("unexpected funct7: {}", funct7)
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
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
                            (value & 255) as Byte,
                        )
                    }
                    0b001 => {
                        // sh
                        let imm = sign_extention_i16(imm, 12);
                        if verbose {
                            println!("sh x{}, {}(x{})", rs2, imm, rs1);
                        }
                        let value = core.get_int_register(rs2 as usize);
                        core.store_half(
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
                            (value & 65535) as Half,
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
                            (imm as i32 + core.get_int_register(rs1 as usize)) as Address,
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
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                0b001 => {
                    // bne
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("bne x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if rs1_value != rs2_value {
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
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
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                0b101 => {
                    // bge
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("bge x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if rs1_value >= rs2_value {
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                0b110 => {
                    // bltu
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("bltu x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if i32_to_u32(rs1_value) < i32_to_u32(rs2_value) {
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
                        core.set_pc(new_pc as Address);
                    } else {
                        core.increment_pc();
                    }
                }
                0b111 => {
                    // bgeu
                    let imm = sign_extention_i16(imm, 12);
                    if verbose {
                        println!("bgeu x{}, x{}, {} + {}", rs2, rs1, core.get_pc(), imm << 1);
                    }
                    let rs2_value = core.get_int_register(rs2 as usize);
                    let rs1_value = core.get_int_register(rs1 as usize);
                    if i32_to_u32(rs1_value) >= i32_to_u32(rs2_value) {
                        let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
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
                core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
                core.set_pc(new_pc as Address);
            }
            _ => {
                println!("unexpected op: {}", op);
            }
        },
        Instruction::UInstruction(imm, rd, op) => {
            match op {
                23 => {
                    // auipc
                    let imm = sign_extention_i32(imm, 20);
                    if verbose {
                        println!("auipc x{}, {}", rd, imm << 12);
                    }
                    core.set_int_register(
                        rd as usize,
                        (core.get_pc() as i64 + (imm << 12) as i64) as Int,
                    );
                }
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
        Instruction::R4Instruction(fs1, _, fs2, fs3, _, fd, op) => {
            match op {
                67 => {
                    // fmadd
                    if verbose {
                        println!("fmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3);
                    }
                    let fs1_value = core.get_float_register(fs1 as usize);
                    let fs2_value = core.get_float_register(fs2 as usize);
                    let fs3_value = core.get_float_register(fs3 as usize);
                    core.set_float_register(fd as usize, fs1_value * fs2_value + fs3_value);
                }
                71 => {
                    // fmsub
                    if verbose {
                        println!("fmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3);
                    }
                    let fs1_value = core.get_float_register(fs1 as usize);
                    let fs2_value = core.get_float_register(fs2 as usize);
                    let fs3_value = core.get_float_register(fs3 as usize);
                    core.set_float_register(fd as usize, fs1_value * fs2_value - fs3_value);
                }
                75 => {
                    if verbose {
                        println!("fnmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3);
                    }
                    let fs1_value = core.get_float_register(fs1 as usize);
                    let fs2_value = core.get_float_register(fs2 as usize);
                    let fs3_value = core.get_float_register(fs3 as usize);
                    core.set_float_register(fd as usize, -(fs1_value * fs2_value + fs3_value));
                }
                79 => {
                    if verbose {
                        println!("fnmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3);
                    }
                    let fs1_value = core.get_float_register(fs1 as usize);
                    let fs2_value = core.get_float_register(fs2 as usize);
                    let fs3_value = core.get_float_register(fs3 as usize);
                    core.set_float_register(fd as usize, -(fs1_value * fs2_value + fs3_value));
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
