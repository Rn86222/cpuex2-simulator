use std::fmt::Debug;

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

// fn println_inst(text: &str) {
//     println!("{}", text);
//     colorized_println(text, RED);
// }

// #[derive(Debug, Clone, Copy)]
// pub enum ExecResult {
//     I32(i32),
//     F32(f32),
//     None,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum LoadValue {
//     Byte(Byte),
//     UByte(UByte),
//     Half(Half),
//     UHalf(UHalf),
//     Word(Word),
//     None,
// }

pub trait InstructionTrait: Clone + Debug {
    fn register_fetch(&mut self, _: &Core) {}
    fn exec(&mut self, _: &mut Core) {}
    fn memory(&mut self, _: &mut Core) {}
    fn write_back(&self, _: &mut Core) {}
    fn get_source_registers(&self) -> Vec<Rs> {
        vec![]
    }
    fn get_destination_register(&self) -> Option<Rd> {
        None
    }
    fn is_load_instruction(&self) -> bool {
        false
    }
    fn is_branch_instruction(&self) -> bool {
        false
    }
    fn get_jump_address(&self) -> Option<Address> {
        None
    }
    fn get_instruction_count(&self) -> Option<InstructionCount>;
    fn get_name(&self) -> String {
        "".to_string()
    }
}

#[derive(Clone)]
struct IInstructionData {
    imm: Imm12,
    rs1: Rs1,
    rd: Rd,
    extended_imm: Option<i32>,
    rs1_value: Option<Int>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct SInstructionData {
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    extended_imm: Option<i32>,
    rs2_value: Option<Int>,
    rs1_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct RInstructionData {
    rs2: Rs2,
    rs1: Rs1,
    rd: Rd,
    rs2_value: Option<Int>,
    rs1_value: Option<Int>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct BInstructionData {
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    extended_imm: Option<i32>,
    rs2_value: Option<Int>,
    rs1_value: Option<Int>,
    inst_count: Option<InstructionCount>,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

#[derive(Clone)]
struct JInstructionData {
    imm: Imm20,
    rd: Rd,
    extended_imm: Option<i32>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

#[derive(Clone)]
struct UInstructionData {
    imm: Imm20,
    rd: Rd,
    upimm: Option<i32>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
    origin_pc: Option<Address>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct R4InstructionData {
    fs3: Fs3,
    fs2: Fs2,
    fs1: Fs1,
    fd: Fd,
    fs3_value: Option<Float>,
    fs2_value: Option<Float>,
    fs1_value: Option<Float>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
pub struct Lb {
    data: IInstructionData,
    addr: Option<Address>,
}

impl Lb {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Lb { data, addr: None }
    }
}

impl Debug for Lb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "lb x{}, {}(x{})",
            self.data.rd, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lb {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.addr = Some((self.data.rs1_value.unwrap() + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        let value = core.load_byte(addr) as Int;
        self.data.rd_value = Some(value);
        core.set_forwarding_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lb".to_string()
    }
}

#[derive(Clone)]
pub struct Lh {
    data: IInstructionData,
    addr: Option<Address>,
}

impl Lh {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Lh { data, addr: None }
    }
}

impl Debug for Lh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "lh x{}, {}(x{})",
            self.data.rd, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lh {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.addr = Some((self.data.rs1_value.unwrap() + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        let value = core.load_half(addr) as Int;
        self.data.rd_value = Some(value);
        core.set_forwarding_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lh".to_string()
    }
}

#[derive(Clone)]
pub struct Lw {
    data: IInstructionData,
    addr: Option<Address>,
}

impl Lw {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Lw { data, addr: None }
    }
}

impl Debug for Lw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "lw x{}, {}(x{})",
            self.data.rd, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.addr = Some((self.data.rs1_value.unwrap() + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        let value = core.load_word(addr) as Int;
        self.data.rd_value = Some(value);
        core.set_forwarding_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lw".to_string()
    }
}

#[derive(Clone)]
pub struct Lbu {
    data: IInstructionData,
    addr: Option<Address>,
}

impl Lbu {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Lbu { data, addr: None }
    }
}

impl Debug for Lbu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lbu x{}, {}(x{})",
            self.data.rd, self.data.imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lbu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.addr = Some((self.data.rs1_value.unwrap() + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        let value = core.load_ubyte(addr) as Int;
        self.data.rd_value = Some(value);
        core.set_forwarding_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lbu".to_string()
    }
}

#[derive(Clone)]
pub struct Lhu {
    data: IInstructionData,
    addr: Option<Address>,
}

impl Lhu {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Lhu { data, addr: None }
    }
}

impl Debug for Lhu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "lhu x{}, {}(x{})",
            self.data.rd, self.data.imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lhu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.addr = Some((self.data.rs1_value.unwrap() + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        let value = core.load_uhalf(addr) as Int;
        self.data.rd_value = Some(value);
        core.set_forwarding_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lhu".to_string()
    }
}

#[derive(Clone)]
pub struct Addi {
    data: IInstructionData,
}

impl Addi {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Addi { data }
    }
}

impl Debug for Addi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "addi x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Addi {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value + extended_imm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "addi".to_string()
    }
}

#[derive(Clone)]
pub struct Slli {
    data: IInstructionData,
    uimm: Option<u32>,
}

impl Slli {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Slli { data, uimm: None }
    }
}

impl Debug for Slli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uimm = self.data.imm & 0x1f;
        write!(f, "slli x{}, x{}, {}", self.data.rd, self.data.rs1, uimm)
    }
}

impl InstructionTrait for Slli {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.uimm = Some((self.data.imm & 0x1f) as u32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(self.data.rs1_value.unwrap() << uimm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "slli".to_string()
    }
}

#[derive(Clone)]
pub struct Slti {
    data: IInstructionData,
}

impl Slti {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Slti { data }
    }
}

impl Debug for Slti {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "slti x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Slti {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = if rs1_value < extended_imm {
            Some(1)
        } else {
            Some(0)
        };
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "slti".to_string()
    }
}

#[derive(Clone)]
pub struct Sltiu {
    data: IInstructionData,
}

impl Sltiu {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Sltiu { data }
    }
}

impl Debug for Sltiu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "sltiu x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Sltiu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = i32_to_u32(self.data.rs1_value.unwrap());
        self.data.rd_value = if (rs1_value as i64) < (extended_imm as i64) {
            Some(1)
        } else {
            Some(0)
        };
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sltiu".to_string()
    }
}

#[derive(Clone)]
pub struct Xori {
    data: IInstructionData,
}

impl Xori {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Xori { data }
    }
}

impl Debug for Xori {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "xori x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Xori {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value ^ extended_imm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "xori".to_string()
    }
}

#[derive(Clone)]
pub struct Srli {
    data: IInstructionData,
    uimm: Option<u32>,
}

impl Srli {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Srli { data, uimm: None }
    }
}

impl Debug for Srli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uimm = self.data.imm & 0x1f;
        write!(f, "srli x{}, x{}, {}", self.data.rd, self.data.rs1, uimm)
    }
}

impl InstructionTrait for Srli {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.uimm = Some((self.data.imm & 0x1f) as u32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(u32_to_i32(
            i32_to_u32(self.data.rs1_value.unwrap()) >> uimm as u32,
        ));
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "srli".to_string()
    }
}

#[derive(Clone)]
pub struct Srai {
    data: IInstructionData,
    uimm: Option<u32>,
}

impl Srai {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Srai { data, uimm: None }
    }
}

impl Debug for Srai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uimm = self.data.imm & 0x1f;
        write!(f, "srai x{}, x{}, {}", self.data.rd, self.data.rs1, uimm)
    }
}

impl InstructionTrait for Srai {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.uimm = Some((self.data.imm & 0x1f) as u32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(self.data.rs1_value.unwrap() >> uimm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "srai".to_string()
    }
}

#[derive(Clone)]
pub struct Ori {
    data: IInstructionData,
}

impl Ori {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Ori { data }
    }
}

impl Debug for Ori {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "ori x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Ori {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value | extended_imm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "ori".to_string()
    }
}

#[derive(Clone)]
pub struct Andi {
    data: IInstructionData,
}

impl Andi {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Andi { data }
    }
}

impl Debug for Andi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "andi x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Andi {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value & extended_imm);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "andi".to_string()
    }
}

#[derive(Clone)]
pub struct Auipc {
    data: UInstructionData,
}

impl Auipc {
    fn new(imm: Imm20, rd: Rd) -> Self {
        let data = UInstructionData {
            imm,
            rd,
            upimm: None,
            rd_value: None,
            inst_count: None,
            origin_pc: None,
        };
        Auipc { data }
    }
}

impl Debug for Auipc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "auipc x{}, {}", self.data.rd, self.data.imm)
    }
}

