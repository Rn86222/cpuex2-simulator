use crate::pseudo_lru_cache::LINE_SIZE;
use crate::types::*;
use crate::utils::*;
pub const MEMORY_SIZE: usize = 128 * 1024 * 1024;
pub const WORD_SIZE: usize = 4;

pub struct Memory {
    values: [MemoryValue; MEMORY_SIZE / WORD_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let init_val = 0;
        let values = [init_val; MEMORY_SIZE / WORD_SIZE];
        Memory { values }
    }

    pub fn load_word(&self, addr: Address) -> Word {
        u32_to_i32(self.values[addr as usize >> 2])
    }

    pub fn store_word(&mut self, addr: Address, value: Word) {
        self.values[addr as usize >> 2] = i32_to_u32(value);
    }

    pub fn get_cache_line(&self, addr: Address) -> [MemoryValue; LINE_SIZE / WORD_SIZE] {
        let mut line = [0; LINE_SIZE / WORD_SIZE];
        for (i, value) in line.iter_mut().enumerate() {
            *value = i32_to_u32(self.load_word(addr + i as Address * 4));
        }
        line
    }

    pub fn set_cache_line(&mut self, line: [(Address, MemoryValue); LINE_SIZE / WORD_SIZE]) {
        for (addr, value) in line.iter() {
            self.store_word(*addr, u32_to_i32(*value));
        }
    }
}
