use std::fmt::Debug;

use crate::core::*;
use crate::decoder::*;
use crate::fpu_emulator::*;
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

trait InstructionTrait: Clone + Debug {
    fn register_fetch(&mut self, _: &Core) {}
    fn exec(&mut self, _: &mut Core) {}
    fn memory(&mut self, _: &mut Core) {}
    fn write_back(&self, _: &mut Core) {}
    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![]
    }
    fn get_destination_register(&self) -> Option<RegisterId> {
        None
    }
    fn is_load_instruction(&self) -> bool {
        false
    }
    fn get_jump_address(&self) -> Option<Address> {
        None
    }
    fn get_instruction_count(&self) -> Option<InstructionCount>;
    fn get_name(&self) -> String;
}

#[derive(Clone)]
struct IntIInstructionData {
    imm: Imm13,
    rs1: Rs1,
    rd: Rd,
    extended_imm: Option<i32>,
    rs1_value: Option<Int>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct FloatIInstructionData {
    imm: Imm13,
    rs1: Rs1,
    fd: Fd,
    extended_imm: Option<i32>,
    rs1_value: Option<Int>,
    fd_value: Option<FloatingPoint>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct IntSInstructionData {
    imm: Imm13,
    rs2: Rs2,
    rs1: Rs1,
    extended_imm: Option<i32>,
    rs2_value: Option<Int>,
    rs1_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct FloatSInstructionData {
    imm: Imm13,
    fs2: Fs2,
    rs1: Rs1,
    extended_imm: Option<i32>,
    fs2_value: Option<FloatingPoint>,
    rs1_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct IntRInstructionData {
    rs2: Rs2,
    rs1: Rs1,
    rd: Rd,
    rs2_value: Option<Int>,
    rs1_value: Option<Int>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct FloatRInstructionData {
    fs2: Fs2,
    fs1: Fs1,
    fd: Fd,
    rs2_value: Option<FloatingPoint>,
    rs1_value: Option<FloatingPoint>,
    rd_value: Option<FloatingPoint>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct FloatIntRInstructionData {
    fs2: Fs2,
    fs1: Fs1,
    rd: Rd,
    fs2_value: Option<FloatingPoint>,
    fs1_value: Option<FloatingPoint>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct IntFloatRInstructionData {
    _rs2: Rs2,
    rs1: Rs1,
    fd: Fd,
    _rs2_value: Option<FloatingPoint>,
    rs1_value: Option<Int>,
    fd_value: Option<FloatingPoint>,
    inst_count: Option<InstructionCount>,
}

#[derive(Clone)]
struct BInstructionData {
    imm: Imm13,
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
struct FBInstructionData {
    imm: Imm13,
    rs2: Rs2,
    rs1: Rs1,
    extended_imm: Option<i32>,
    rs2_value: Option<FloatingPoint>,
    rs1_value: Option<FloatingPoint>,
    inst_count: Option<InstructionCount>,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

#[derive(Clone)]
struct JInstructionData {
    imm: Imm19,
    rd: Rd,
    extended_imm: Option<i32>,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

#[derive(Clone)]
struct UInstructionData {
    imm: Imm19,
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
pub struct Lw {
    data: IntIInstructionData,
    addr: Option<Address>,
}

impl Lw {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "lw x{}, {}(x{})",
            self.data.rd, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Lw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
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
        core.set_forwarding_int_source(self.data.rd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, load_value as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
pub struct Addi {
    data: IntIInstructionData,
}

impl Addi {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "addi x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Addi {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value + extended_imm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
    uimm: Option<u32>,
}

impl Slli {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        let uimm = self.data.imm & 63;
        write!(f, "slli x{}, x{}, {}", self.data.rd, self.data.rs1, uimm)
    }
}

impl InstructionTrait for Slli {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.uimm = Some((self.data.imm & 63) as u32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(self.data.rs1_value.unwrap() << uimm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
}

impl Slti {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
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
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "slti".to_string()
    }
}

#[derive(Clone)]
pub struct Xori {
    data: IntIInstructionData,
}

impl Xori {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value ^ extended_imm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
    uimm: Option<u32>,
}

impl Srli {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        let uimm = self.data.imm & 63;
        write!(f, "srli x{}, x{}, {}", self.data.rd, self.data.rs1, uimm)
    }
}

impl InstructionTrait for Srli {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.uimm = Some((self.data.imm & 63) as u32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(u32_to_i32(i32_to_u32(self.data.rs1_value.unwrap()) >> uimm));
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
    uimm: Option<u32>,
}

impl Srai {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.uimm = Some((self.data.imm & 0x1f) as u32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let uimm = self.uimm.unwrap();
        self.data.rd_value = Some(self.data.rs1_value.unwrap() >> uimm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
}

impl Ori {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value | extended_imm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntIInstructionData,
}

impl Andi {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 12) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(rs1_value & extended_imm);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "andi".to_string()
    }
}

#[derive(Clone)]
pub struct Sw {
    data: IntSInstructionData,
    addr: Option<Address>,
}

impl Sw {
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
        let data = IntSInstructionData {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "sw x{}, {}(x{})",
            self.data.rs2, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Sw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
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
    data: IntRInstructionData,
}

impl Add {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value + rs2_value);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Sub {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value - rs2_value);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Sll {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value as Int);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Slt {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "slt".to_string()
    }
}

#[derive(Clone)]
pub struct Xor {
    data: IntRInstructionData,
}

impl Xor {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value ^ rs2_value);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Srl {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Sra {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl Or {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value | rs2_value);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    data: IntRInstructionData,
}

impl And {
    fn new(rs2: Rs2, rs1: Rs1, rd: Rd) -> Self {
        let data = IntRInstructionData {
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
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_int_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value & rs2_value);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    fn new(imm: Imm19, rd: Rd) -> Self {
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
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.upimm = Some(self.data.imm << 13);
        self.data.origin_pc = Some(core.get_pc() - 8);
    }

    fn exec(&mut self, core: &mut Core) {
        self.data.rd_value = Some(self.data.upimm.unwrap());
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "beq x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Beq {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
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
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "bne x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Bne {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
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
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "blt x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Blt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
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
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "bge x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Bge {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_int_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
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
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Int(self.data.rs2),
        ]
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
pub struct Jalr {
    data: IntIInstructionData,
    origin_pc: Option<Address>,
    jump_address: Option<Address>,
}

impl Jalr {
    fn new(imm: Imm13, rs1: Rs1, rd: Rd) -> Self {
        let data = IntIInstructionData {
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
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "jalr x{}, x{}, {}",
            self.data.rd, self.data.rs1, extended_imm
        )
    }
}

impl InstructionTrait for Jalr {
    fn register_fetch(&mut self, core: &Core) {
        self.origin_pc = Some(core.get_pc() - 8);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.jump_address = Some((rs1_value + (extended_imm << 2)) as Address);
        self.data.rd_value = Some(self.origin_pc.unwrap() as i32 + 4);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
    fn new(imm: Imm19, rd: Rd) -> Self {
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
        let extended_imm = sign_extention_i32(self.data.imm, 19);
        write!(f, "jal x{}, {}", self.data.rd, extended_imm)
    }
}

impl InstructionTrait for Jal {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i32(self.data.imm, 19));
        self.data.origin_pc = Some(core.get_pc() - 8);
        self.data.inst_count = Some(core.get_cycle_count());
    }

    fn exec(&mut self, core: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        self.data.jump_address =
            Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        self.data.rd_value = Some(self.data.origin_pc.unwrap() as i32 + 4);
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result as Int);
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
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
pub struct Flw {
    data: FloatIInstructionData,
    addr: Option<Address>,
}

impl Flw {
    fn new(imm: Imm13, rs1: Rs1, fd: Fd) -> Self {
        let data = FloatIInstructionData {
            imm,
            rs1,
            fd,
            extended_imm: None,
            rs1_value: None,
            fd_value: None,
            inst_count: None,
        };
        Flw { data, addr: None }
    }
}

impl Debug for Flw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "flw f{}, {}(x{})",
            self.data.fd, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Flw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source {
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
        let value = FloatingPoint::new(i32_to_u32(core.load_word(addr)));
        self.data.fd_value = Some(value);
        core.set_forwarding_float_source(self.data.fd, self.data.inst_count.unwrap(), value);
    }

    fn write_back(&self, core: &mut Core) {
        let load_value = self.data.fd_value.unwrap();
        core.set_float_register(self.data.fd as usize, load_value as FloatingPoint);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "flw".to_string()
    }
}

#[derive(Clone)]
pub struct Fadd {
    data: FloatRInstructionData,
}

impl Fadd {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fadd { data }
    }
}

impl Debug for Fadd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fadd f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fadd {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value + rs2_value);
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fadd".to_string()
    }
}

#[derive(Clone)]
pub struct Fsub {
    data: FloatRInstructionData,
}

impl Fsub {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fsub { data }
    }
}

impl Debug for Fsub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fsub f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fsub {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value - rs2_value);
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsub".to_string()
    }
}

#[derive(Clone)]
pub struct Fmul {
    data: FloatRInstructionData,
}

impl Fmul {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fmul { data }
    }
}

impl Debug for Fmul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fmul f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fmul {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(rs1_value * rs2_value);
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fmul".to_string()
    }
}

#[derive(Clone)]
pub struct Fdiv {
    data: FloatRInstructionData,
}

impl Fdiv {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fdiv { data }
    }
}

impl Debug for Fdiv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fdiv f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fdiv {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(div_fp(rs1_value, rs2_value, core.get_inv_map()));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fdiv".to_string()
    }
}

#[derive(Clone)]
pub struct Fsqrt {
    data: FloatRInstructionData,
}

impl Fsqrt {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fsqrt { data }
    }
}

impl Debug for Fsqrt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fsqrt f{}, f{}", self.data.fd, self.data.fs1)
    }
}

impl InstructionTrait for Fsqrt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.rd_value = Some(sqrt_fp(rs1_value, core.get_sqrt_map()));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Float(self.data.fs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsqrt".to_string()
    }
}

#[derive(Clone)]
pub struct Fsgnj {
    data: FloatRInstructionData,
}

impl Fsgnj {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fsgnj { data }
    }
}

