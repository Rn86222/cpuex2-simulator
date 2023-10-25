use crate::types::*;
const INSTRUCTION_MEMORY_SIZE: usize = 4 * 1024 * 1024;

pub struct InstructionMemory {
    values: [InstructionValue; INSTRUCTION_MEMORY_SIZE],
}

impl InstructionMemory {
    pub fn new() -> Self {
        let init_val = 0;
        let values = [init_val; INSTRUCTION_MEMORY_SIZE];
        let memory = InstructionMemory { values };
        memory
    }

    pub fn load(&self, addr: Address) -> InstructionValue {
        self.values[(addr >> 2) as usize]
    }

    pub fn store(&mut self, addr: Address, value: InstructionValue) {
        self.values[(addr >> 2) as usize] = value;
    }

    // pub fn get_cache_line(&self, addr: Address) -> [MemoryValue; LINE_SIZE] {
    //     let mut line = [0; LINE_SIZE];
    //     for i in 0..LINE_SIZE {
    //         line[i] = self.load_ubyte(addr + i as Address);
    //     }
    //     line
    // }

    // pub fn set_cache_line(&mut self, line: [(Address, MemoryValue); LINE_SIZE]) {
    //     for i in 0..LINE_SIZE {
    //         self.store_ubyte(line[i].0, line[i].1);
    //     }
    // }
}