impl InstructionTrait for Auipc {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.upimm = Some(self.data.imm << 12);
        self.data.origin_pc = Some(core.get_pc() - 4);
    }

    fn exec(&mut self, core: &mut Core) {
        let upimm = self.data.upimm.unwrap();
        let origin_pc = self.data.origin_pc.unwrap();
        self.data.rd_value = Some((upimm + origin_pc as i32) as Int);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "auipc".to_string()
    }
}

#[derive(Clone)]
pub struct Sb {
    data: SInstructionData,
    addr: Option<Address>,
}

impl Sb {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = SInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
        };
        Sb { data, addr: None }
    }
}

impl Debug for Sb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "sb x{}, {}(x{})",
            self.data.rs2, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Sb {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.addr = Some((rs1_value + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        core.store_byte(addr, (self.data.rs2_value.unwrap() & 0xff) as Byte);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sb".to_string()
    }
}

#[derive(Clone)]
pub struct Sh {
    data: SInstructionData,
    addr: Option<Address>,
}

impl Sh {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = SInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
        };
        Sh { data, addr: None }
    }
}

impl Debug for Sh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "sh x{}, {}(x{})",
            self.data.rs2, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Sh {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.addr = Some((rs1_value + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        core.store_half(addr, (self.data.rs2_value.unwrap() & 0xffff) as Half);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sh".to_string()
    }
}

#[derive(Clone)]
pub struct Sw {
    data: SInstructionData,
    addr: Option<Address>,
}

impl Sw {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = SInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
        };
        Sw { data, addr: None }
    }
}

impl Debug for Sw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "sw x{}, {}(x{})",
            self.data.rs2, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Sw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.addr = Some((rs1_value + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        core.store_word(addr, self.data.rs2_value.unwrap());
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sw".to_string()
    }
}

#[derive(Clone)]
pub struct Add {
    data: RInstructionData,
}

impl Add {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Add { data }
    }
}

impl Debug for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "add x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Add {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value + rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "add".to_string()
    }
}

#[derive(Clone)]
pub struct Sub {
    data: RInstructionData,
}

impl Sub {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Sub { data }
    }
}

impl Debug for Sub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sub x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Sub {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value - rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sub".to_string()
    }
}

#[derive(Clone)]
pub struct Sll {
    data: RInstructionData,
}

impl Sll {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Sll { data }
    }
}

impl Debug for Sll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sll x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Sll {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value as Int);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        let shift_value = rs2_value & 0x1f;
        self.data.rd_value = Some(u32_to_i32(i32_to_u32(rs1_value) << (shift_value as u32)));
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sll".to_string()
    }
}

#[derive(Clone)]
pub struct Slt {
    data: RInstructionData,
}

impl Slt {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Slt { data }
    }
}

impl Debug for Slt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "slt x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Slt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = if rs1_value < rs2_value {
            Some(1)
        } else {
            Some(0)
        };
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "slt".to_string()
    }
}

#[derive(Clone)]
pub struct Sltu {
    data: RInstructionData,
}

impl Sltu {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Sltu { data }
    }
}

impl Debug for Sltu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sltu x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Sltu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = if i32_to_u32(rs1_value) < i32_to_u32(rs2_value) {
            Some(1)
        } else {
            Some(0)
        };
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sltu".to_string()
    }
}

#[derive(Clone)]
pub struct Xor {
    data: RInstructionData,
}

impl Xor {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Xor { data }
    }
}

impl Debug for Xor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "xor x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Xor {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value ^ rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "xor".to_string()
    }
}

#[derive(Clone)]
pub struct Srl {
    data: RInstructionData,
}

impl Srl {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Srl { data }
    }
}

impl Debug for Srl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "srl x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Srl {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        let shift_value = rs2_value & 0x1f;
        self.data.rd_value = Some(u32_to_i32(i32_to_u32(rs1_value) >> (shift_value as u32)));
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "srl".to_string()
    }
}

#[derive(Clone)]
pub struct Sra {
    data: RInstructionData,
}

impl Sra {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Sra { data }
    }
}

impl Debug for Sra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sra x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Sra {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        let shift_value = rs2_value & 0x1f;
        self.data.rd_value = Some(rs1_value >> shift_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "sra".to_string()
    }
}

#[derive(Clone)]
pub struct Or {
    data: RInstructionData,
}

impl Or {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Or { data }
    }
}

impl Debug for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "or x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Or {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value | rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "or".to_string()
    }
}

#[derive(Clone)]
pub struct And {
    data: RInstructionData,
}

impl And {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        And { data }
    }
}

impl Debug for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "and x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for And {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value & rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "and".to_string()
    }
}

#[derive(Clone)]
pub struct Lui {
    data: UInstructionData,
}

impl Lui {
    fn new(imm: Imm20, rd: Rd) -> Self {
        let data = UInstructionData {
            imm,
            rd,
            upimm: None,
            rd_value: None,
            inst_count: None,
            origin_pc: None,
        };
        Lui { data }
    }
}

impl Debug for Lui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lui x{}, {}", self.data.rd, self.data.imm)
    }
}

impl InstructionTrait for Lui {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.upimm = Some(self.data.imm << 12);
        self.data.origin_pc = Some(core.get_pc() - 4);
    }

    fn exec(&mut self, core: &mut Core) {
        self.data.rd_value = Some(self.data.upimm.unwrap());
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "lui".to_string()
    }
}

#[derive(Clone)]
pub struct Beq {
    data: BInstructionData,
}

impl Beq {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Beq { data }
    }
}

impl Debug for Beq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "beq x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Beq {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value == rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "beq".to_string()
    }
}

#[derive(Clone)]
pub struct Bne {
    data: BInstructionData,
}

impl Bne {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Bne { data }
    }
}

impl Debug for Bne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "bne x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Bne {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value != rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "bne".to_string()
    }
}

#[derive(Clone)]
pub struct Blt {
    data: BInstructionData,
}

impl Blt {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Blt { data }
    }
}

impl Debug for Blt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "blt x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Blt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value < rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "blt".to_string()
    }
}

#[derive(Clone)]
pub struct Bge {
    data: BInstructionData,
}

impl Bge {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Bge { data }
    }
}

impl Debug for Bge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "bge x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Bge {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value >= rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "bge".to_string()
    }
}

#[derive(Clone)]
pub struct Bltu {
    data: BInstructionData,
}

impl Bltu {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Bltu { data }
    }
}

impl Debug for Bltu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "bltu x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Bltu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if i32_to_u32(rs1_value) < i32_to_u32(rs2_value) {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "bltu".to_string()
    }
}

#[derive(Clone)]
pub struct Bgeu {
    data: BInstructionData,
}

impl Bgeu {
    fn new(imm: Imm12, rs2: Rs2, rs1: Rs1) -> Self {
        let data = BInstructionData {
            imm,
            rs2,
            rs1,
            extended_imm: None,
            rs2_value: None,
            rs1_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Bgeu { data }
    }
}

impl Debug for Bgeu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        let origin_pc = self.data.origin_pc.unwrap();
        write!(
            f,
            "bgeu x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 1
        )
    }
}

impl InstructionTrait for Bgeu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.origin_pc = Some(core.get_pc() - 4);
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if i32_to_u32(rs1_value) == i32_to_u32(rs2_value) {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "bgeu".to_string()
    }
}

#[derive(Clone)]
pub struct Jalr {
    data: IInstructionData,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

impl Jalr {
    fn new(imm: Imm12, rs1: Rs1, rd: Rd) -> Self {
        let data = IInstructionData {
            imm,
            rs1,
            rd,
            extended_imm: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Jalr {
            data,
            origin_pc: None,
            jump_address: None,
        }
    }
}

impl Debug for Jalr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 12);
        write!(
            f,
            "jalr x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Jalr {
    fn register_fetch(&mut self, core: &Core) {
        self.origin_pc = Some(core.get_pc() - 4);
        self.data.inst_count = Some(core.get_instruction_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_source(self.data.rs1);
        if forwarding_source.is_some() {
            let (_, rs1_value) = forwarding_source.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.jump_address = Some((rs1_value + (extended_imm << 1)) as Address);
        self.data.rd_value = Some(self.origin_pc.unwrap() as i32 + 4);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "jalr".to_string()
    }
}

#[derive(Clone)]
pub struct Jal {
    data: JInstructionData,
}

impl Jal {
    fn new(imm: Imm20, rd: Rd) -> Self {
        let data = JInstructionData {
            imm,
            rd,
            extended_imm: None,
            rd_value: None,
            inst_count: None,
            origin_pc: None,
            jump_address: None,
        };
        Jal { data }
    }
}

impl Debug for Jal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i32(self.data.imm, 20);
        write!(f, "jal x{}, {}", self.data.rd, extended_imm)
    }
}

impl InstructionTrait for Jal {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i32(self.data.imm, 20));
        self.data.origin_pc = Some(core.get_pc() - 4);
        self.data.inst_count = Some(core.get_instruction_count());
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.data.jump_address =
            Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 1)) as Address);
        self.data.rd_value = Some(self.data.origin_pc.unwrap() as i32 + 4);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "jal".to_string()
    }
}