impl Debug for Fsgnj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fsgnj f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fsgnj {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(fp_sign_injection(rs1_value, rs2_value));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsgnj".to_string()
    }
}

#[derive(Clone)]
pub struct Fsgnjn {
    data: FloatRInstructionData,
}

impl Fsgnjn {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fsgnjn { data }
    }
}

impl Debug for Fsgnjn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fsgnjn f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fsgnjn {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(fp_negative_sign_injection(rs1_value, rs2_value));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsgnjn".to_string()
    }
}

#[derive(Clone)]
pub struct Fsgnjx {
    data: FloatRInstructionData,
}

impl Fsgnjx {
    fn new(fs2: Rs2, fs1: Rs1, fd: Rd) -> Self {
        let data = FloatRInstructionData {
            fs2,
            fs1,
            fd,
            rs2_value: None,
            rs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fsgnjx { data }
    }
}

impl Debug for Fsgnjx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fsgnjx f{}, f{}, f{}",
            self.data.fd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fsgnjx {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        self.data.rd_value = Some(fp_xor_sign_injection(rs1_value, rs2_value));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsgnjx".to_string()
    }
}

#[derive(Clone)]
pub struct Feq {
    data: FloatIntRInstructionData,
}

impl Feq {
    fn new(fs2: Rs2, fs1: Rs1, rd: Rd) -> Self {
        let data = FloatIntRInstructionData {
            fs2,
            fs1,
            rd,
            fs2_value: None,
            fs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Feq { data }
    }
}

impl Debug for Feq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "feq x{}, f{}, f{}",
            self.data.rd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Feq {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.fs1_value = Some(*rs1_value);
        } else {
            self.data.fs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.fs2_value = Some(*rs2_value);
        } else {
            self.data.fs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.fs1_value.unwrap();
        let rs2_value = self.data.fs2_value.unwrap();
        self.data.rd_value = Some(if rs1_value == rs2_value { 1 } else { 0 });
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "feq".to_string()
    }
}

impl Flt {
    fn new(fs2: Rs2, fs1: Rs1, rd: Rd) -> Self {
        let data = FloatIntRInstructionData {
            fs2,
            fs1,
            rd,
            fs2_value: None,
            fs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Flt { data }
    }
}

#[derive(Clone)]
pub struct Flt {
    data: FloatIntRInstructionData,
}

impl Debug for Flt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "flt x{}, f{}, f{}",
            self.data.rd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Flt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.fs1_value = Some(*rs1_value);
        } else {
            self.data.fs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.fs2_value = Some(*rs2_value);
        } else {
            self.data.fs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.fs1_value.unwrap();
        let rs2_value = self.data.fs2_value.unwrap();
        self.data.rd_value = Some(if rs1_value < rs2_value { 1 } else { 0 });
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "flt".to_string()
    }
}

#[derive(Clone)]
pub struct Fle {
    data: FloatIntRInstructionData,
}

