use crate::types::*;

// pub struct IInstruction {
//     imm: i16,   // 12
//     rs1: u8,    // 5
//     funct3: u8, // 3
//     rd: u8,     // 5
//     op: u8,     // 7
// }

// pub struct RInstruction {
//     funct7: u8, // 7
//     rs2: u8,    // 5
//     rs1: u8,    // 5
//     funct3: u8, // 3
//     rd: u8,     // 5
//     op: u8,     // 7
// }

// pub struct SInstruction {
//     imm: i16,   // 12
//     rs2: u8,    // 5
//     rs1: u8,    // 5
//     funct3: u8, // 3
//     op: u8,     // 7
// }

// pub struct JInstruction {
//     imm: i32, // 20
//     rd: u8,   // 5
//     op: u8,   // 7
// }

// pub struct BInstruction {
//     imm: i16,   // 12
//     rs2: u8,    // 5
//     rs1: u8,    // 5
//     funct3: u8, // 3
//     op: u8,     // 7
// }

// pub struct UInstruction {
//     imm: i32, // 20
//     rd: u8,   // 5
//     op: u8,   // 7
// }

pub enum Instruction {
    IInstruction(i16, u8, u8, u8, u8),
    RInstruction(u8, u8, u8, u8, u8, u8),
    SInstruction(i16, u8, u8, u8, u8),
    JInstruction(i32, u8, u8),
    BInstruction(i16, u8, u8, u8, u8),
    UInstruction(i32, u8, u8),
    OtherInstruction,
}

enum InstructionType {
    I,
    R,
    S,
    J,
    B,
    U,
    Other,
}

fn instruction_typeof(inst: [MemoryValue; 4]) -> InstructionType {
    let op = inst[0] & 127;
    match op {
        3 | 19 | 27 | 103 => InstructionType::I,
        51 | 59 | 83 => InstructionType::R,
        35 => InstructionType::S,
        111 => InstructionType::J,
        99 => InstructionType::B,
        23 | 55 => InstructionType::U,
        _ => InstructionType::Other,
    }
}

fn decode_i_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let imm: i16 = ((inst[3] as i16) << 4) + (((inst[2] as i16) >> 4) & 0xf);
    // let imm_string = format!("{:>08b}{:>04b}", inst[3], inst[2] >> 4);
    let rs1 = ((inst[2] & 15) << 1) + (inst[1] >> 7);
    let funct3 = (inst[1] & 127) >> 4;
    let rd = ((inst[1] & 15) << 1) + (inst[0] >> 7);
    let op = inst[0] & 127;
    Instruction::IInstruction(imm, rs1, funct3, rd, op)
}

fn decode_r_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let funct7 = inst[3] >> 1;
    let rs2 = ((inst[3] & 1) << 4) + (inst[2] >> 4);
    let rs1 = ((inst[2] & 15) << 1) + (inst[1] >> 7);
    let funct3 = (inst[1] & 127) >> 4;
    let rd = ((inst[1] & 15) << 1) + (inst[0] >> 7);
    let op = inst[0] & 127;
    Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op)
}

fn decode_s_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let imm: i16 =
        ((inst[3] as i16 >> 1) << 5) + ((inst[1] as i16 & 15) << 1) + ((inst[0] as i16 >> 7) & 1);
    // let imm_string = format!(
    //     "{:>07b}{:>04b}{:>01b}",
    //     inst[3] >> 1,
    //     inst[1] & 15,
    //     inst[0] >> 7
    // );
    let rs2 = ((inst[3] & 1) << 4) + (inst[2] >> 4);
    let rs1 = ((inst[2] & 15) << 1) + (inst[1] >> 7);
    let funct3 = (inst[1] & 127) >> 4;
    let op = inst[0] & 127;
    Instruction::SInstruction(imm, rs2, rs1, funct3, op)
}

fn decode_j_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let imm: i32 = ((inst[3] as i32 >> 7) << 19)
        + ((inst[2] as i32 & 15) << 15)
        + ((inst[1] as i32 >> 4) << 11)
        + (((inst[2] as i32 >> 4) & 1) << 10)
        + ((inst[3] as i32 & 127) << 3)
        + (inst[2] as i32 >> 5);
    // let imm_string = format!(
    //     "{:>01b}{:>04b}{:>04b}{:>01b}{:>07b}{:>03b}",
    //     inst[3] >> 7,
    //     inst[2] & 15,
    //     inst[1] >> 4,
    //     (inst[2] >> 4) & 1,
    //     inst[3] & 127,
    //     inst[2] >> 5
    // );
    let rd = ((inst[1] & 15) << 1) + (inst[0] >> 7);
    let op = inst[0] & 127;
    Instruction::JInstruction(imm, rd, op)
}

fn decode_b_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let imm: i16 = ((inst[3] as i16 >> 7) << 11)
        + ((inst[3] as i16 & 126) << 3)
        + (inst[1] as i16 & 15)
        + ((inst[0] as i16 >> 7) << 10);
    // let imm_string = format!(
    //     "{:>01b}{:>01b}{:>06b}{:>04b}",
    //     inst[3] >> 7,
    //     inst[0] >> 7,
    //     (inst[3] & 126) >> 1,
    //     inst[1] & 15
    // );
    let rs2 = ((inst[3] & 1) << 4) + (inst[2] >> 4);
    let rs1 = ((inst[2] & 15) << 1) + (inst[1] >> 7);
    let funct3 = (inst[1] & 127) >> 4;
    let op = inst[0] & 127;
    Instruction::BInstruction(imm, rs2, rs1, funct3, op)
}

fn decode_u_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let imm: i32 = ((inst[3] as i32) << 12) + ((inst[2] as i32) << 4) + ((inst[1] as i32) >> 4);
    // let imm_string = format!("{:>08b}{:>08b}{:>04b}", inst[3], inst[2], inst[1] >> 4);
    let rd = ((inst[1] & 15) << 1) + (inst[0] >> 7);
    let op = inst[0] & 127;
    Instruction::UInstruction(imm, rd, op)
}

pub fn decode_instruction(inst: [MemoryValue; 4]) -> Instruction {
    let instruction_type = instruction_typeof(inst);
    match instruction_type {
        InstructionType::I => decode_i_instruction(inst),
        InstructionType::R => decode_r_instruction(inst),
        InstructionType::S => decode_s_instruction(inst),
        InstructionType::J => decode_j_instruction(inst),
        InstructionType::B => decode_b_instruction(inst),
        InstructionType::U => decode_u_instruction(inst),
        InstructionType::Other => Instruction::OtherInstruction,
    }
}
