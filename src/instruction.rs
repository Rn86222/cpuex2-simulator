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

fn println_inst(text: &str) {
    println!("{}", text);
    // colorized_println(text, RED);
}

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
        // println!(
        //     "{:x} {}  load word: {} from {:x}",
        //     self.data.rs1_value.unwrap(),
        //     self.data.extended_imm.unwrap(),
        //     value,
        //     addr
        // );
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
        // println!("store word: {} to {:x}", self.data.rs2_value.unwrap(), addr);
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        vec![self.data.rs1, self.data.rs2]
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        self.data.inst_count
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
        write!(
            f,
            "beq x{}, x{}, {}",
            self.data.rs1, self.data.rs2, extended_imm
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
        write!(
            f,
            "bne x{}, x{}, {}",
            self.data.rs1, self.data.rs2, extended_imm
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
        write!(
            f,
            "blt x{}, x{}, {}",
            self.data.rs1, self.data.rs2, extended_imm
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
        write!(
            f,
            "bge x{}, x{}, {}",
            self.data.rs1, self.data.rs2, extended_imm
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
}

#[derive(Clone)]
pub enum InstructionEnum {
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Addi(Addi),
    Slti(Slti),
    Ori(Ori),
    Andi(Andi),
    Sw(Sw),
    Add(Add),
    Sub(Sub),
    Slt(Slt),
    Or(Or),
    And(And),
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Bge(Bge),
    Jalr(Jalr),
    Jal(Jal),
}

impl Debug for InstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionEnum::Lb(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lh(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Lw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Addi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slti(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Ori(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Andi(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sw(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Add(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Sub(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Slt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Or(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::And(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Beq(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bne(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Blt(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Bge(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jalr(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jal(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for InstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lh(instruction) => instruction.register_fetch(core),
            InstructionEnum::Lw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Addi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slti(instruction) => instruction.register_fetch(core),
            InstructionEnum::Ori(instruction) => instruction.register_fetch(core),
            InstructionEnum::Andi(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sw(instruction) => instruction.register_fetch(core),
            InstructionEnum::Add(instruction) => instruction.register_fetch(core),
            InstructionEnum::Sub(instruction) => instruction.register_fetch(core),
            InstructionEnum::Slt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Or(instruction) => instruction.register_fetch(core),
            InstructionEnum::And(instruction) => instruction.register_fetch(core),
            InstructionEnum::Beq(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bne(instruction) => instruction.register_fetch(core),
            InstructionEnum::Blt(instruction) => instruction.register_fetch(core),
            InstructionEnum::Bge(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jalr(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jal(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.exec(core),
            InstructionEnum::Lh(instruction) => instruction.exec(core),
            InstructionEnum::Lw(instruction) => instruction.exec(core),
            InstructionEnum::Addi(instruction) => instruction.exec(core),
            InstructionEnum::Slti(instruction) => instruction.exec(core),
            InstructionEnum::Ori(instruction) => instruction.exec(core),
            InstructionEnum::Andi(instruction) => instruction.exec(core),
            InstructionEnum::Sw(instruction) => instruction.exec(core),
            InstructionEnum::Add(instruction) => instruction.exec(core),
            InstructionEnum::Sub(instruction) => instruction.exec(core),
            InstructionEnum::Slt(instruction) => instruction.exec(core),
            InstructionEnum::Or(instruction) => instruction.exec(core),
            InstructionEnum::And(instruction) => instruction.exec(core),
            InstructionEnum::Beq(instruction) => instruction.exec(core),
            InstructionEnum::Bne(instruction) => instruction.exec(core),
            InstructionEnum::Blt(instruction) => instruction.exec(core),
            InstructionEnum::Bge(instruction) => instruction.exec(core),
            InstructionEnum::Jalr(instruction) => instruction.exec(core),
            InstructionEnum::Jal(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.memory(core),
            InstructionEnum::Lh(instruction) => instruction.memory(core),
            InstructionEnum::Lw(instruction) => instruction.memory(core),
            InstructionEnum::Addi(instruction) => instruction.memory(core),
            InstructionEnum::Slti(instruction) => instruction.memory(core),
            InstructionEnum::Ori(instruction) => instruction.memory(core),
            InstructionEnum::Andi(instruction) => instruction.memory(core),
            InstructionEnum::Sw(instruction) => instruction.memory(core),
            InstructionEnum::Add(instruction) => instruction.memory(core),
            InstructionEnum::Sub(instruction) => instruction.memory(core),
            InstructionEnum::Slt(instruction) => instruction.memory(core),
            InstructionEnum::Or(instruction) => instruction.memory(core),
            InstructionEnum::And(instruction) => instruction.memory(core),
            InstructionEnum::Beq(instruction) => instruction.memory(core),
            InstructionEnum::Bne(instruction) => instruction.memory(core),
            InstructionEnum::Blt(instruction) => instruction.memory(core),
            InstructionEnum::Bge(instruction) => instruction.memory(core),
            InstructionEnum::Jalr(instruction) => instruction.memory(core),
            InstructionEnum::Jal(instruction) => instruction.memory(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            InstructionEnum::Lb(instruction) => instruction.write_back(core),
            InstructionEnum::Lh(instruction) => instruction.write_back(core),
            InstructionEnum::Lw(instruction) => instruction.write_back(core),
            InstructionEnum::Addi(instruction) => instruction.write_back(core),
            InstructionEnum::Slti(instruction) => instruction.write_back(core),
            InstructionEnum::Ori(instruction) => instruction.write_back(core),
            InstructionEnum::Andi(instruction) => instruction.write_back(core),
            InstructionEnum::Sw(instruction) => instruction.write_back(core),
            InstructionEnum::Add(instruction) => instruction.write_back(core),
            InstructionEnum::Sub(instruction) => instruction.write_back(core),
            InstructionEnum::Slt(instruction) => instruction.write_back(core),
            InstructionEnum::Or(instruction) => instruction.write_back(core),
            InstructionEnum::And(instruction) => instruction.write_back(core),
            InstructionEnum::Beq(instruction) => instruction.write_back(core),
            InstructionEnum::Bne(instruction) => instruction.write_back(core),
            InstructionEnum::Blt(instruction) => instruction.write_back(core),
            InstructionEnum::Bge(instruction) => instruction.write_back(core),
            InstructionEnum::Jalr(instruction) => instruction.write_back(core),
            InstructionEnum::Jal(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lh(instruction) => instruction.get_source_registers(),
            InstructionEnum::Lw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Addi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slti(instruction) => instruction.get_source_registers(),
            InstructionEnum::Ori(instruction) => instruction.get_source_registers(),
            InstructionEnum::Andi(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sw(instruction) => instruction.get_source_registers(),
            InstructionEnum::Add(instruction) => instruction.get_source_registers(),
            InstructionEnum::Sub(instruction) => instruction.get_source_registers(),
            InstructionEnum::Slt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Or(instruction) => instruction.get_source_registers(),
            InstructionEnum::And(instruction) => instruction.get_source_registers(),
            InstructionEnum::Beq(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bne(instruction) => instruction.get_source_registers(),
            InstructionEnum::Blt(instruction) => instruction.get_source_registers(),
            InstructionEnum::Bge(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jalr(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jal(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lh(instruction) => instruction.get_destination_register(),
            InstructionEnum::Lw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Addi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slti(instruction) => instruction.get_destination_register(),
            InstructionEnum::Ori(instruction) => instruction.get_destination_register(),
            InstructionEnum::Andi(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sw(instruction) => instruction.get_destination_register(),
            InstructionEnum::Add(instruction) => instruction.get_destination_register(),
            InstructionEnum::Sub(instruction) => instruction.get_destination_register(),
            InstructionEnum::Slt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Or(instruction) => instruction.get_destination_register(),
            InstructionEnum::And(instruction) => instruction.get_destination_register(),
            InstructionEnum::Beq(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bne(instruction) => instruction.get_destination_register(),
            InstructionEnum::Blt(instruction) => instruction.get_destination_register(),
            InstructionEnum::Bge(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jalr(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jal(instruction) => instruction.get_destination_register(),
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
            InstructionEnum::Addi(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Slti(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Ori(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Andi(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sw(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Add(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Sub(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Slt(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Or(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::And(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Beq(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bne(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Blt(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Bge(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Jalr(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Jal(instruction) => instruction.is_branch_instruction(),
        }
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lh(instruction) => instruction.get_jump_address(),
            InstructionEnum::Lw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Addi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slti(instruction) => instruction.get_jump_address(),
            InstructionEnum::Ori(instruction) => instruction.get_jump_address(),
            InstructionEnum::Andi(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sw(instruction) => instruction.get_jump_address(),
            InstructionEnum::Add(instruction) => instruction.get_jump_address(),
            InstructionEnum::Sub(instruction) => instruction.get_jump_address(),
            InstructionEnum::Slt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Or(instruction) => instruction.get_jump_address(),
            InstructionEnum::And(instruction) => instruction.get_jump_address(),
            InstructionEnum::Beq(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bne(instruction) => instruction.get_jump_address(),
            InstructionEnum::Blt(instruction) => instruction.get_jump_address(),
            InstructionEnum::Bge(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jalr(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jal(instruction) => instruction.get_jump_address(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            InstructionEnum::Lb(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lh(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Lw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Addi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slti(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Ori(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Andi(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sw(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Add(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Sub(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Slt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Or(instruction) => instruction.get_instruction_count(),
            InstructionEnum::And(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Beq(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bne(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Blt(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Bge(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jalr(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jal(instruction) => instruction.get_instruction_count(),
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
            // 0b100 => {
            //     // lbu
            // }
            // 0b101 => {
            //     // lhu
            // }
            _ => {
                println!("unexpected funct3: {}", funct3);
                panic!();
            }
        },
        19 => match funct3 {
            0b000 => InstructionEnum::Addi(Addi::new(imm, rs1, rd)),
            // 0b001 => {
            //     // slli;
            // }
            0b010 => InstructionEnum::Slti(Slti::new(imm, rs1, rd)),
            // 0b011 => {
            //     // sltiu
            // }
            // 0b100 => {
            //     // xori
            // }
            // 0b101 => {
            //     let funct7 = (imm >> 5) & 0b1111111;
            //     match funct7 {
            //         0b0000000 => {
            //             // srli
            //         }
            //         0b0100000 => {
            //             // srai
            //         }
            //         _ => {
            //             println!("unexpected funct7: {}", funct7)
            //         }
            //     }
            // }
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
                // 0b0000001 => {
                //     // mul
                // }
                // 0b0110000 => {
                //     // absdiff
                // }
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            // 0b001 => match funct7 {
            //     0b0000000 => {
            //         // sll
            //     }
            //     0b0000001 => {
            //         // mulh
            //     }
            //     _ => {
            //         println!("unexpected funct7: {}", funct7)
            //     }
            // },
            0b010 => match funct7 {
                0b0000000 => InstructionEnum::Slt(Slt::new(rs2, rs1, rd)),
                // 0b0000001 => {
                //     // mulhsu
                // }
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            // 0b011 => match funct7 {
            //     0b0000000 => {
            //         // sltu
            //     }
            //     0b0000001 => {
            //         // mulhu
            //     }
            //     _ => {
            //         println!("unexpected funct7: {}", funct7)
            //     }
            // },
            // 0b100 => match funct7 {
            //     0b0000000 => {
            //         // xor
            //     }
            //     0b0000001 => {
            //         // div
            //     }
            //     _ => {
            //         println!("unexpected funct7: {}", funct7)
            //     }
            // },
            // 0b101 => match funct7 {
            //     0b0000000 => {
            //         // srl
            //     }
            //     0b0100000 => {
            //         // sra
            //     }
            //     0b0000001 => {
            //         // divu
            //     }
            //     _ => {
            //         println!("unexpected funct7: {}", funct7)
            //     }
            // },
            0b110 => match funct7 {
                0b0000000 => InstructionEnum::Or(Or::new(rs2, rs1, rd)),
                // 0b0000001 => {
                //     // rem
                // }
                _ => {
                    println!("unexpected funct7: {}", funct7);
                    panic!();
                }
            },
            0b111 => match funct7 {
                0b0000000 => InstructionEnum::And(And::new(rs2, rs1, rd)),
                // 0b0000001 => {
                //     // remu
                // }
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
            // 0b000 => {
            //     // sb
            // }
            // 0b001 => {
            //     // sh
            // }
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
            // 0b110 => {
            //     // bltu
            // }
            // 0b111 => {
            //     // bgeu
            // }
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
        // 23 => {
        //     // auipc
        // }
        // 55 => {
        //     // lui
        // }
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
//     let lb = IInstructionExecutor {
//         exec: |core: &mut Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b000, _, 3) => {
//                     let extended_imm = core.get_extended_imm();
//                     let rs1_value = core.get_int_register(rs1 as usize);
//                     let addr = rs1_value + extended_imm;
//                     ExecResult::I32(addr)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         memory: |core: &mut Core| {
//             let instruction = core.get_instruction_in_memory_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b000, _, 3) => {
//                     let exec_result = core.get_exec_result().unwrap();
//                     match exec_result {
//                         ExecResult::I32(addr) => {
//                             let value = core.load_byte(addr as Address);
//                             LoadValue::Byte(value)
//                         }
//                         _ => {
//                             println!("unexpected addr: {:?}", exec_result);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         write_back: |core: &mut Core| {
//             let instruction = core.get_instruction_in_write_back_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b000, rd, 3) => {
//                     let load_value = core.get_load_value().unwrap();
//                     match load_value {
//                         LoadValue::Byte(value) => {
//                             core.set_int_register(rd as usize, value as Int);
//                         }
//                         _ => {
//                             println!("unexpected load_value: {:?}", load_value);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                 }
//             }
//         },
//         debug: |core: &Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b000, rd, 3) => {
//                     let extended_imm = sign_extention_i16(imm, 12);
//                     format!("lb x{}, {}(x{})", rd, extended_imm, rs1)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         name: "lb",
//     };
//     map.insert((3, 0b000), lb);
//     // let lh = IInstructionExecutor {
//     //     exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//     //         let imm = sign_extention_i16(imm, 12);
//     //         if verbose {
//     //             println_inst(&format!("lh x{}, {}(x{})", rd, imm, rs1));
//     //         }
//     //         let value = core
//     //             .load_half((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
//     //         core.set_int_register(rd as usize, value as Int);
//     //     },
//     //     name: "lh",
//     // };
//     // map.insert((3, 0b001), lh);
//     let lw = IInstructionExecutor {
//         exec: |core: &mut Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b010, _, 3) => {
//                     let extended_imm = core.get_extended_imm();
//                     let rs1_value = core.get_int_register(rs1 as usize);
//                     let addr = rs1_value + extended_imm;
//                     ExecResult::I32(addr)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         memory: |core: &mut Core| {
//             let instruction = core.get_instruction_in_memory_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b010, _, 3) => {
//                     let exec_result = core.get_exec_result().unwrap();
//                     match exec_result {
//                         ExecResult::I32(addr) => {
//                             let value = core.load_word(addr as Address);
//                             LoadValue::Word(value)
//                         }
//                         _ => {
//                             println!("unexpected addr: {:?}", exec_result);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         write_back: |core: &mut Core| {
//             let instruction = core.get_instruction_in_write_back_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b010, rd, 3) => {
//                     let load_value = core.get_load_value().unwrap();
//                     match load_value {
//                         LoadValue::Byte(value) => {
//                             core.set_int_register(rd as usize, value as Int);
//                         }
//                         _ => {
//                             println!("unexpected load_value: {:?}", load_value);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                 }
//             }
//         },
//         debug: |core: &Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b010, rd, 3) => {
//                     let extended_imm = sign_extention_i16(imm, 12);
//                     format!("lw x{}, {}(x{})", rd, extended_imm, rs1)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         name: "lw",
//     };
//     map.insert((3, 0b010), lw);
//     // let lbu = IInstructionExecutor {
//     //     exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//     //         let imm = sign_extention_i16(imm, 12);
//     //         if verbose {
//     //             println_inst(&format!("lbu x{}, {}(x{})", rd, imm, rs1));
//     //         }
//     //         let value = core
//     //             .load_ubyte((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
//     //         core.set_int_register(rd as usize, value as Int);
//     //     },
//     //     name: "lbu",
//     // };
//     // map.insert((3, 0b100), lbu);
//     // let lhu = IInstructionExecutor {
//     //     exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//     //         let imm = sign_extention_i16(imm, 12);
//     //         if verbose {
//     //             println_inst(&format!("lhu x{}, {}(x{})", rd, imm, rs1));
//     //         }
//     //         let value = core
//     //             .load_uhalf((imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address);
//     //         core.set_int_register(rd as usize, value as Int);
//     //     },
//     //     name: "lhu",
//     // };
//     // map.insert((3, 0b101), lhu);
//     let addi = IInstructionExecutor {
//         exec: |core: &mut Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b000, _, 19) => {
//                     let extended_imm = core.get_extended_imm();
//                     let rs1_value = core.get_int_register(rs1 as usize);
//                     let value = rs1_value + extended_imm;
//                     ExecResult::I32(value)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         memory: |core: &mut Core| {
//             let instruction = core.get_instruction_in_memory_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b000, _, 19) => {
//                     let exec_result = core.get_exec_result().unwrap();
//                     match exec_result {
//                         ExecResult::I32(addr) => LoadValue::None,
//                         _ => {
//                             println!("unexpected addr: {:?}", exec_result);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         write_back: |core: &mut Core| {
//             let instruction = core.get_instruction_in_write_back_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(_, _, 0b000, rd, 19) => {
//                     let before_exec_result = core.get_before_exec_result();
//                     match before_exec_result {
//                         ExecResult::I32(value) => {
//                             core.set_int_register(rd as usize, value as Int);
//                         }
//                         _ => {
//                             println!("unexpected before_exec_value: {:?}", before_exec_result);
//                             panic!();
//                         }
//                     }
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                 }
//             }
//         },
//         debug: |core: &Core| {
//             let instruction = core.get_instruction_in_exec_stage().unwrap();
//             match instruction {
//                 Instruction::IInstruction(imm, rs1, 0b000, rd, 19) => {
//                     let extended_imm = sign_extention_i16(imm, 12);
//                     format!("addi x{}, x{}, {}", rd, rs1, extended_imm)
//                 }
//                 _ => {
//                     println!("unexpected instruction: {:?}", instruction);
//                     panic!();
//                 }
//             }
//         },
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("addi x{}, x{}, {}", rd, rs1, imm));
//             }
//             let value = core.get_int_register(rs1 as usize) + imm as i32;
//             core.set_int_register(rd as usize, value);
//         },
//         name: "addi",
//     };
//     map.insert((19, 0b000), addi);
//     let slli = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = imm & 0b11111;
//             let funct7 = (imm >> 5) & 0b1111111;
//             assert_eq!(funct7, 0);
//             if verbose {
//                 println_inst(&format!("slli x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value << imm);
//         },
//         name: "slli",
//     };
//     map.insert((19, 0b001), slli);
//     let slti = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("slti x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value < imm as i32 {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "slti",
//     };
//     map.insert((19, 0b010), slti);
//     let sltiu = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("sltiu x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
//             if (rs1_value as i64) < (imm as i64) {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "sltiu",
//     };
//     map.insert((19, 0b011), sltiu);
//     let xori = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("xori x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value ^ imm as i32);
//         },
//         name: "xori",
//     };
//     map.insert((19, 0b100), xori);
//     let srli_srai = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let funct7 = (imm >> 5) & 0b1111111;
//             let imm = imm & 0b11111;
//             if funct7 == 0b0000000 {
//                 // srli
//                 if verbose {
//                     println_inst(&format!("srli x{}, x{}, {}", rd, rs1, imm));
//                 }
//                 let rs1_value = core.get_int_register(rs1 as usize);
//                 core.set_int_register(rd as usize, u32_to_i32(i32_to_u32(rs1_value) >> imm));
//             } else if funct7 == 0b0100000 {
//                 // srai
//                 if verbose {
//                     println_inst(&format!("srai x{}, x{}, {}", rd, rs1, imm));
//                 }
//                 let rs1_value = core.get_int_register(rs1 as usize);
//                 core.set_int_register(rd as usize, rs1_value >> imm);
//             } else {
//                 println!("unexpected funct7: {}", funct7);
//             }
//         },
//         name: "srli_srai",
//     };
//     map.insert((19, 0b101), srli_srai);
//     let ori = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("ori x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value | imm as i32);
//         },
//         name: "ori",
//     };
//     map.insert((19, 0b110), ori);
//     let andi = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("andi x{}, x{}, {}", rd, rs1, imm));
//             }
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value & imm as i32);
//         },
//         name: "andi",
//     };
//     map.insert((19, 0b111), andi);
//     let jalr = IInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs1: u8, rd: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("jalr x{}, x{}, {}", rd, rs1, imm));
//             }
//             let new_pc = core.get_int_register(rs1 as usize) + imm as i32;
//             core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
//             core.set_pc(new_pc as Address);
//         },
//         name: "jalr",
//     };
//     map.insert((103, 0b000), jalr);
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
//     let add = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("add x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value + rs2_value);
//         },
//         name: "add",
//     };
//     map.insert((51, 0b000, 0b0000000), add);
//     let sub = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("sub x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value - rs2_value);
//         },
//         name: "sub",
//     };
//     map.insert((51, 0b000, 0b0100000), sub);
//     let mul = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("mul x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) as i64;
//             let rs1_value = core.get_int_register(rs1 as usize) as i64;
//             core.set_int_register(rd as usize, ((rs1_value * rs2_value) & 0xffffffff) as i32);
//         },
//         name: "mul",
//     };
//     map.insert((51, 0b000, 0b0000001), mul);
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
//     let sll = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("sll x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(
//                 rd as usize,
//                 u32_to_i32(i32_to_u32(rs1_value) << (rs2_value & 0b11111)) as Int,
//             );
//         },
//         name: "sll",
//     };
//     map.insert((51, 0b001, 0b0000000), sll);
//     let mulh = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("mulh x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) as i64;
//             let rs1_value = core.get_int_register(rs1 as usize) as i64;
//             core.set_int_register(
//                 rd as usize,
//                 (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
//             );
//         },
//         name: "mulh",
//     };
//     map.insert((51, 0b001, 0b0000001), mulh);
//     let slt = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("slt x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value < rs2_value {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "slt",
//     };
//     map.insert((51, 0b010, 0b0000000), slt);
//     let mulhsu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("mulhsu x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as i64;
//             let rs1_value = core.get_int_register(rs1 as usize) as i64;
//             core.set_int_register(
//                 rd as usize,
//                 (((rs1_value * rs2_value) >> 32) & 0xffffffff) as i32,
//             );
//         },
//         name: "mulhsu",
//     };
//     map.insert((51, 0b010, 0b0000001), mulhsu);
//     let sltu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("sltu x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
//             if rs1_value < rs2_value {
//                 core.set_int_register(rd as usize, 1);
//             } else {
//                 core.set_int_register(rd as usize, 0);
//             }
//         },
//         name: "slty",
//     };
//     map.insert((51, 0b011, 0b0000000), sltu);
//     let mulhu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("mulhu x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize)) as u64;
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize)) as u64;
//             core.set_int_register(
//                 rd as usize,
//                 u32_to_i32((((rs1_value * rs2_value) >> 32) & 0xffffffff) as u32),
//             );
//         },
//         name: "mulhu",
//     };
//     map.insert((51, 0b011, 0b0000001), mulhu);
//     let xor = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("xor x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value ^ rs2_value);
//         },
//         name: "xor",
//     };
//     map.insert((51, 0b100, 0b0000000), xor);
//     let div = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("div x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) as i64;
//             let rs1_value = core.get_int_register(rs1 as usize) as i64;
//             if rs2_value == 0 {
//                 core.set_int_register(rd as usize, -1);
//             } else {
//                 core.set_int_register(rd as usize, ((rs1_value / rs2_value) & 0xffffffff) as i32);
//             }
//         },
//         name: "div",
//     };
//     map.insert((51, 0b100, 0b0000001), div);
//     let srl = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("srl x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(
//                 rd as usize,
//                 u32_to_i32(i32_to_u32(rs1_value) >> (rs2_value & 0b11111)),
//             );
//         },
//         name: "srl",
//     };
//     map.insert((51, 0b101, 0b0000000), srl);
//     let sra = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("sra x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value >> (rs2_value & 0b11111));
//         },
//         name: "sra",
//     };
//     map.insert((51, 0b101, 0b0100000), sra);
//     let divu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("divu x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
//             if rs2_value == 0 {
//                 core.set_int_register(rd as usize, -1);
//             } else {
//                 core.set_int_register(
//                     rd as usize,
//                     u32_to_i32((rs1_value / rs2_value) & 0xffffffff),
//                 );
//             }
//         },
//         name: "divu",
//     };
//     map.insert((51, 0b101, 0b0000001), divu);
//     let or = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("or x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value | rs2_value);
//         },
//         name: "or",
//     };
//     map.insert((51, 0b110, 0b0000000), or);
//     let rem = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("rem x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize) as i64;
//             let rs1_value = core.get_int_register(rs1 as usize) as i64;
//             if rs2_value == 0 {
//                 core.set_int_register(rd as usize, rs1_value as i32);
//             } else {
//                 core.set_int_register(rd as usize, ((rs1_value % rs2_value) & 0xffffffff) as i32);
//             }
//         },
//         name: "rem",
//     };
//     map.insert((51, 0b110, 0b0000001), rem);
//     let and = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("and x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             core.set_int_register(rd as usize, rs1_value & rs2_value);
//         },
//         name: "and",
//     };
//     map.insert((51, 0b111, 0b0000000), and);
//     let remu = RInstructionExecutor {
//         exec: |core: &mut Core, rs2: u8, rs1: u8, rd: u8, verbose: bool| {
//             if verbose {
//                 println_inst(&format!("remu x{}, x{}, x{}", rd, rs1, rs2));
//             }
//             let rs2_value = i32_to_u32(core.get_int_register(rs2 as usize));
//             let rs1_value = i32_to_u32(core.get_int_register(rs1 as usize));
//             if rs2_value == 0 {
//                 core.set_int_register(rd as usize, u32_to_i32(rs1_value));
//             } else {
//                 core.set_int_register(
//                     rd as usize,
//                     u32_to_i32((rs1_value % rs2_value) & 0xffffffff),
//                 );
//             }
//         },
//         name: "remu",
//     };
//     map.insert((51, 0b111, 0b0000001), remu);
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
//     let sb = SInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("sb x{}, {}(x{})", rs2, imm, rs1));
//             }
//             let value = core.get_int_register(rs2 as usize);
//             core.store_byte(
//                 (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
//                 (value & 255) as Byte,
//             )
//         },
//         name: "sb",
//     };
//     map.insert((35, 0b000), sb);
//     let sh = SInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("sh x{}, {}(x{})", rs2, imm, rs1));
//             }
//             let value = core.get_int_register(rs2 as usize);
//             core.store_half(
//                 (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
//                 (value & 65535) as Half,
//             )
//         },
//         name: "sh",
//     };
//     map.insert((35, 0b001), sh);
//     let sw = SInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!("sw x{}, {}(x{})", rs2, imm, rs1));
//             }
//             let value = core.get_int_register(rs2 as usize);
//             core.store_word(
//                 (imm as i64 + core.get_int_register(rs1 as usize) as i64) as Address,
//                 value as Word,
//             )
//         },
//         name: "sw",
//     };
//     map.insert((35, 0b010), sw);
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

// fn create_b_instruction_map() -> BInstructionMap {
//     let mut map = BInstructionMap::new();
//     let beq = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "beq x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value == rs2_value {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "beq",
//     };
//     map.insert((99, 0b000), beq);
//     let bne = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "bne x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value != rs2_value {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "bne",
//     };
//     map.insert((99, 0b001), bne);
//     let blt = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "blt x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value < rs2_value {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "blt",
//     };
//     map.insert((99, 0b100), blt);
//     let bge = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "bge x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if rs1_value >= rs2_value {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "bge",
//     };
//     map.insert((99, 0b101), bge);
//     let bltu = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "bltu x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if i32_to_u32(rs1_value) < i32_to_u32(rs2_value) {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "bltu",
//     };
//     map.insert((99, 0b110), bltu);
//     let bgeu = BInstructionExecutor {
//         exec: |core: &mut Core, imm: i16, rs2: u8, rs1: u8, verbose: bool| {
//             let imm = sign_extention_i16(imm, 12);
//             if verbose {
//                 println_inst(&format!(
//                     "bgeu x{}, x{}, {} + {}",
//                     rs2,
//                     rs1,
//                     core.get_pc(),
//                     imm << 1,
//                 ));
//             }
//             let rs2_value = core.get_int_register(rs2 as usize);
//             let rs1_value = core.get_int_register(rs1 as usize);
//             if i32_to_u32(rs1_value) >= i32_to_u32(rs2_value) {
//                 let new_pc = core.get_pc() as i64 + (imm << 1) as i64;
//                 core.set_pc(new_pc as Address);
//             } else {
//                 core.increment_pc();
//             }
//         },
//         name: "bgeu",
//     };
//     map.insert((99, 0b111), bgeu);
//     map
// }

// fn create_j_instruction_map() -> JInstructionMap {
//     let mut map = JInstructionMap::new();
//     let jal = JInstructionExecutor {
//         exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
//             let imm = sign_extention_i32(imm, 20);
//             if verbose {
//                 println_inst(&format!("jal x{}, {} + {}", rd, core.get_pc(), imm << 1));
//             }
//             let new_pc = core.get_pc() as i32 + (imm << 1);
//             core.set_int_register(rd as usize, u32_to_i32(core.get_pc() as u32 + 4));
//             core.set_pc(new_pc as Address);
//         },
//         name: "jal",
//     };
//     map.insert(111, jal);
//     map
// }

// fn create_u_instruction_map() -> UInstructionMap {
//     let mut map = UInstructionMap::new();
//     let auipc = UInstructionExecutor {
//         exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
//             let imm = sign_extention_i32(imm, 20);
//             if verbose {
//                 println_inst(&format!("auipc x{}, {}", rd, imm << 12));
//             }
//             core.set_int_register(
//                 rd as usize,
//                 (core.get_pc() as i64 + (imm << 12) as i64) as Int,
//             );
//         },
//         name: "auipc",
//     };
//     map.insert(23, auipc);
//     let lui = UInstructionExecutor {
//         exec: |core: &mut Core, imm: i32, rd: u8, verbose: bool| {
//             let imm = sign_extention_i32(imm, 20);
//             if verbose {
//                 println_inst(&format!("lui x{}, {}", rd, imm));
//             }
//             core.set_int_register(rd as usize, (imm as Int) << 12);
//         },
//         name: "lui",
//     };
//     map.insert(55, lui);
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