#[derive(Clone)]
pub struct Mul {
    data: RInstructionData,
}

impl Mul {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Mul { data }
    }
}

impl Debug for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mul x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Mul {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(((rs1_value as i64 * rs2_value as i64) & 0xffffffff) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "mul".to_string()
    }
}

#[derive(Clone)]
pub struct Mulh {
    data: RInstructionData,
}

impl Mulh {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Mulh { data }
    }
}

impl Debug for Mulh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mulh x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Mulh {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value =
            Some((((rs1_value as i64 * rs2_value as i64) >> 32) & 0xffffffff) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "mulh".to_string()
    }
}

#[derive(Clone)]
pub struct Mulhsu {
    data: RInstructionData,
}

impl Mulhsu {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Mulhsu { data }
    }
}

impl Debug for Mulhsu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mulhsu x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Mulhsu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = i32_to_u32(self.data.rs2_value.unwrap());
        self.data.rd_value = Some(((rs1_value as i64 * rs2_value as i64) & 0xffffffff) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "mulhsu".to_string()
    }
}

#[derive(Clone)]
pub struct Mulhu {
    data: RInstructionData,
}

impl Mulhu {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Mulhu { data }
    }
}

impl Debug for Mulhu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mulhu x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Mulhu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = i32_to_u32(self.data.rs1_value.unwrap());
        let rs2_value = i32_to_u32(self.data.rs2_value.unwrap());
        self.data.rd_value = Some(((rs1_value as i64 * rs2_value as i64) & 0xffffffff) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "mulhu".to_string()
    }
}

#[derive(Clone)]
pub struct Div {
    data: RInstructionData,
}

impl Div {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Div { data }
    }
}

impl Debug for Div {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "div x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Div {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value / rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "div".to_string()
    }
}

#[derive(Clone)]
pub struct Divu {
    data: RInstructionData,
}

impl Divu {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Divu { data }
    }
}

impl Debug for Divu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "divu x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Divu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = i32_to_u32(self.data.rs1_value.unwrap());
        let rs2_value = i32_to_u32(self.data.rs2_value.unwrap());
        self.data.rd_value = Some((rs1_value / rs2_value) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "divu".to_string()
    }
}

#[derive(Clone)]
pub struct Rem {
    data: RInstructionData,
}

impl Rem {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Rem { data }
    }
}

impl Debug for Rem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rem x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Rem {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value % rs2_value);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "div".to_string()
    }
}

#[derive(Clone)]
pub struct Remu {
    data: RInstructionData,
}

impl Remu {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = RInstructionData {
            rs2,
            rs1,
            rd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Remu { data }
    }
}

impl Debug for Remu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "remu x{}, x{}, x{}",
            self.data.rd, self.data.rs1, self.data.rs2
        )
    }
}