impl Fle {
    fn new(fs2: Rs2, fs1: Rs1, rd: Rd) -> Self {
        let data = FloatIntRInstructionData {
            fs2,
            fs1,
            rd,
            fs2_value: None,
            fs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        Fle { data }
    }
}

impl Debug for Fle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fle x{}, f{}, f{}",
            self.data.rd, self.data.fs1, self.data.fs2
        )
    }
}

impl InstructionTrait for Fle {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.fs1_value = Some(*rs1_value);
        } else {
            self.data.fs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.fs2_value = Some(*rs2_value);
        } else {
            self.data.fs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.fs1_value.unwrap();
        let rs2_value = self.data.fs2_value.unwrap();
        // eprintln!(
        //     "rs1: {}({}), rs2: {}({})",
        //     f32::from_bits(rs1_value.get_32_bits()),
        //     rs1_value.get_32_bits(),
        //     f32::from_bits(rs2_value.get_32_bits()),
        //     rs2_value.get_32_bits()
        // );
        self.data.rd_value = Some(if rs1_value <= rs2_value { 1 } else { 0 });
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.fs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fle".to_string()
    }
}

#[derive(Clone)]
pub struct FcvtWS {
    data: FloatIntRInstructionData,
}

impl FcvtWS {
    fn new(fs2: Rs2, fs1: Rs1, rd: Rd) -> Self {
        let data = FloatIntRInstructionData {
            fs2,
            fs1,
            rd,
            fs2_value: None,
            fs1_value: None,
            rd_value: None,
            inst_count: None,
        };
        FcvtWS { data }
    }
}

impl Debug for FcvtWS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.w.s x{}, f{}", self.data.rd, self.data.fs1)
    }
}

impl InstructionTrait for FcvtWS {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.fs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.fs1_value = Some(*rs1_value);
        } else {
            self.data.fs1_value = Some(core.get_float_register(self.data.fs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.fs1_value.unwrap();
        self.data.rd_value = Some(fp_to_int(rs1_value));
        core.set_forwarding_int_source(
            self.data.rd,
            self.data.inst_count.unwrap(),
            self.data.rd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.rd_value.unwrap();
        core.set_int_register(self.data.rd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Float(self.data.fs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.data.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fcvt.w.s".to_string()
    }
}

#[derive(Clone)]
pub struct FcvtSW {
    data: IntFloatRInstructionData,
}

impl FcvtSW {
    fn new(rs2: Rs2, rs1: Rs1, fd: Fd) -> Self {
        let data = IntFloatRInstructionData {
            _rs2: rs2,
            rs1,
            fd,
            _rs2_value: None,
            rs1_value: None,
            fd_value: None,
            inst_count: None,
        };
        FcvtSW { data }
    }
}

impl Debug for FcvtSW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fcvt.s.w f{}, x{}", self.data.fd, self.data.rs1)
    }
}

impl InstructionTrait for FcvtSW {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let rs1_value = self.data.rs1_value.unwrap();
        self.data.fd_value = Some(int_to_fp(rs1_value));
        core.set_forwarding_float_source(
            self.data.fd,
            self.data.inst_count.unwrap(),
            self.data.fd_value.unwrap(),
        );
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.data.fd_value.unwrap();
        core.set_float_register(self.data.fd as usize, result);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.data.rs1)]
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.data.fd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fcvt.s.w".to_string()
    }
}

#[derive(Clone)]
pub struct In {
    rd: Rd,
    rd_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

impl In {
    fn new(rd: Rd) -> Self {
        In {
            rd,
            rd_value: None,
            inst_count: None,
        }
    }
}

impl Debug for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "in x{}", self.rd)
    }
}

impl InstructionTrait for In {
    fn register_fetch(&mut self, core: &Core) {
        self.inst_count = Some(core.get_cycle_count());
    }

    fn exec(&mut self, core: &mut Core) {
        self.rd_value = Some(core.read_int());
        core.set_forwarding_int_source(self.rd, self.inst_count.unwrap(), self.rd_value.unwrap());
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.rd_value.unwrap();
        core.set_int_register(self.rd as usize, result);
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Int(self.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.inst_count
    }

    fn get_name(&self) -> String {
        "in".to_string()
    }
}

#[derive(Clone)]
pub struct Fin {
    rd: Rd,
    rd_value: Option<FloatingPoint>,
    inst_count: Option<InstructionCount>,
}

impl Fin {
    fn new(rd: Rd) -> Self {
        Fin {
            rd,
            rd_value: None,
            inst_count: None,
        }
    }
}

impl Debug for Fin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "in x{}", self.rd)
    }
}

impl InstructionTrait for Fin {
    fn register_fetch(&mut self, core: &Core) {
        self.inst_count = Some(core.get_cycle_count());
    }

    fn exec(&mut self, core: &mut Core) {
        let value = core.read_float();
        self.rd_value = Some(FloatingPoint::new(i32_to_u32(value)));
        core.set_forwarding_float_source(self.rd, self.inst_count.unwrap(), self.rd_value.unwrap());
    }

    fn write_back(&self, core: &mut Core) {
        let result = self.rd_value.unwrap();
        core.set_float_register(self.rd as usize, result);
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        Some(RegisterId::Float(self.rd))
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.inst_count
    }

    fn get_name(&self) -> String {
        "fin".to_string()
    }
}

#[derive(Clone)]
pub struct Fsw {
    data: FloatSInstructionData,
    addr: Option<Address>,
}

impl Fsw {
    fn new(imm: Imm13, fs2: Rs2, rs1: Rs1) -> Self {
        let data = FloatSInstructionData {
            imm,
            fs2,
            rs1,
            extended_imm: None,
            fs2_value: None,
            rs1_value: None,
            inst_count: None,
        };
        Fsw { data, addr: None }
    }
}

impl Debug for Fsw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        write!(
            f,
            "fsw f{}, {}(x{})",
            self.data.fs2, extended_imm, self.data.rs1
        )
    }
}

impl InstructionTrait for Fsw {
    fn register_fetch(&mut self, core: &Core) {
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        let forwarding_source_1 = core.get_forwarding_int_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_int_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.fs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.fs2_value = Some(*rs2_value);
        } else {
            self.data.fs2_value = Some(core.get_float_register(self.data.fs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        self.addr = Some((rs1_value + extended_imm) as Address);
    }

    fn memory(&mut self, core: &mut Core) {
        let addr = self.addr.unwrap();
        core.store_word(addr, u32_to_i32(self.data.fs2_value.unwrap().get_32_bits()));
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Int(self.data.rs1),
            RegisterId::Float(self.data.fs2),
        ]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fsw".to_string()
    }
}

#[derive(Clone)]
pub struct Outchar {
    rs2: Rs1,
    rs2_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

impl Outchar {
    fn new(rs2: Rs2) -> Self {
        Outchar {
            rs2,
            rs2_value: None,
            inst_count: None,
        }
    }
}

impl Debug for Outchar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "outchar x{}", self.rs2)
    }
}

