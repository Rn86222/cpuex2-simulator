use crate::types::*;
use crate::utils::*;
const MEMORY_SIZE: usize = 128000000;
const CACHE_SIZE: usize = 4860 * 1024 / 8;

pub struct Memory {
    values: [MemoryValue; MEMORY_SIZE],
    cache: [MemoryValue; CACHE_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let init_val = 0;
        let values = [init_val; MEMORY_SIZE];
        let cache = [init_val; CACHE_SIZE];
        let memory = Memory { values, cache };
        memory
    }

    pub fn load_byte(&self, addr: Address) -> Byte {
        self.values[addr] as Byte
    }

    pub fn load_ubyte(&self, addr: Address) -> UByte {
        self.values[addr] as UByte
    }

    pub fn store_byte(&mut self, addr: Address, value: Byte) {
        self.values[addr] = i8_to_u8(value);
    }

    pub fn get_byte(&self, addr: Address) -> String {
        self.values[addr].to_string()
    }

    pub fn load_word(&self, addr: Address) -> Word {
        let mut load_value: u32 = 0;

        for i in 0..4 {
            load_value += (self.load_ubyte(addr + i) as u32) << (8 * i);
        }
        u32_to_i32(load_value) as Word
    }

    pub fn store_word(&mut self, addr: Address, value: Word) {
        for i in 0..4 {
            self.store_byte(addr + i, ((value >> (i * 8)) & 0xff) as Byte);
        }
    }

    pub fn show(&self) {
        for i in 0..MEMORY_SIZE {
            print!("{} {}\n", i, self.values[i]);
        }
        println!("");
    }

    pub fn show_word(&self, addr: Address) {
        for i in 0..4 {
            print!("{}", self.get_byte(addr + 3 - i));
        }
        println!("");
    }
}
