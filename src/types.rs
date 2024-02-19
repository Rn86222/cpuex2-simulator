pub type Word = i32;
pub type Int = i32;
pub type Float = f32;
pub type MemoryValue = u32;
pub type Address = u32;
pub type InstructionValue = u32;
pub type Tag = u32;
pub type CacheIndex = usize;
pub type Imm13 = i16;
pub type Imm19 = i32;
#[derive(PartialEq, Eq)]
pub enum RegisterId {
    Int(u8),
    Float(u8),
}
pub type Rs1 = u8;
pub type Rs2 = u8;
pub type Rs = u8;
pub type Rd = u8;
pub type Funct3 = u8;
pub type Funct7 = u8;
pub type Op = u8;
pub type Fs3 = u8;
pub type Fs2 = u8;
pub type Fs1 = u8;
pub type Fd = u8;
pub type InstructionCount = u128;