impl InstructionTrait for Outchar {
    fn register_fetch(&mut self, core: &Core) {
        self.inst_count = Some(core.get_cycle_count());
        let forwarding_source = core.get_forwarding_int_source(self.rs2);
        if let Some((_, rs2_value)) = forwarding_source {
            self.rs2_value = Some(*rs2_value);
        } else {
            self.rs2_value = Some(core.get_int_register(self.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let value = self.rs2_value.unwrap();
        core.print_char(value);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.rs2)]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.inst_count
    }

    fn get_name(&self) -> String {
        "outchar".to_string()
    }
}

#[derive(Clone)]
pub struct Outint {
    rs2: Rs1,
    rs2_value: Option<Int>,
    inst_count: Option<InstructionCount>,
}

impl Outint {
    fn new(rs2: Rs2) -> Self {
        Outint {
            rs2,
            rs2_value: None,
            inst_count: None,
        }
    }
}

impl Debug for Outint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "outint x{}", self.rs2)
    }
}

impl InstructionTrait for Outint {
    fn register_fetch(&mut self, core: &Core) {
        self.inst_count = Some(core.get_cycle_count());
        let forwarding_source = core.get_forwarding_int_source(self.rs2);
        if let Some((_, rs2_value)) = forwarding_source {
            self.rs2_value = Some(*rs2_value);
        } else {
            self.rs2_value = Some(core.get_int_register(self.rs2 as usize));
        }
    }

    fn exec(&mut self, core: &mut Core) {
        let value = self.rs2_value.unwrap();
        core.print_int(value);
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![RegisterId::Int(self.rs2)]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.inst_count
    }

    fn get_name(&self) -> String {
        "outint".to_string()
    }
}

#[derive(Clone)]
pub struct Fbeq {
    data: FBInstructionData,
}

impl Fbeq {
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
        let data = FBInstructionData {
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
        Fbeq { data }
    }
}

impl Debug for Fbeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "fbeq x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Fbeq {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value == rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.rs1),
            RegisterId::Float(self.data.rs2),
        ]
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fbeq".to_string()
    }
}

#[derive(Clone)]
pub struct Fbne {
    data: FBInstructionData,
}

impl Fbne {
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
        let data = FBInstructionData {
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
        Fbne { data }
    }
}

impl Debug for Fbne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "fbne x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Fbne {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value != rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.rs1),
            RegisterId::Float(self.data.rs2),
        ]
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fbne".to_string()
    }
}

#[derive(Clone)]
pub struct Fblt {
    data: FBInstructionData,
}

impl Fblt {
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
        let data = FBInstructionData {
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
        Fblt { data }
    }
}

impl Debug for Fblt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "fblt x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Fblt {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value < rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.rs1),
            RegisterId::Float(self.data.rs2),
        ]
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fblt".to_string()
    }
}

#[derive(Clone)]
pub struct Fble {
    data: FBInstructionData,
}

impl Fble {
    fn new(imm: Imm13, rs2: Rs2, rs1: Rs1) -> Self {
        let data = FBInstructionData {
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
        Fble { data }
    }
}

impl Debug for Fble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extended_imm = sign_extention_i16(self.data.imm, 13);
        let origin_pc = {
            if let Some(pc) = self.data.origin_pc {
                pc.to_string()
            } else {
                "?".to_string()
            }
        };
        write!(
            f,
            "fble x{}, x{}, {} + {}",
            self.data.rs1,
            self.data.rs2,
            origin_pc,
            extended_imm << 2
        )
    }
}

impl InstructionTrait for Fble {
    fn register_fetch(&mut self, core: &Core) {
        self.data.extended_imm = Some(sign_extention_i16(self.data.imm, 13) as i32);
        self.data.inst_count = Some(core.get_cycle_count());
        self.data.origin_pc = Some(core.get_pc() - 8);
        let forwarding_source_1 = core.get_forwarding_float_source(self.data.rs1);
        if let Some((_, rs1_value)) = forwarding_source_1 {
            self.data.rs1_value = Some(*rs1_value);
        } else {
            self.data.rs1_value = Some(core.get_float_register(self.data.rs1 as usize));
        }
        let forwarding_source_2 = core.get_forwarding_float_source(self.data.rs2);
        if let Some((_, rs2_value)) = forwarding_source_2 {
            self.data.rs2_value = Some(*rs2_value);
        } else {
            self.data.rs2_value = Some(core.get_float_register(self.data.rs2 as usize));
        }
    }

    fn exec(&mut self, _: &mut Core) {
        let extended_imm = self.data.extended_imm.unwrap();
        let rs1_value = self.data.rs1_value.unwrap();
        let rs2_value = self.data.rs2_value.unwrap();
        if rs1_value <= rs2_value {
            self.data.jump_address =
                Some((self.data.origin_pc.unwrap() as i32 + (extended_imm << 2)) as Address);
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        vec![
            RegisterId::Float(self.data.rs1),
            RegisterId::Float(self.data.rs2),
        ]
    }

    fn get_jump_address(&self) -> Option<Address> {
        self.data.jump_address
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
    }

    fn get_name(&self) -> String {
        "fble".to_string()
    }
}

#[derive(Clone)]
pub struct End {
    inst_count: Option<InstructionCount>,
}

impl End {
    fn new() -> Self {
        End { inst_count: None }
    }
}

impl Debug for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "end")
    }
}

impl InstructionTrait for End {
    fn register_fetch(&mut self, core: &Core) {
        self.inst_count = Some(core.get_cycle_count());
    }

    fn write_back(&self, core: &mut Core) {
        core.end();
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.inst_count
    }

    fn get_name(&self) -> String {
        "end".to_string()
    }
}

#[derive(Clone)]
pub enum InstructionEnum {
    Lw(Lw),
    Addi(Addi),
    Slli(Slli),
    Slti(Slti),
    Xori(Xori),
    Srli(Srli),
    Srai(Srai),
    Ori(Ori),
    Andi(Andi),
    Sw(Sw),
    Add(Add),
    Sub(Sub),
    Sll(Sll),
    Slt(Slt),
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
    Jalr(Jalr),
    Jal(Jal),
    Flw(Flw),
    Fadd(Fadd),
    Fsub(Fsub),
    Fmul(Fmul),
    Fdiv(Fdiv),
    Fsqrt(Fsqrt),
    Fsgnj(Fsgnj),
    Fsgnjn(Fsgnjn),
    Fsgnjx(Fsgnjx),
    Feq(Feq),
    Flt(Flt),
    Fle(Fle),
    FcvtWS(FcvtWS),
    FcvtSW(FcvtSW),
    Fsw(Fsw),
    In(In),
    Fin(Fin),
    Outchar(Outchar),
    Outint(Outint),
    Fbeq(Fbeq),
    Fbne(Fbne),
    Fblt(Fblt),
    Fble(Fble),
    End(End),
}