impl InstructionTrait for Remu {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_instruction_count());
        let forwarding_source_1 = core.get_forwarding_source(self.data.rs1);
        if forwarding_source_1.is_some() {
            let (_, rs1_value) = forwarding_source_1.unwrap();
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_source(self.data.rs2);
        if forwarding_source_2.is_some() {
            let (_, rs2_value) = forwarding_source_2.unwrap();
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = i32_to_u32(self.data.rs1_value.unwrap());
        let rs2_value = i32_to_u32(self.data.rs2_value.unwrap());
        self.data.rd_value = Some((rs1_value % rs2_value) as i32);
        core.set_forwarding_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_destination_register(&self) -> Option<Rd> {
        Some(self.data.rd)
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "divu".to_string()
    }
}

#[derive(Clone)]
pub enum InstructionEnum {
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Lbu(Lbu),
    Lhu(Lhu),
    Addi(Addi),
    Slli(Slli),
    Slti(Slti),
    Sltiu(Sltiu),
    Xori(Xori),
    Srli(Srli),
    Srai(Srai),
    Ori(Ori),
    Andi(Andi),
    Auipc(Auipc),
    Sb(Sb),
    Sh(Sh),
    Sw(Sw),
    Add(Add),
    Sub(Sub),
    Sll(Sll),
    Slt(Slt),
    Sltu(Sltu),
    Xor(Xor),
    Srl(Srl),
    Sra(Sra),
    Or(Or),
    And(And),
    Lui(Lui),
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Bge(Bge),
    Bltu(Bltu),
    Bgeu(Bgeu),
    Jalr(Jalr),
    Jal(Jal),
    Mul(Mul),
    Mulh(Mulh),
    Mulhsu(Mulhsu),
    Mulhu(Mulhu),
    Div(Div),
    Divu(Divu),
    Rem(Rem),
    Remu(Remu),
}

impl Debug for InstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionEnum::Lb(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lh(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lbu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lhu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Addi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slli(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slti(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sltiu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Xori(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Srli(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Srai(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Ori(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Andi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Auipc(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sb(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sh(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Add(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sub(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sll(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sltu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Xor(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Srl(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sra(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Or(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::And(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lui(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Beq(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bne(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Blt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bge(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bltu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bgeu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jalr(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jal(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Mul(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Mulh(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Mulhsu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Mulhu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Div(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Divu(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Rem(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Remu(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for InstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lh(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lbu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lhu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Addi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slli(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slti(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sltiu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Xori(instruction) => instruction.register_fetch(core),
            InstructionEnum::Srli(instruction) => instruction.register_fetch(core),
            InstructionEnum::Srai(instruction) => instruction.register_fetch(core),
            InstructionEnum::Ori(instruction) => instruction.register_fetch(core),
            InstructionEnum::Andi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Auipc(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sb(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sh(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Add(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sub(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sll(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sltu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Xor(instruction) => instruction.register_fetch(core),
            InstructionEnum::Srl(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sra(instruction) => instruction.register_fetch(core),
            InstructionEnum::Or(instruction) => instruction.register_fetch(core),
            InstructionEnum::And(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lui(instruction) => instruction.register_fetch(core),
            InstructionEnum::Beq(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bne(instruction) => instruction.register_fetch(core),
            InstructionEnum::Blt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bge(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bltu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bgeu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jalr(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jal(instruction) => instruction.register_fetch(core),
            InstructionEnum::Mul(instruction) => instruction.register_fetch(core),
            InstructionEnum::Mulh(instruction) => instruction.register_fetch(core),
            InstructionEnum::Mulhsu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Mulhu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Div(instruction) => instruction.register_fetch(core),
            InstructionEnum::Divu(instruction) => instruction.register_fetch(core),
            InstructionEnum::Rem(instruction) => instruction.register_fetch(core),
            InstructionEnum::Remu(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.exec(core),
            InstructionEnum::Lh(instruction) => instruction.exec(core),
            InstructionEnum::Lw(instruction) => instruction.exec(core),
            InstructionEnum::Lbu(instruction) => instruction.exec(core),
            InstructionEnum::Lhu(instruction) => instruction.exec(core),
            InstructionEnum::Addi(instruction) => instruction.exec(core),
            InstructionEnum::Slli(instruction) => instruction.exec(core),
            InstructionEnum::Slti(instruction) => instruction.exec(core),
            InstructionEnum::Sltiu(instruction) => instruction.exec(core),
            InstructionEnum::Xori(instruction) => instruction.exec(core),
            InstructionEnum::Srli(instruction) => instruction.exec(core),
            InstructionEnum::Srai(instruction) => instruction.exec(core),
            InstructionEnum::Ori(instruction) => instruction.exec(core),
            InstructionEnum::Andi(instruction) => instruction.exec(core),
            InstructionEnum::Auipc(instruction) => instruction.exec(core),
            InstructionEnum::Sb(instruction) => instruction.exec(core),
            InstructionEnum::Sh(instruction) => instruction.exec(core),
            InstructionEnum::Sw(instruction) => instruction.exec(core),
            InstructionEnum::Add(instruction) => instruction.exec(core),
            InstructionEnum::Sub(instruction) => instruction.exec(core),
            InstructionEnum::Sll(instruction) => instruction.exec(core),
            InstructionEnum::Slt(instruction) => instruction.exec(core),
            InstructionEnum::Sltu(instruction) => instruction.exec(core),
            InstructionEnum::Xor(instruction) => instruction.exec(core),
            InstructionEnum::Srl(instruction) => instruction.exec(core),
            InstructionEnum::Sra(instruction) => instruction.exec(core),
            InstructionEnum::Or(instruction) => instruction.exec(core),
            InstructionEnum::And(instruction) => instruction.exec(core),
            InstructionEnum::Lui(instruction) => instruction.exec(core),
            InstructionEnum::Beq(instruction) => instruction.exec(core),
            InstructionEnum::Bne(instruction) => instruction.exec(core),
            InstructionEnum::Blt(instruction) => instruction.exec(core),
            InstructionEnum::Bge(instruction) => instruction.exec(core),
            InstructionEnum::Bltu(instruction) => instruction.exec(core),
            InstructionEnum::Bgeu(instruction) => instruction.exec(core),
            InstructionEnum::Jalr(instruction) => instruction.exec(core),
            InstructionEnum::Jal(instruction) => instruction.exec(core),
            InstructionEnum::Mul(instruction) => instruction.exec(core),
            InstructionEnum::Mulh(instruction) => instruction.exec(core),
            InstructionEnum::Mulhsu(instruction) => instruction.exec(core),
            InstructionEnum::Mulhu(instruction) => instruction.exec(core),
            InstructionEnum::Div(instruction) => instruction.exec(core),
            InstructionEnum::Divu(instruction) => instruction.exec(core),
            InstructionEnum::Rem(instruction) => instruction.exec(core),
            InstructionEnum::Remu(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.memory(core),
            InstructionEnum::Lh(instruction) => instruction.memory(core),
            InstructionEnum::Lw(instruction) => instruction.memory(core),
            InstructionEnum::Lbu(instruction) => instruction.memory(core),
            InstructionEnum::Lhu(instruction) => instruction.memory(core),
            InstructionEnum::Addi(instruction) => instruction.memory(core),
            InstructionEnum::Slli(instruction) => instruction.memory(core),
            InstructionEnum::Slti(instruction) => instruction.memory(core),
            InstructionEnum::Sltiu(instruction) => instruction.memory(core),
            InstructionEnum::Xori(instruction) => instruction.memory(core),
            InstructionEnum::Srli(instruction) => instruction.memory(core),
            InstructionEnum::Srai(instruction) => instruction.memory(core),
            InstructionEnum::Ori(instruction) => instruction.memory(core),
            InstructionEnum::Andi(instruction) => instruction.memory(core),
            InstructionEnum::Auipc(instruction) => instruction.memory(core),
            InstructionEnum::Sb(instruction) => instruction.memory(core),
            InstructionEnum::Sh(instruction) => instruction.memory(core),
            InstructionEnum::Sw(instruction) => instruction.memory(core),
            InstructionEnum::Add(instruction) => instruction.memory(core),
            InstructionEnum::Sub(instruction) => instruction.memory(core),
            InstructionEnum::Sll(instruction) => instruction.memory(core),
            InstructionEnum::Slt(instruction) => instruction.memory(core),
            InstructionEnum::Sltu(instruction) => instruction.memory(core),
            InstructionEnum::Xor(instruction) => instruction.memory(core),
            InstructionEnum::Srl(instruction) => instruction.memory(core),
            InstructionEnum::Sra(instruction) => instruction.memory(core),
            InstructionEnum::Or(instruction) => instruction.memory(core),
            InstructionEnum::And(instruction) => instruction.memory(core),
            InstructionEnum::Lui(instruction) => instruction.memory(core),
            InstructionEnum::Beq(instruction) => instruction.memory(core),
            InstructionEnum::Bne(instruction) => instruction.memory(core),
            InstructionEnum::Blt(instruction) => instruction.memory(core),
            InstructionEnum::Bge(instruction) => instruction.memory(core),
            InstructionEnum::Bltu(instruction) => instruction.memory(core),
            InstructionEnum::Bgeu(instruction) => instruction.memory(core),
            InstructionEnum::Jalr(instruction) => instruction.memory(core),
            InstructionEnum::Jal(instruction) => instruction.memory(core),
            InstructionEnum::Mul(instruction) => instruction.memory(core),
            InstructionEnum::Mulh(instruction) => instruction.memory(core),
            InstructionEnum::Mulhsu(instruction) => instruction.memory(core),
            InstructionEnum::Mulhu(instruction) => instruction.memory(core),
            InstructionEnum::Div(instruction) => instruction.memory(core),
            InstructionEnum::Divu(instruction) => instruction.memory(core),
            InstructionEnum::Rem(instruction) => instruction.memory(core),
            InstructionEnum::Remu(instruction) => instruction.memory(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.write_back(core),
            InstructionEnum::Lh(instruction) => instruction.write_back(core),
            InstructionEnum::Lw(instruction) => instruction.write_back(core),
            InstructionEnum::Lbu(instruction) => instruction.write_back(core),
            InstructionEnum::Lhu(instruction) => instruction.write_back(core),
            InstructionEnum::Addi(instruction) => instruction.write_back(core),
            InstructionEnum::Slli(instruction) => instruction.write_back(core),
            InstructionEnum::Slti(instruction) => instruction.write_back(core),
            InstructionEnum::Sltiu(instruction) => instruction.write_back(core),
            InstructionEnum::Xori(instruction) => instruction.write_back(core),
            InstructionEnum::Srli(instruction) => instruction.write_back(core),
            InstructionEnum::Srai(instruction) => instruction.write_back(core),
            InstructionEnum::Ori(instruction) => instruction.write_back(core),
            InstructionEnum::Andi(instruction) => instruction.write_back(core),
            InstructionEnum::Auipc(instruction) => instruction.write_back(core),
            InstructionEnum::Sb(instruction) => instruction.write_back(core),
            InstructionEnum::Sh(instruction) => instruction.write_back(core),
            InstructionEnum::Sw(instruction) => instruction.write_back(core),
            InstructionEnum::Add(instruction) => instruction.write_back(core),
            InstructionEnum::Sub(instruction) => instruction.write_back(core),
            InstructionEnum::Sll(instruction) => instruction.write_back(core),
            InstructionEnum::Slt(instruction) => instruction.write_back(core),
            InstructionEnum::Sltu(instruction) => instruction.write_back(core),
            InstructionEnum::Xor(instruction) => instruction.write_back(core),
            InstructionEnum::Srl(instruction) => instruction.write_back(core),
            InstructionEnum::Sra(instruction) => instruction.write_back(core),
            InstructionEnum::Or(instruction) => instruction.write_back(core),
            InstructionEnum::And(instruction) => instruction.write_back(core),
            InstructionEnum::Lui(instruction) => instruction.write_back(core),
            InstructionEnum::Beq(instruction) => instruction.write_back(core),
            InstructionEnum::Bne(instruction) => instruction.write_back(core),
            InstructionEnum::Blt(instruction) => instruction.write_back(core),
            InstructionEnum::Bge(instruction) => instruction.write_back(core),
            InstructionEnum::Bltu(instruction) => instruction.write_back(core),
            InstructionEnum::Bgeu(instruction) => instruction.write_back(core),
            InstructionEnum::Jalr(instruction) => instruction.write_back(core),
            InstructionEnum::Jal(instruction) => instruction.write_back(core),
            InstructionEnum::Mul(instruction) => instruction.write_back(core),
            InstructionEnum::Mulh(instruction) => instruction.write_back(core),
            InstructionEnum::Mulhsu(instruction) => instruction.write_back(core),
            InstructionEnum::Mulhu(instruction) => instruction.write_back(core),
            InstructionEnum::Div(instruction) => instruction.write_back(core),
            InstructionEnum::Divu(instruction) => instruction.write_back(core),
            InstructionEnum::Rem(instruction) => instruction.write_back(core),
            InstructionEnum::Remu(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lh(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lbu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lhu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Addi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slli(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slti(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sltiu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Xori(instruction) => instruction.get_source_registers(),
            InstructionEnum::Srli(instruction) => instruction.get_source_registers(),
            InstructionEnum::Srai(instruction) => instruction.get_source_registers(),
            InstructionEnum::Ori(instruction) => instruction.get_source_registers(),
            InstructionEnum::Andi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Auipc(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sb(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sh(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Add(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sub(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sll(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sltu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Xor(instruction) => instruction.get_source_registers(),
            InstructionEnum::Srl(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sra(instruction) => instruction.get_source_registers(),
            InstructionEnum::Or(instruction) => instruction.get_source_registers(),
            InstructionEnum::And(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lui(instruction) => instruction.get_source_registers(),
            InstructionEnum::Beq(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bne(instruction) => instruction.get_source_registers(),
            InstructionEnum::Blt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bge(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bltu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bgeu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jalr(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jal(instruction) => instruction.get_source_registers(),
            InstructionEnum::Mul(instruction) => instruction.get_source_registers(),
            InstructionEnum::Mulh(instruction) => instruction.get_source_registers(),
            InstructionEnum::Mulhsu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Mulhu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Div(instruction) => instruction.get_source_registers(),
            InstructionEnum::Divu(instruction) => instruction.get_source_registers(),
            InstructionEnum::Rem(instruction) => instruction.get_source_registers(),
            InstructionEnum::Remu(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lh(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lbu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lhu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Addi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slli(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slti(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sltiu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Xori(instruction) => instruction.get_destination_register(),
            InstructionEnum::Srli(instruction) => instruction.get_destination_register(),
            InstructionEnum::Srai(instruction) => instruction.get_destination_register(),
            InstructionEnum::Ori(instruction) => instruction.get_destination_register(),
            InstructionEnum::Andi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Auipc(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sb(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sh(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Add(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sub(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sll(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sltu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Xor(instruction) => instruction.get_destination_register(),
            InstructionEnum::Srl(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sra(instruction) => instruction.get_destination_register(),
            InstructionEnum::Or(instruction) => instruction.get_destination_register(),
            InstructionEnum::And(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lui(instruction) => instruction.get_destination_register(),
            InstructionEnum::Beq(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bne(instruction) => instruction.get_destination_register(),
            InstructionEnum::Blt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bge(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bltu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bgeu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jalr(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jal(instruction) => instruction.get_destination_register(),
            InstructionEnum::Mul(instruction) => instruction.get_destination_register(),
            InstructionEnum::Mulh(instruction) => instruction.get_destination_register(),
            InstructionEnum::Mulhsu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Mulhu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Div(instruction) => instruction.get_destination_register(),
            InstructionEnum::Divu(instruction) => instruction.get_destination_register(),
            InstructionEnum::Rem(instruction) => instruction.get_destination_register(),
            InstructionEnum::Remu(instruction) => instruction.get_destination_register(),
        }
    }

    fn is_load_instruction(&self) -> bool {
        match self {
            InstructionEnum::Lb(_) => true,
            InstructionEnum::Lh(_) => true,
            InstructionEnum::Lw(_) => true,
            _ => false,
        }
    }

    fn is_branch_instruction(&self) -> bool {
        match self {
            InstructionEnum::Lb(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Lh(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Lw(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Lbu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Lhu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Addi(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Slli(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Slti(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sltiu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Xori(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Srli(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Srai(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Ori(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Andi(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Auipc(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sb(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sh(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sw(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Add(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sub(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sll(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Slt(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sltu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Xor(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Srl(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sra(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Or(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::And(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Lui(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Beq(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bne(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Blt(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bge(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bltu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bgeu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Jalr(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Jal(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Mul(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Mulh(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Mulhsu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Mulhu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Div(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Divu(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Rem(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Remu(instruction) => instruction.is_branch_instruction(),
        }
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lh(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lbu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lhu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Addi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slli(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slti(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sltiu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Xori(instruction) => instruction.get_jump_address(),
            InstructionEnum::Srli(instruction) => instruction.get_jump_address(),
            InstructionEnum::Srai(instruction) => instruction.get_jump_address(),
            InstructionEnum::Ori(instruction) => instruction.get_jump_address(),
            InstructionEnum::Andi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Auipc(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sb(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sh(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Add(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sub(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sll(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sltu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Xor(instruction) => instruction.get_jump_address(),
            InstructionEnum::Srl(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sra(instruction) => instruction.get_jump_address(),
            InstructionEnum::Or(instruction) => instruction.get_jump_address(),
            InstructionEnum::And(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lui(instruction) => instruction.get_jump_address(),
            InstructionEnum::Beq(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bne(instruction) => instruction.get_jump_address(),
            InstructionEnum::Blt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bge(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bltu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bgeu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jalr(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jal(instruction) => instruction.get_jump_address(),
            InstructionEnum::Mul(instruction) => instruction.get_jump_address(),
            InstructionEnum::Mulh(instruction) => instruction.get_jump_address(),
            InstructionEnum::Mulhsu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Mulhu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Div(instruction) => instruction.get_jump_address(),
            InstructionEnum::Divu(instruction) => instruction.get_jump_address(),
            InstructionEnum::Rem(instruction) => instruction.get_jump_address(),
            InstructionEnum::Remu(instruction) => instruction.get_jump_address(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lh(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lbu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lhu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Addi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slli(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slti(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sltiu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Xori(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Srli(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Srai(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Ori(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Andi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Auipc(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sb(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sh(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Add(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sub(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sll(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sltu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Xor(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Srl(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sra(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Or(instruction) => instruction.get_instruction_count(),
            InstructionEnum::And(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lui(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Beq(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bne(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Blt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bge(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bltu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bgeu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jalr(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jal(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Mul(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Mulh(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Mulhsu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Mulhu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Div(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Divu(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Rem(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Remu(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_name(),
            InstructionEnum::Lh(instruction) => instruction.get_name(),
            InstructionEnum::Lw(instruction) => instruction.get_name(),
            InstructionEnum::Lbu(instruction) => instruction.get_name(),
            InstructionEnum::Lhu(instruction) => instruction.get_name(),
            InstructionEnum::Addi(instruction) => instruction.get_name(),
            InstructionEnum::Slli(instruction) => instruction.get_name(),
            InstructionEnum::Slti(instruction) => instruction.get_name(),
            InstructionEnum::Sltiu(instruction) => instruction.get_name(),
            InstructionEnum::Xori(instruction) => instruction.get_name(),
            InstructionEnum::Srli(instruction) => instruction.get_name(),
            InstructionEnum::Srai(instruction) => instruction.get_name(),
            InstructionEnum::Ori(instruction) => instruction.get_name(),
            InstructionEnum::Andi(instruction) => instruction.get_name(),
            InstructionEnum::Auipc(instruction) => instruction.get_name(),
            InstructionEnum::Sb(instruction) => instruction.get_name(),
            InstructionEnum::Sh(instruction) => instruction.get_name(),
            InstructionEnum::Sw(instruction) => instruction.get_name(),
            InstructionEnum::Add(instruction) => instruction.get_name(),
            InstructionEnum::Sub(instruction) => instruction.get_name(),
            InstructionEnum::Sll(instruction) => instruction.get_name(),
            InstructionEnum::Slt(instruction) => instruction.get_name(),
            InstructionEnum::Sltu(instruction) => instruction.get_name(),
            InstructionEnum::Xor(instruction) => instruction.get_name(),
            InstructionEnum::Srl(instruction) => instruction.get_name(),
            InstructionEnum::Sra(instruction) => instruction.get_name(),
            InstructionEnum::Or(instruction) => instruction.get_name(),
            InstructionEnum::And(instruction) => instruction.get_name(),
            InstructionEnum::Lui(instruction) => instruction.get_name(),
            InstructionEnum::Beq(instruction) => instruction.get_name(),
            InstructionEnum::Bne(instruction) => instruction.get_name(),
            InstructionEnum::Blt(instruction) => instruction.get_name(),
            InstructionEnum::Bge(instruction) => instruction.get_name(),
            InstructionEnum::Bltu(instruction) => instruction.get_name(),
            InstructionEnum::Bgeu(instruction) => instruction.get_name(),
            InstructionEnum::Jalr(instruction) => instruction.get_name(),
            InstructionEnum::Jal(instruction) => instruction.get_name(),
            InstructionEnum::Mul(instruction) => instruction.get_name(),
            InstructionEnum::Mulh(instruction) => instruction.get_name(),
            InstructionEnum::Mulhsu(instruction) => instruction.get_name(),
            InstructionEnum::Mulhu(instruction) => instruction.get_name(),
            InstructionEnum::Div(instruction) => instruction.get_name(),
            InstructionEnum::Divu(instruction) => instruction.get_name(),
            InstructionEnum::Rem(instruction) => instruction.get_name(),
            InstructionEnum::Remu(instruction) => instruction.get_name(),
        }
    }
}

fn create_i_instruction_struct(
    imm: Imm12,
    rs1: Rs1,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        3 => match funct3 {
            0b000 => InstructionEnum::Lb(Lb::new(imm, rs1, rd)),
            0b001 => InstructionEnum::Lh(Lh::new(imm, rs1, rd)),
            0b010 => InstructionEnum::Lw(Lw::new(imm, rs1, rd)),
            0b100 => InstructionEnum::Lbu(Lbu::new(imm, rs1, rd)),
            0b101 => InstructionEnum::Lhu(Lhu::new(imm, rs1, rd)),
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        19 => match funct3 {
            0b000 => InstructionEnum::Addi(Addi::new(imm, rs1, rd)),
            0b001 => InstructionEnum::Slli(Slli::new(imm, rs1, rd)),
            0b010 => InstructionEnum::Slti(Slti::new(imm, rs1, rd)),
            0b011 => InstructionEnum::Sltiu(Sltiu::new(imm, rs1, rd)),
            0b100 => InstructionEnum::Xori(Xori::new(imm, rs1, rd)),
            0b101 => {
                let funct7 = (imm >> 5) & 0b1111111;
                match funct7 {
                    0b0000000 => InstructionEnum::Srli(Srli::new(imm, rs1, rd)),
                    0b0100000 => InstructionEnum::Srai(Srai::new(imm, rs1, rd)),
                    _ => {
                        println!("unexpected funct7: {}", funct7);
                        panic!();
                    }
                }
            }
            0b110 => InstructionEnum::Ori(Ori::new(imm, rs1, rd)),
            0b111 => InstructionEnum::Andi(Andi::new(imm, rs1, rd)),
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        103 => match funct3 {
            0b000 => InstructionEnum::Jalr(Jalr::new(imm, rs1, rd)),
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        // 7 => {
        //     match funct3 {
        //         0b010 => {
        //             // flw
        //         }
        //         _ => {
        //             println!("unexpected funct3: {}", funct3)
        //         }
        //     }
        // }
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_r_instruction_struct(
    funct7: Funct7,
    rs2: Rs2,
    rs1: Rs2,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        // TODO: swap instructions
        51 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => InstructionEnum::Add(Add::new(rs2, rs1, rd)),
                0b0100000 => InstructionEnum::Sub(Sub::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Mul(Mul::new(rs2, rs1, rd)),
                // 0b0110000 => {
                //     // absdiff
                // }
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b001 => match funct7 {
                0b0000000 => InstructionEnum::Sll(Sll::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Mulh(Mulh::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b010 => match funct7 {
                0b0000000 => InstructionEnum::Slt(Slt::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Mulhsu(Mulhsu::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b011 => match funct7 {
                0b0000000 => InstructionEnum::Sltu(Sltu::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Mulhu(Mulhu::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b100 => match funct7 {
                0b0000000 => InstructionEnum::Xor(Xor::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Div(Div::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b101 => match funct7 {
                0b0000000 => InstructionEnum::Srl(Srl::new(rs2, rs1, rd)),
                0b0100000 => InstructionEnum::Sra(Sra::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Divu(Divu::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b110 => match funct7 {
                0b0000000 => InstructionEnum::Or(Or::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Rem(Rem::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b111 => match funct7 {
                0b0000000 => InstructionEnum::And(And::new(rs2, rs1, rd)),
                0b0000001 => InstructionEnum::Remu(Remu::new(rs2, rs1, rd)),
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        // 83 => match funct7 >> 2 {
        // 0b00000 => {
        //     // fadd
        // }
        // 0b00001 => {
        //     // fsub
        // }
        // 0b00010 => {
        //     // fmul
        // }
        // 0b00011 => {
        //     // fdiv
        // }
        // 0b01011 => {
        //     // fsqrt
        // }
        // 0b00100 => {
        //     match funct3 {
        //         0b000 => {
        //             // fsgnj
        //         }
        //         0b001 => {
        //             // fsgnjn
        //         }
        //         0b010 => {
        //             // fsgnjx
        //         }
        //         _ => {
        //             println!("unexpected funct3: {}", funct3)
        //         }
        //     }
        // }
        // 0b00101 => {
        //     match funct3 {
        //         0b000 => {
        //             // fmin
        //         }
        //         0b001 => {
        //             // fmax
        //         }
        //         _ => {
        //             println!("unexpected funct3: {}", funct3)
        //         }
        //     }
        // }
        // 0b10100 => match funct3 {
        //     0b010 => {
        //         // feq
        //     }
        //     0b001 => {
        //         // flt
        //     }
        //     0b000 => {
        //         // fle
        //     }
        //     _ => {
        //         println!("unexpected funct3: {}", funct3)
        //     }
        // },
        // 0b11100 => match funct3 {
        //     0b001 => {
        //         // fclass
        //     }
        //     _ => {
        //         println!("unexpected funct3: {}", funct3)
        //     }
        // },
        // 0b1100000 => {
        //     match rs2 {
        //         0b00000 => {
        //             // fcvt.w.s
        //         }
        //         0b00001 => {
        //             // fcvt.wu.s
        //         }
        //         _ => {
        //             println!("unexpected rs2: {}", rs2)
        //         }
        //     }
        // }
        // 0b1101000 => {
        //     match rs2 {
        //         0b00000 => {
        //             // fcvt.s.w
        //         }
        //         0b00001 => {
        //             // fcvt.s.wu
        //         }
        //         _ => {
        //             println!("unexpected rs2: {}", rs2)
        //         }
        //     }
        // }
        // 0b1110000 => {
        //     match rs2 {
        //         0b00000 => {
        //             // fmvs.x.w
        //         }
        //         _ => {
        //             println!("unexpected rs2: {}", rs2)
        //         }
        //     }
        // }
        // 0b1111000 => {
        //     match rs2 {
        //         0b00000 => {
        //             // fmv.w.x
        //         }
        //         _ => {
        //             println!("unexpected rs2: {}", rs2)
        //         }
        //     }
        // }
        // _ => {
        //     println!("unexpected funct7: {}", funct7)
        // }
        // },
        // 52 => {
        // match funct3 {
        // 0b000 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swapw
        //         }
        //         _ => {
        //             println!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // 0b001 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swaph
        //         }
        //         _ => {
        //             println!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // 0b010 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swapb
        //         }
        //         _ => {
        //             println!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // _ => {
        //     println!("unexpected funct3: {}", funct3)
        // }
        // }
        // }
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_s_instruction_struct(
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        35 => match funct3 {
            0b000 => InstructionEnum::Sb(Sb::new(imm, rs2, rs1)),
            0b001 => InstructionEnum::Sh(Sh::new(imm, rs2, rs1)),
            0b010 => InstructionEnum::Sw(Sw::new(imm, rs2, rs1)),
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        // 39 => {
        //     match funct3 {
        //         0b010 => {
        //             // fsw
        //         }
        //         _ => {
        //             println!("unexpected funct3: {}", funct3)
        //         }
        //     }
        // }
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_b_instruction_struct(
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        99 => match funct3 {
            0b000 => InstructionEnum::Beq(Beq::new(imm, rs2, rs1)),
            0b001 => InstructionEnum::Bne(Bne::new(imm, rs2, rs1)),
            0b100 => InstructionEnum::Blt(Blt::new(imm, rs2, rs1)),
            0b101 => InstructionEnum::Bge(Bge::new(imm, rs2, rs1)),
            0b110 => InstructionEnum::Bltu(Bltu::new(imm, rs2, rs1)),
            0b111 => InstructionEnum::Bgeu(Bgeu::new(imm, rs2, rs1)),
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_j_instruction_struct(imm: Imm20, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        111 => InstructionEnum::Jal(Jal::new(imm, rd)),
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_u_instruction_struct(imm: Imm20, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        23 => InstructionEnum::Auipc(Auipc::new(imm, rd)),
        55 => InstructionEnum::Lui(Lui::new(imm, rd)),
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

fn create_r4_instruction_struct(
    fs3: Fs3,
    funct2: Funct2,
    fs2: Fs2,
    fs1: Fs1,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        // 67 => {
        //     // fmadd
        // }
        // 71 => {
        //     // fmsub
        // }
        // 75 => {
        //     // fnmsub
        // }
        // 79 => {
        //     // fnmadd
        // }
        _ => {
            println!("unexpected op: {}", op);
            panic!();
        }
    }
}

pub fn create_instruction_struct(inst: Instruction) -> InstructionEnum {
    match inst {
        Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
            create_i_instruction_struct(imm, rs1, funct3, rd, op)
        }
        Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
            create_r_instruction_struct(funct7, rs2, rs1, funct3, rd, op)
        }
        Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
            create_s_instruction_struct(imm, rs2, rs1, funct3, op)
        }
        Instruction::BInstruction(imm, rs2, rs1, funct3, op) => {
            create_b_instruction_struct(imm, rs2, rs1, funct3, op)
        }
        Instruction::JInstruction(imm, rd, op) => create_j_instruction_struct(imm, rd, op),
        Instruction::UInstruction(imm, rd, op) => create_u_instruction_struct(imm, rd, op),
        Instruction::R4Instruction(fs3, funct2, fs2, fs1, funct3, rd, op) => {
            create_r4_instruction_struct(fs3, funct2, fs2, fs1, funct3, rd, op)
        }
        _ => {
            println!("unexpected instruction: {:?}", inst);
            panic!();
        }
    }
}

pub fn register_fetch(core: &mut Core) {
    if core.get_decoded_instruction().is_none() {
        return;
    }
    let mut inst = core.get_decoded_instruction().clone().unwrap();
    inst.register_fetch(core);
    core.set_decoded_instruction(Some(inst));
}

pub fn exec_instruction(core: &mut Core) {
    if core.get_instruction_in_exec_stage().is_none() {
        return;
    }
    let mut inst = core.get_instruction_in_exec_stage().clone().unwrap();
    inst.exec(core);
    core.set_instruction_in_exec_stage(Some(inst));
}

pub fn memory_access(core: &mut Core) {
    if core.get_instruction_in_memory_stage().is_none() {
        return;
    }
    let mut inst = core.get_instruction_in_memory_stage().clone().unwrap();
    inst.memory(core);
    core.set_instruction_in_memory_stage(Some(inst));
}

pub fn write_back(core: &mut Core) {
    if core.get_instruction_in_write_back_stage().is_none() {
        return;
    }
    let inst = core.get_instruction_in_write_back_stage().clone().unwrap();
    inst.write_back(core);
}

pub fn get_source_registers(inst: &InstructionEnum) -> Vec<Rs> {
    inst.get_source_registers()
}

pub fn get_destination_register(inst: &InstructionEnum) -> Option<Rd> {
    inst.get_destination_register()
}

pub fn is_load_instruction(inst: &InstructionEnum) -> bool {
    inst.is_load_instruction()
}

pub fn is_branch_instruction(inst: &InstructionEnum) -> bool {
    inst.is_branch_instruction()
}

pub fn get_jump_address(inst: &InstructionEnum) -> Option<Address> {
    inst.get_jump_address()
}

pub fn get_instruction_count(inst: &InstructionEnum) -> Option<InstructionCount> {
    inst.get_instruction_count()
}

// fn create_i_instruction_map() -> IInstructionMap {
//     let mut map = IInstructionMap::new();
//     let flw = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("flw f{}, {}(x{})", rd, imm, rs1));
//             }
//             let value = core
//                 .load_word((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
//             core.set_float_register(rd as usize, f32::from_bits(i32_to_u32(value)));
//         },
//         name: "flw",
//     };
//     map.insert((7, 0b010), flw);
//     let in_ = IInstructionExecutor {
//         exec: |_: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(imm, 0);
//             assert_eq!(rd, 0);
//             if verbose {
//                 println_inst(&format!("in x{}", rs1));
//             }
//         },
//         name: "in",
//     };
//     map.insert((115, 0b000), in_);
//     let outuart = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(imm, 0);
//             assert_eq!(rd, 0);
//             if verbose {
//                 println_inst(&format!("outuart x{}", rs1));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             eprintln!("{:>08x}", rs1_value);
//         },
//         name: "outuart",
//     };
//     map.insert((115, 0b100), outuart);
//     let out7seg8 = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
//             assert_eq!(imm, 0);
//             assert_eq!(rd, 0);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             eprintln!("{:>08x}", rs1_value);
//         },
//         name: "out7seg8",
//     };
//     map.insert((115, 0b101), out7seg8);
//     let out7seg1 = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
//             assert_eq!(imm, 0);
//             assert_eq!(rd, 0);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             eprintln!("{:>01x}", i32_to_u32(rs1_value) & 0b1111);
//         },
//         name: "out7seg16",
//     };
//     map.insert((115, 0b110), out7seg1);
//     let outled = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, _: bool| {
//             assert_eq!(imm, 0);
//             assert_eq!(rd, 0);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             eprintln!("{:>016b}", i32_to_u32(rs1_value) & 65535);
//         },
//         name: "outled",
//     };
//     map.insert((115, 0b111), outled);
//     map
// }

// fn create_r_instruction_map() -> RInstructionMap {
//     let mut map = RInstructionMap::new();
//     let absdiff = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("absdiff x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value > rs2_value {
//                 core.set_int_register(rd as usize, rs1_value - rs2_value);
//             } else {
//                 core.set_int_register(rd as usize, rs2_value - rs1_value);
//             }
//         },
//         name: "absdiff",
//     };
//     map.insert((51, 0b000, 0b0110000), absdiff);
//     let fadd = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fadd f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value + rs2_value);
//         },
//         name: "fadd",
//     };
//     map.insert((83, 0b000, 0b0000000), fadd);
//     let fsub = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fsub f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value - rs2_value);
//         },
//         name: "fsub",
//     };
//     map.insert((83, 0b000, 0b0000100), fsub);
//     let fmul = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fmul f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value * rs2_value);
//         },
//         name: "fmul",
//     };
//     map.insert((83, 0b000, 0b0001000), fmul);
//     let fdiv = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fadd f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value / rs2_value);
//         },
//         name: "fdiv",
//     };
//     map.insert((83, 0b000, 0b0001100), fdiv);
//     let fsqrt = RInstructionExecutor {
//         exec: |core: &mut Core, _: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fsqrt f{}, f{}", rd, rs1));
//             }
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value.sqrt());
//         },
//         name: "fsqrt",
//     };
//     map.insert((83, 0b000, 0b0101100), fsqrt);
//     let fsgnj = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             let mut rd_value = rs1_value;
//             if rs2_value.is_sign_negative() {
//                 rd_value = -rd_value;
//             }
//             core.set_float_register(rd as usize, rd_value);
//         },
//         name: "fsgnj",
//     };
//     map.insert((83, 0b000, 0b0010000), fsgnj);
//     let fsgnjn = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             let mut rd_value = rs1_value;
//             if !rs2_value.is_sign_negative() {
//                 rd_value = -rd_value;
//             }
//             core.set_float_register(rd as usize, rd_value);
//         },
//         name: "fsgnjn",
//     };
//     map.insert((83, 0b001, 0b0010000), fsgnjn);
//     let fsgnjx = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fsgnj f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             let mut rd_value = rs1_value;
//             if rs2_value.is_sign_negative() ^ rs1_value.is_sign_negative() {
//                 rd_value = -rd_value;
//             }
//             core.set_float_register(rd as usize, rd_value);
//         },
//         name: "fsgnjx",
//     };
//     map.insert((83, 0b010, 0b0010000), fsgnjx);
//     let fmin = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fmin f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             if rs1_value.is_nan() {
//                 core.set_float_register(rd as usize, rs1_value);
//             } else if rs2_value.is_nan() {
//                 core.set_float_register(rd as usize, rs2_value);
//             } else if rs1_value < rs2_value {
//                 core.set_float_register(rd as usize, rs1_value);
//             } else {
//                 core.set_float_register(rd as usize, rs2_value);
//             }
//         },
//         name: "fmin",
//     };
//     map.insert((83, 0b000, 0b0010100), fmin);
//     let fmax = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fmax f{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             if rs1_value.is_nan() {
//                 core.set_float_register(rd as usize, rs1_value);
//             } else if rs2_value.is_nan() {
//                 core.set_float_register(rd as usize, rs2_value);
//             } else if rs1_value > rs2_value {
//                 core.set_float_register(rd as usize, rs1_value);
//             } else {
//                 core.set_float_register(rd as usize, rs2_value);
//             }
//         },
//         name: "fmax",
//     };
//     map.insert((83, 0b001, 0b0010100), fmax);
//     let feq = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             // feq
//             if verbose {
//                 println_inst(&format!("feq x{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             if rs1_value == rs2_value {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "feq",
//     };
//     map.insert((83, 0b010, 0b1010000), feq);
//     let flt = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("flt x{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             if rs1_value < rs2_value {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "flt",
//     };
//     map.insert((83, 0b001, 0b1010000), flt);
//     let fle = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fle x{}, f{}, f{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_float_register(rs2 as usize);
//             let rs1_value = core.get_float_register(rs1 as usize);
//             if rs1_value <= rs2_value {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "fle",
//     };
//     map.insert((83, 0b000, 0b1010000), fle);
//     let fclass = RInstructionExecutor {
//         exec: |core: &mut Core, _: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fclass x{}, f{}", rd, rs1));
//             }
//             let rs1_value = core.get_float_register(rs1 as usize);
//             let mut rd_value = 0;
//             if rs1_value.is_nan() {
//                 if rd_value >> 30 & 1 == 0 {
//                     rd_value |= 0b0100000000; // signaling nan
//                 } else {
//                     rd_value |= 0b1000000000; // quiet nan
//                 }
//             } else if rs1_value == 0. {
//                 if rs1_value.is_sign_negative() {
//                     rd_value |= 0b0000001000;
//                 } else {
//                     rd_value |= 0b0000010000;
//                 }
//             } else if rs1_value.is_infinite() {
//                 if rs1_value.is_sign_negative() {
//                     rd_value |= 0b0000000001;
//                 } else {
//                     rd_value |= 0b0010000000;
//                 }
//             } else if rs1_value.is_normal() {
//                 if rs1_value.is_sign_negative() {
//                     rd_value |= 0b0000000010;
//                 } else {
//                     rd_value |= 0b0001000000;
//                 }
//             } else {
//                 if rs1_value.is_sign_negative() {
//                     rd_value |= 0b0000000100;
//                 } else {
//                     rd_value |= 0b0000100000;
//                 }
//             }
//             core.set_int_register(rd as usize, rd_value);
//         },
//         name: "fclass",
//     };
//     map.insert((83, 0b001, 0b1110000), fclass);

//     let fcvt_w_s = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(rs2, 0b00000);
//             if verbose {
//                 println_inst(&format!("fcvt.wu.s x{}, f{}", rd, rs1));
//             }
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value as i32);
//         },
//         name: "fcvt.w.s",
//     };
//     map.insert((83, 0b000, 0b1100000), fcvt_w_s);

//     let fcvt_wu_s = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(rs2, 0b00000);
//             if verbose {
//                 println_inst(&format!("fcvt.wu.s x{}, f{}", rd, rs1));
//             }
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value.abs() as i32);
//         },
//         name: "fcvt.wu.s",
//     };
//     map.insert((83, 0b000, 0b1100001), fcvt_wu_s);
//     let fcvt_s_w = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(rs2, 0b00000);
//             if verbose {
//                 println_inst(&format!("fcvt.s.w f{}, x{}", rd, rs1));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_float_register(rd as usize, rs1_value as f32);
//         },
//         name: "fcvt.s.w",
//     };
//     map.insert((83, 0b000, 0b1101000), fcvt_s_w);
//     let fcvt_s_wu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(rs2, 0b00000);
//             if verbose {
//                 println_inst(&format!("fcvt.s.wu f{}, x{}", rd, rs1));
//             }
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
//             core.set_float_register(rd as usize, rs1_value as f32);
//         },
//         name: "fcvt.s.wu",
//     };
//     map.insert((83, 0b000, 0b1101001), fcvt_s_wu);
//     let fmv_x_w = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| match rs2 {
//             0b00000 => {
//                 if verbose {
//                     println_inst(&format!("fmvs.x.w f{}, x{}", rd, rs1));
//                 }
//                 let rs1_value = core.get_int_register(rs1 as usize);
//                 core.set_float_register(rd as usize, rs1_value as f32);
//             }
//             _ => {
//                 println!("unexpected rs2: {}", rs2)
//             }
//         },
//         name: "fmv.x.w",
//     };
//     map.insert((83, 0b000, 0b1110000), fmv_x_w);
//     let fmv_x_w = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             assert_eq!(rs2, 0b00000);
//             if verbose {
//                 println_inst(&format!("fmv.w.x x{}, f{}", rd, rs1));
//             }
//             let rs1_value = core.get_float_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value as i32);
//         },
//         name: "fmv.x.w",
//     };
//     map.insert((83, 0b000, 0b1111000), fmv_x_w);
//     let swapw = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("swapw x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs2_value);
//             core.set_int_register(rs2 as usize, rs1_value);
//             core.set_int_register(rs1 as usize, rs2_value);
//         },
//         name: "swapw",
//     };
//     map.insert((52, 0b000, 0b0000000), swapw);
//     let swaph = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("swaph x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) & 0xffff;
//             let rs1_value = core.get_int_register(rs1 as usize) & 0xffff;
//             core.set_int_register(rd as usize, rs2_value);
//             core.set_int_register(rs2 as usize, rs1_value);
//             core.set_int_register(rs1 as usize, rs2_value);
//         },
//         name: "swaph",
//     };
//     map.insert((52, 0b001, 0b0000000), swaph);
//     let swapb = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("swapb x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) & 0xff;
//             let rs1_value = core.get_int_register(rs1 as usize) & 0xff;
//             core.set_int_register(rd as usize, rs2_value);
//             core.set_int_register(rs2 as usize, rs1_value);
//             core.set_int_register(rs1 as usize, rs2_value);
//         },
//         name: "swapb",
//     };
//     map.insert((52, 0b010, 0b0000000), swapb);
//     map
// }

// fn create_s_instruction_map() -> SInstructionMap {
//     let mut map = SInstructionMap::new();
//     let fsw = SInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("fsw f{}, {}(x{})", rs2, imm, rs1));
//             }
//             let value = core.get_float_register(rs2 as usize);
//             core.store_word(
//                 (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
//                 value.to_bits() as Word,
//             )
//         },
//         name: "fsw",
//     };
//     map.insert((39, 0b010), fsw);
//     map
// }

// fn create_r4_instruction_map() -> R4InstructionMap {
//     let mut map = R4InstructionMap::new();
//     let fmadd = R4InstructionExecutor {
//         exec: |core: &mut Core, fs3: u8, fs2: u8, fs1: u8, fd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
//             }
//             let fs1_value = core.get_float_register(fs1 as usize);
//             let fs2_value = core.get_float_register(fs2 as usize);
//             let fs3_value = core.get_float_register(fs3 as usize);
//             core.set_float_register(fd as usize, fs1_value * fs2_value + fs3_value);
//         },
//         name: "fmadd",
//     };
//     map.insert(67, fmadd);
//     let fmsub = R4InstructionExecutor {
//         exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
//             }
//             let fs1_value = core.get_float_register(fs1 as usize);
//             let fs2_value = core.get_float_register(fs2 as usize);
//             let fs3_value = core.get_float_register(fs3 as usize);
//             core.set_float_register(fd as usize, fs1_value * fs2_value - fs3_value);
//         },
//         name: "fmsub",
//     };
//     map.insert(71, fmsub);
//     let fnmsub = R4InstructionExecutor {
//         exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fnmsub f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
//             }
//             let fs1_value = core.get_float_register(fs1 as usize);
//             let fs2_value = core.get_float_register(fs2 as usize);
//             let fs3_value = core.get_float_register(fs3 as usize);
//             core.set_float_register(fd as usize, -(fs1_value * fs2_value) - fs3_value);
//         },
//         name: "fnmsub",
//     };
//     map.insert(75, fnmsub);
//     let fnmadd = R4InstructionExecutor {
//         exec: |core: &mut Core, fs1: u8, fs2: u8, fs3: u8, fd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("fnmadd f{}, f{}, f{}, f{}", fd, fs1, fs2, fs3));
//             }
//             let fs1_value = core.get_float_register(fs1 as usize);
//             let fs2_value = core.get_float_register(fs2 as usize);
//             let fs3_value = core.get_float_register(fs3 as usize);
//             core.set_float_register(fd as usize, -(fs1_value * fs2_value - fs3_value));
//         },
//         name: "fnmadd",
//     };
//     map.insert(79, fnmadd);
//     map
// }

// pub fn exec_instruction(core: &mut Core, inst: InstructionValue, verbose: bool) {
//     match decode_instruction(inst).0 {
//         Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
//             exec_i_instruction(core, imm, rs1, funct3, rd, op, verbose);
//             if op != 103 {
//                 core.increment_pc();
//             }
//         }
//         Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
//             exec_r_instruction(core, funct7, rs2, rs1, funct3, rd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
//             exec_s_instruction(core, imm, rs2, rs1, funct3, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::BInstruction(imm, rs2, rs1, funct3, op) => {
//             exec_b_instruction(core, imm, rs2, rs1, funct3, op, verbose);
//         }
//         Instruction::JInstruction(imm, rd, op) => {
//             exec_j_instruction(core, imm, rd, op, verbose);
//         }
//         Instruction::UInstruction(imm, rd, op) => {
//             exec_u_instruction(core, imm, rd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::R4Instruction(fs1, _, fs2, fs3, _, fd, op) => {
//             exec_r4_instruction(core, fs3, fs2, fs1, fd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::OtherInstruction => {
//             println!("other instruction {:>032b}", inst);
//         }
//     }
// }

// pub fn exec_decoded_instruction(core: &mut Core, decoded_inst: Instruction, verbose: bool) {
//     match decoded_inst {
//         Instruction::IInstruction(imm, rs1, funct3, rd, op) => {
//             exec_i_instruction(core, imm, rs1, funct3, rd, op, verbose);
//             if op != 103 {
//                 core.increment_pc();
//             }
//         }
//         Instruction::RInstruction(funct7, rs2, rs1, funct3, rd, op) => {
//             exec_r_instruction(core, funct7, rs2, rs1, funct3, rd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::SInstruction(imm, rs2, rs1, funct3, op) => {
//             exec_s_instruction(core, imm, rs2, rs1, funct3, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::BInstruction(imm, rs2, rs1, funct3, op) => {
//             exec_b_instruction(core, imm, rs2, rs1, funct3, op, verbose);
//         }
//         Instruction::JInstruction(imm, rd, op) => {
//             exec_j_instruction(core, imm, rd, op, verbose);
//         }
//         Instruction::UInstruction(imm, rd, op) => {
//             exec_u_instruction(core, imm, rd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::R4Instruction(fs1, _, fs2, fs3, _, fd, op) => {
//             exec_r4_instruction(core, fs3, fs2, fs1, fd, op, verbose);
//             core.increment_pc();
//         }
//         Instruction::OtherInstruction => {
//             println!("other instruction");
//         }
//     }
// }
