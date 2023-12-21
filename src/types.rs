use crate::fpu_emulator::FloatingPoint;

pub type Byte = i8;
pub type UByte = u8;
pub type Half = i16;
pub type UHalf = u16;
pub type Word = i32;
// pub type Double = f64;
pub type Int = i32;
pub type Float = f32;
pub type MemoryValue = u8;
pub type Address = u32;
pub type InstructionValue = u32;
// pub type InstructionCacheIndex = usize;
pub type Tag = u32;
pub type CacheIndex = usize;
pub type Imm12 = i16;
pub type Imm20 = i32;
pub enum RegisterType {
    Int,
    Float,
}
pub type RegisterId = (RegisterType, Rs);
pub enum RegisterValue {
    Int(Int),
    Float(FloatingPoint),
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
pub type Funct2 = u8;
pub type Fd = u8;
pub type InstructionCount = u128;