impl Debug for InstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionEnum::Lw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Addi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slli(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slti(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Xori(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Srli(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Srai(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Ori(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Andi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Add(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sub(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sll(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slt(instruction) => write!(f, "{:?}", instruction),
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
            InstructionEnum::Jalr(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jal(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Flw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fadd(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsub(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fmul(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fdiv(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsqrt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsgnj(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsgnjn(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsgnjx(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Feq(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Flt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fle(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::FcvtWS(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::FcvtSW(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fsw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::In(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fin(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Outchar(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Outint(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fbeq(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fbne(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fblt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Fble(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::End(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for InstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            InstructionEnum::Lw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Addi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slli(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slti(instruction) => instruction.register_fetch(core),
            InstructionEnum::Xori(instruction) => instruction.register_fetch(core),
            InstructionEnum::Srli(instruction) => instruction.register_fetch(core),
            InstructionEnum::Srai(instruction) => instruction.register_fetch(core),
            InstructionEnum::Ori(instruction) => instruction.register_fetch(core),
            InstructionEnum::Andi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Add(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sub(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sll(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slt(instruction) => instruction.register_fetch(core),
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
            InstructionEnum::Jalr(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jal(instruction) => instruction.register_fetch(core),
            InstructionEnum::Flw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fadd(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsub(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fmul(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fdiv(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsqrt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsgnj(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsgnjn(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsgnjx(instruction) => instruction.register_fetch(core),
            InstructionEnum::Feq(instruction) => instruction.register_fetch(core),
            InstructionEnum::Flt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fle(instruction) => instruction.register_fetch(core),
            InstructionEnum::FcvtWS(instruction) => instruction.register_fetch(core),
            InstructionEnum::FcvtSW(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fsw(instruction) => instruction.register_fetch(core),
            InstructionEnum::In(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fin(instruction) => instruction.register_fetch(core),
            InstructionEnum::Outchar(instruction) => instruction.register_fetch(core),
            InstructionEnum::Outint(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fbeq(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fbne(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fblt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Fble(instruction) => instruction.register_fetch(core),
            InstructionEnum::End(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lw(instruction) => instruction.exec(core),
            InstructionEnum::Addi(instruction) => instruction.exec(core),
            InstructionEnum::Slli(instruction) => instruction.exec(core),
            InstructionEnum::Slti(instruction) => instruction.exec(core),
            InstructionEnum::Xori(instruction) => instruction.exec(core),
            InstructionEnum::Srli(instruction) => instruction.exec(core),
            InstructionEnum::Srai(instruction) => instruction.exec(core),
            InstructionEnum::Ori(instruction) => instruction.exec(core),
            InstructionEnum::Andi(instruction) => instruction.exec(core),
            InstructionEnum::Sw(instruction) => instruction.exec(core),
            InstructionEnum::Add(instruction) => instruction.exec(core),
            InstructionEnum::Sub(instruction) => instruction.exec(core),
            InstructionEnum::Sll(instruction) => instruction.exec(core),
            InstructionEnum::Slt(instruction) => instruction.exec(core),
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
            InstructionEnum::Jalr(instruction) => instruction.exec(core),
            InstructionEnum::Jal(instruction) => instruction.exec(core),
            InstructionEnum::Flw(instruction) => instruction.exec(core),
            InstructionEnum::Fadd(instruction) => instruction.exec(core),
            InstructionEnum::Fsub(instruction) => instruction.exec(core),
            InstructionEnum::Fmul(instruction) => instruction.exec(core),
            InstructionEnum::Fdiv(instruction) => instruction.exec(core),
            InstructionEnum::Fsqrt(instruction) => instruction.exec(core),
            InstructionEnum::Fsgnj(instruction) => instruction.exec(core),
            InstructionEnum::Fsgnjn(instruction) => instruction.exec(core),
            InstructionEnum::Fsgnjx(instruction) => instruction.exec(core),
            InstructionEnum::Feq(instruction) => instruction.exec(core),
            InstructionEnum::Flt(instruction) => instruction.exec(core),
            InstructionEnum::Fle(instruction) => instruction.exec(core),
            InstructionEnum::FcvtWS(instruction) => instruction.exec(core),
            InstructionEnum::FcvtSW(instruction) => instruction.exec(core),
            InstructionEnum::Fsw(instruction) => instruction.exec(core),
            InstructionEnum::In(instruction) => instruction.exec(core),
            InstructionEnum::Fin(instruction) => instruction.exec(core),
            InstructionEnum::Outchar(instruction) => instruction.exec(core),
            InstructionEnum::Outint(instruction) => instruction.exec(core),
            InstructionEnum::Fbeq(instruction) => instruction.exec(core),
            InstructionEnum::Fbne(instruction) => instruction.exec(core),
            InstructionEnum::Fblt(instruction) => instruction.exec(core),
            InstructionEnum::Fble(instruction) => instruction.exec(core),
            InstructionEnum::End(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lw(instruction) => instruction.memory(core),
            InstructionEnum::Addi(instruction) => instruction.memory(core),
            InstructionEnum::Slli(instruction) => instruction.memory(core),
            InstructionEnum::Slti(instruction) => instruction.memory(core),
            InstructionEnum::Xori(instruction) => instruction.memory(core),
            InstructionEnum::Srli(instruction) => instruction.memory(core),
            InstructionEnum::Srai(instruction) => instruction.memory(core),
            InstructionEnum::Ori(instruction) => instruction.memory(core),
            InstructionEnum::Andi(instruction) => instruction.memory(core),
            InstructionEnum::Sw(instruction) => instruction.memory(core),
            InstructionEnum::Add(instruction) => instruction.memory(core),
            InstructionEnum::Sub(instruction) => instruction.memory(core),
            InstructionEnum::Sll(instruction) => instruction.memory(core),
            InstructionEnum::Slt(instruction) => instruction.memory(core),
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
            InstructionEnum::Jalr(instruction) => instruction.memory(core),
            InstructionEnum::Jal(instruction) => instruction.memory(core),
            InstructionEnum::Flw(instruction) => instruction.memory(core),
            InstructionEnum::Fadd(instruction) => instruction.memory(core),
            InstructionEnum::Fsub(instruction) => instruction.memory(core),
            InstructionEnum::Fmul(instruction) => instruction.memory(core),
            InstructionEnum::Fdiv(instruction) => instruction.memory(core),
            InstructionEnum::Fsqrt(instruction) => instruction.memory(core),
            InstructionEnum::Fsgnj(instruction) => instruction.memory(core),
            InstructionEnum::Fsgnjn(instruction) => instruction.memory(core),
            InstructionEnum::Fsgnjx(instruction) => instruction.memory(core),
            InstructionEnum::Feq(instruction) => instruction.memory(core),
            InstructionEnum::Flt(instruction) => instruction.memory(core),
            InstructionEnum::Fle(instruction) => instruction.memory(core),
            InstructionEnum::FcvtWS(instruction) => instruction.memory(core),
            InstructionEnum::FcvtSW(instruction) => instruction.memory(core),
            InstructionEnum::Fsw(instruction) => instruction.memory(core),
            InstructionEnum::In(instruction) => instruction.memory(core),
            InstructionEnum::Fin(instruction) => instruction.memory(core),
            InstructionEnum::Outchar(instruction) => instruction.memory(core),
            InstructionEnum::Outint(instruction) => instruction.memory(core),
            InstructionEnum::Fbeq(instruction) => instruction.memory(core),
            InstructionEnum::Fbne(instruction) => instruction.memory(core),
            InstructionEnum::Fblt(instruction) => instruction.memory(core),
            InstructionEnum::Fble(instruction) => instruction.memory(core),
            InstructionEnum::End(instruction) => instruction.memory(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            InstructionEnum::Lw(instruction) => instruction.write_back(core),
            InstructionEnum::Addi(instruction) => instruction.write_back(core),
            InstructionEnum::Slli(instruction) => instruction.write_back(core),
            InstructionEnum::Slti(instruction) => instruction.write_back(core),
            InstructionEnum::Xori(instruction) => instruction.write_back(core),
            InstructionEnum::Srli(instruction) => instruction.write_back(core),
            InstructionEnum::Srai(instruction) => instruction.write_back(core),
            InstructionEnum::Ori(instruction) => instruction.write_back(core),
            InstructionEnum::Andi(instruction) => instruction.write_back(core),
            InstructionEnum::Sw(instruction) => instruction.write_back(core),
            InstructionEnum::Add(instruction) => instruction.write_back(core),
            InstructionEnum::Sub(instruction) => instruction.write_back(core),
            InstructionEnum::Sll(instruction) => instruction.write_back(core),
            InstructionEnum::Slt(instruction) => instruction.write_back(core),
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
            InstructionEnum::Jalr(instruction) => instruction.write_back(core),
            InstructionEnum::Jal(instruction) => instruction.write_back(core),
            InstructionEnum::Flw(instruction) => instruction.write_back(core),
            InstructionEnum::Fadd(instruction) => instruction.write_back(core),
            InstructionEnum::Fsub(instruction) => instruction.write_back(core),
            InstructionEnum::Fmul(instruction) => instruction.write_back(core),
            InstructionEnum::Fdiv(instruction) => instruction.write_back(core),
            InstructionEnum::Fsqrt(instruction) => instruction.write_back(core),
            InstructionEnum::Fsgnj(instruction) => instruction.write_back(core),
            InstructionEnum::Fsgnjn(instruction) => instruction.write_back(core),
            InstructionEnum::Fsgnjx(instruction) => instruction.write_back(core),
            InstructionEnum::Feq(instruction) => instruction.write_back(core),
            InstructionEnum::Flt(instruction) => instruction.write_back(core),
            InstructionEnum::Fle(instruction) => instruction.write_back(core),
            InstructionEnum::FcvtWS(instruction) => instruction.write_back(core),
            InstructionEnum::FcvtSW(instruction) => instruction.write_back(core),
            InstructionEnum::Fsw(instruction) => instruction.write_back(core),
            InstructionEnum::In(instruction) => instruction.write_back(core),
            InstructionEnum::Fin(instruction) => instruction.write_back(core),
            InstructionEnum::Outchar(instruction) => instruction.write_back(core),
            InstructionEnum::Outint(instruction) => instruction.write_back(core),
            InstructionEnum::Fbeq(instruction) => instruction.write_back(core),
            InstructionEnum::Fbne(instruction) => instruction.write_back(core),
            InstructionEnum::Fblt(instruction) => instruction.write_back(core),
            InstructionEnum::Fble(instruction) => instruction.write_back(core),
            InstructionEnum::End(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<RegisterId> {
        match self {
            InstructionEnum::Lw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Addi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slli(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slti(instruction) => instruction.get_source_registers(),
            InstructionEnum::Xori(instruction) => instruction.get_source_registers(),
            InstructionEnum::Srli(instruction) => instruction.get_source_registers(),
            InstructionEnum::Srai(instruction) => instruction.get_source_registers(),
            InstructionEnum::Ori(instruction) => instruction.get_source_registers(),
            InstructionEnum::Andi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Add(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sub(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sll(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slt(instruction) => instruction.get_source_registers(),
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
            InstructionEnum::Jalr(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jal(instruction) => instruction.get_source_registers(),
            InstructionEnum::Flw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fadd(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsub(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fmul(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fdiv(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsqrt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsgnj(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsgnjn(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsgnjx(instruction) => instruction.get_source_registers(),
            InstructionEnum::Feq(instruction) => instruction.get_source_registers(),
            InstructionEnum::Flt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fle(instruction) => instruction.get_source_registers(),
            InstructionEnum::FcvtWS(instruction) => instruction.get_source_registers(),
            InstructionEnum::FcvtSW(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fsw(instruction) => instruction.get_source_registers(),
            InstructionEnum::In(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fin(instruction) => instruction.get_source_registers(),
            InstructionEnum::Outchar(instruction) => instruction.get_source_registers(),
            InstructionEnum::Outint(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fbeq(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fbne(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fblt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Fble(instruction) => instruction.get_source_registers(),
            InstructionEnum::End(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<RegisterId> {
        match self {
            InstructionEnum::Lw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Addi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slli(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slti(instruction) => instruction.get_destination_register(),
            InstructionEnum::Xori(instruction) => instruction.get_destination_register(),
            InstructionEnum::Srli(instruction) => instruction.get_destination_register(),
            InstructionEnum::Srai(instruction) => instruction.get_destination_register(),
            InstructionEnum::Ori(instruction) => instruction.get_destination_register(),
            InstructionEnum::Andi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Add(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sub(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sll(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slt(instruction) => instruction.get_destination_register(),
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
            InstructionEnum::Jalr(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jal(instruction) => instruction.get_destination_register(),
            InstructionEnum::Flw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fadd(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsub(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fmul(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fdiv(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsqrt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsgnj(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsgnjn(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsgnjx(instruction) => instruction.get_destination_register(),
            InstructionEnum::Feq(instruction) => instruction.get_destination_register(),
            InstructionEnum::Flt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fle(instruction) => instruction.get_destination_register(),
            InstructionEnum::FcvtWS(instruction) => instruction.get_destination_register(),
            InstructionEnum::FcvtSW(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fsw(instruction) => instruction.get_destination_register(),
            InstructionEnum::In(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fin(instruction) => instruction.get_destination_register(),
            InstructionEnum::Outchar(instruction) => instruction.get_destination_register(),
            InstructionEnum::Outint(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fbeq(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fbne(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fblt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Fble(instruction) => instruction.get_destination_register(),
            InstructionEnum::End(instruction) => instruction.get_destination_register(),
        }
    }

    fn is_load_instruction(&self) -> bool {
        matches!(self, |InstructionEnum::Lw(_)| InstructionEnum::Flw(_))
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            InstructionEnum::Lw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Addi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slli(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slti(instruction) => instruction.get_jump_address(),
            InstructionEnum::Xori(instruction) => instruction.get_jump_address(),
            InstructionEnum::Srli(instruction) => instruction.get_jump_address(),
            InstructionEnum::Srai(instruction) => instruction.get_jump_address(),
            InstructionEnum::Ori(instruction) => instruction.get_jump_address(),
            InstructionEnum::Andi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Add(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sub(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sll(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slt(instruction) => instruction.get_jump_address(),
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
            InstructionEnum::Jalr(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jal(instruction) => instruction.get_jump_address(),
            InstructionEnum::Flw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fadd(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsub(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fmul(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fdiv(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsqrt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsgnj(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsgnjn(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsgnjx(instruction) => instruction.get_jump_address(),
            InstructionEnum::Feq(instruction) => instruction.get_jump_address(),
            InstructionEnum::Flt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fle(instruction) => instruction.get_jump_address(),
            InstructionEnum::FcvtWS(instruction) => instruction.get_jump_address(),
            InstructionEnum::FcvtSW(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fsw(instruction) => instruction.get_jump_address(),
            InstructionEnum::In(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fin(instruction) => instruction.get_jump_address(),
            InstructionEnum::Outchar(instruction) => instruction.get_jump_address(),
            InstructionEnum::Outint(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fbeq(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fbne(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fblt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Fble(instruction) => instruction.get_jump_address(),
            InstructionEnum::End(instruction) => instruction.get_jump_address(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            InstructionEnum::Lw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Addi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slli(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slti(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Xori(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Srli(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Srai(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Ori(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Andi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Add(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sub(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sll(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slt(instruction) => instruction.get_instruction_count(),
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
            InstructionEnum::Jalr(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jal(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Flw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fadd(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsub(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fmul(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fdiv(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsqrt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsgnj(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsgnjn(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsgnjx(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Feq(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Flt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fle(instruction) => instruction.get_instruction_count(),
            InstructionEnum::FcvtWS(instruction) => instruction.get_instruction_count(),
            InstructionEnum::FcvtSW(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fsw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::In(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fin(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Outchar(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Outint(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fbeq(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fbne(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fblt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Fble(instruction) => instruction.get_instruction_count(),
            InstructionEnum::End(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            InstructionEnum::Lw(instruction) => instruction.get_name(),
            InstructionEnum::Addi(instruction) => instruction.get_name(),
            InstructionEnum::Slli(instruction) => instruction.get_name(),
            InstructionEnum::Slti(instruction) => instruction.get_name(),
            InstructionEnum::Xori(instruction) => instruction.get_name(),
            InstructionEnum::Srli(instruction) => instruction.get_name(),
            InstructionEnum::Srai(instruction) => instruction.get_name(),
            InstructionEnum::Ori(instruction) => instruction.get_name(),
            InstructionEnum::Andi(instruction) => instruction.get_name(),
            InstructionEnum::Sw(instruction) => instruction.get_name(),
            InstructionEnum::Add(instruction) => instruction.get_name(),
            InstructionEnum::Sub(instruction) => instruction.get_name(),
            InstructionEnum::Sll(instruction) => instruction.get_name(),
            InstructionEnum::Slt(instruction) => instruction.get_name(),
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
            InstructionEnum::Jalr(instruction) => instruction.get_name(),
            InstructionEnum::Jal(instruction) => instruction.get_name(),
            InstructionEnum::Flw(instruction) => instruction.get_name(),
            InstructionEnum::Fadd(instruction) => instruction.get_name(),
            InstructionEnum::Fsub(instruction) => instruction.get_name(),
            InstructionEnum::Fmul(instruction) => instruction.get_name(),
            InstructionEnum::Fdiv(instruction) => instruction.get_name(),
            InstructionEnum::Fsqrt(instruction) => instruction.get_name(),
            InstructionEnum::Fsgnj(instruction) => instruction.get_name(),
            InstructionEnum::Fsgnjn(instruction) => instruction.get_name(),
            InstructionEnum::Fsgnjx(instruction) => instruction.get_name(),
            InstructionEnum::Feq(instruction) => instruction.get_name(),
            InstructionEnum::Flt(instruction) => instruction.get_name(),
            InstructionEnum::Fle(instruction) => instruction.get_name(),
            InstructionEnum::FcvtWS(instruction) => instruction.get_name(),
            InstructionEnum::FcvtSW(instruction) => instruction.get_name(),
            InstructionEnum::Fsw(instruction) => instruction.get_name(),
            InstructionEnum::In(instruction) => instruction.get_name(),
            InstructionEnum::Fin(instruction) => instruction.get_name(),
            InstructionEnum::Outchar(instruction) => instruction.get_name(),
            InstructionEnum::Outint(instruction) => instruction.get_name(),
            InstructionEnum::Fbeq(instruction) => instruction.get_name(),
            InstructionEnum::Fbne(instruction) => instruction.get_name(),
            InstructionEnum::Fblt(instruction) => instruction.get_name(),
            InstructionEnum::Fble(instruction) => instruction.get_name(),
            InstructionEnum::End(instruction) => instruction.get_name(),
        }
    }
}

fn create_i_instruction_struct(
    imm: Imm13,
    rs1: Rs1,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        0 => match funct3 {
            0b010 => InstructionEnum::Lw(Lw::new(imm, rs1, rd)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        1 => match funct3 {
            0b000 => InstructionEnum::Addi(Addi::new(imm, rs1, rd)),
            0b001 => InstructionEnum::Slli(Slli::new(imm, rs1, rd)),
            0b010 => InstructionEnum::Slti(Slti::new(imm, rs1, rd)),
            0b100 => InstructionEnum::Xori(Xori::new(imm, rs1, rd)),
            0b101 => {
                let funct7 = (imm >> 6) & 0b1111111;
                match funct7 {
                    0b0000000 => InstructionEnum::Srli(Srli::new(imm, rs1, rd)),
                    0b0100000 => InstructionEnum::Srai(Srai::new(imm, rs1, rd)),
                    _ => {
                        panic!("unexpected funct7: {}", funct7);
                    }
                }
            }
            0b110 => InstructionEnum::Ori(Ori::new(imm, rs1, rd)),
            0b111 => InstructionEnum::Andi(Andi::new(imm, rs1, rd)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        6 => match funct3 {
            0b000 => InstructionEnum::Jalr(Jalr::new(imm, rs1, rd)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        8 => match funct3 {
            0b010 => InstructionEnum::Flw(Flw::new(imm, rs1, rd)),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        14 => match funct3 {
            0b000 => InstructionEnum::End(End::new()),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        11 => match funct3 {
            0b000 => InstructionEnum::In(In::new(rd)),
            0b001 => InstructionEnum::Fin(Fin::new(rd)),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
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
        3 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => InstructionEnum::Add(Add::new(rs2, rs1, rd)),
                0b0100000 => InstructionEnum::Sub(Sub::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b001 => match funct7 {
                0b0000000 => InstructionEnum::Sll(Sll::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b010 => match funct7 {
                0b0000000 => InstructionEnum::Slt(Slt::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b100 => match funct7 {
                0b0000000 => InstructionEnum::Xor(Xor::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b101 => match funct7 {
                0b0000000 => InstructionEnum::Srl(Srl::new(rs2, rs1, rd)),
                0b0100000 => InstructionEnum::Sra(Sra::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b110 => match funct7 {
                0b0000000 => InstructionEnum::Or(Or::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b111 => match funct7 {
                0b0000000 => InstructionEnum::And(And::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        9 => match funct7 {
            0b0000000 => InstructionEnum::Fadd(Fadd::new(rs2, rs1, rd)),
            0b0000100 => InstructionEnum::Fsub(Fsub::new(rs2, rs1, rd)),
            0b0001000 => InstructionEnum::Fmul(Fmul::new(rs2, rs1, rd)),
            0b0001100 => InstructionEnum::Fdiv(Fdiv::new(rs2, rs1, rd)),
            0b0101100 => InstructionEnum::Fsqrt(Fsqrt::new(rs2, rs1, rd)),
            0b0010000 => match funct3 {
                0b000 => InstructionEnum::Fsgnj(Fsgnj::new(rs2, rs1, rd)),
                0b001 => InstructionEnum::Fsgnjn(Fsgnjn::new(rs2, rs1, rd)),
                0b010 => InstructionEnum::Fsgnjx(Fsgnjx::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct3: {}", funct3)
                }
            },
            0b1010000 => match funct3 {
                0b010 => InstructionEnum::Feq(Feq::new(rs2, rs1, rd)),
                0b001 => InstructionEnum::Flt(Flt::new(rs2, rs1, rd)),
                0b000 => InstructionEnum::Fle(Fle::new(rs2, rs1, rd)),
                _ => {
                    panic!("unexpected funct3: {}", funct3)
                }
            },
            0b1100000 => InstructionEnum::FcvtWS(FcvtWS::new(rs2, rs1, rd)),
            0b1101000 => InstructionEnum::FcvtSW(FcvtSW::new(rs2, rs1, rd)),
            _ => {
                panic!("unexpected funct7: {}", funct7)
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_s_instruction_struct(
    imm: Imm13,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        2 => match funct3 {
            0b010 => InstructionEnum::Sw(Sw::new(imm, rs2, rs1)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        10 => match funct3 {
            0b010 => InstructionEnum::Fsw(Fsw::new(imm, rs2, rs1)),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        12 => match funct3 {
            0b000 => InstructionEnum::Outchar(Outchar::new(rs2)),
            0b001 => InstructionEnum::Outint(Outint::new(rs2)),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_b_instruction_struct(
    imm: Imm13,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        5 => match funct3 {
            0b000 => InstructionEnum::Beq(Beq::new(imm, rs2, rs1)),
            0b001 => InstructionEnum::Bne(Bne::new(imm, rs2, rs1)),
            0b100 => InstructionEnum::Blt(Blt::new(imm, rs2, rs1)),
            0b101 => InstructionEnum::Bge(Bge::new(imm, rs2, rs1)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        13 => match funct3 {
            0b000 => InstructionEnum::Fbeq(Fbeq::new(imm, rs2, rs1)),
            0b001 => InstructionEnum::Fbne(Fbne::new(imm, rs2, rs1)),
            0b100 => InstructionEnum::Fblt(Fblt::new(imm, rs2, rs1)),
            0b101 => InstructionEnum::Fble(Fble::new(imm, rs2, rs1)),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_j_instruction_struct(imm: Imm19, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        7 => InstructionEnum::Jal(Jal::new(imm, rd)),
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_u_instruction_struct(imm: Imm19, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        4 => InstructionEnum::Lui(Lui::new(imm, rd)),
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

pub fn create_instruction_struct(inst: Instruction) -> InstructionEnum {
    match inst {
        Instruction::I(imm, rs1, funct3, rd, op) => {
            create_i_instruction_struct(imm, rs1, funct3, rd, op)
        }
        Instruction::R(funct7, rs2, rs1, funct3, rd, op) => {
            create_r_instruction_struct(funct7, rs2, rs1, funct3, rd, op)
        }
        Instruction::S(imm, rs2, rs1, funct3, op) => {
            create_s_instruction_struct(imm, rs2, rs1, funct3, op)
        }
        Instruction::B(imm, rs2, rs1, funct3, op) => {
            create_b_instruction_struct(imm, rs2, rs1, funct3, op)
        }
        Instruction::J(imm, rd, op) => create_j_instruction_struct(imm, rd, op),
        Instruction::U(imm, rd, op) => create_u_instruction_struct(imm, rd, op),
        _ => {
            panic!("unexpected instruction: {:?}", inst);
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

pub fn get_source_registers(inst: &InstructionEnum) -> Vec<RegisterId> {
    inst.get_source_registers()
}

pub fn get_destination_register(inst: &InstructionEnum) -> Option<RegisterId> {
    inst.get_destination_register()
}

pub fn is_load_instruction(inst: &InstructionEnum) -> bool {
    inst.is_load_instruction()
}

pub fn get_jump_address(inst: &InstructionEnum) -> Option<Address> {
    inst.get_jump_address()
}

pub fn get_instruction_count(inst: &InstructionEnum) -> Option<InstructionCount> {
    inst.get_instruction_count()
}

pub fn get_name(inst: &InstructionEnum) -> String {
    inst.get_name()
}
