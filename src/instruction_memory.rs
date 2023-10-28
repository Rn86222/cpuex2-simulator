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

    // pub fn get_cache_line(&self, addr: Address) -> [InstructionValue; VALUE_IN_LINE_NUM] {
    //     let mut line = [0; VALUE_IN_LINE_NUM];
    //     for i in 0..VALUE_IN_LINE_NUM {
    //         line[i] = self.load(addr + (4 * i) as Address);
    //     }
    //     line
    // }

    // pub fn set_cache_line(&mut self, line: [(Address, InstructionValue); VALUE_IN_LINE_NUM]) {
    //     for i in 0..VALUE_IN_LINE_NUM {
    //         self.store(line[i].0, line[i].1);
    //     }
    // }
}
