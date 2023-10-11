use std::time::Duration;
use std::time::Instant;

use crate::instruction::exec_instruction;
use crate::memory::*;
use crate::register::*;
use crate::types::*;
use crate::utils::*;

const INT_REGISTER_SIZE: usize = 32;
const FLOAT_REGISTER_SIZE: usize = 16;

pub struct Core {
    memory: Memory,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        Core {
            memory,
            int_registers,
            float_registers,
            pc: 0,
        }
    }

    pub fn get_pc(&self) -> Address {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc += 4;
    }

    pub fn set_pc(&mut self, new_pc: Address) {
        self.pc = new_pc;
    }

    pub fn get_int_register(&self, index: usize) -> Int {
        self.int_registers[index].get()
    }

    pub fn set_int_register(&mut self, index: usize, value: Int) {
        if index == 0 {
            return; // zero register
        }
        self.int_registers[index].set(value);
    }

    pub fn get_float_register(&self, index: usize) -> Float {
        self.float_registers[index].get()
    }

    pub fn set_float_register(&mut self, index: usize, value: Float) {
        self.float_registers[index].set(value);
    }

    pub fn load_byte(&self, addr: Address) -> Byte {
        self.memory.load_byte(addr)
    }

    pub fn load_ubyte(&self, addr: Address) -> UByte {
        self.memory.load_ubyte(addr)
    }

    pub fn store_byte(&mut self, addr: Address, value: Byte) {
        self.memory.store_byte(addr, value);
    }

    pub fn load_half(&self, addr: Address) -> Half {
        self.memory.load_half(addr)
    }

    pub fn load_uhalf(&self, addr: Address) -> UHalf {
        self.memory.load_uhalf(addr)
    }

    pub fn load_word(&self, addr: Address) -> Word {
        self.memory.load_word(addr)
    }

    pub fn store_half(&mut self, addr: Address, value: Half) {
        self.memory.store_half(addr, value);
    }

    pub fn store_word(&mut self, addr: Address, value: Word) {
        self.memory.store_word(addr, value);
    }

    pub fn get_memory_byte(&self, addr: Address) -> String {
        self.memory.get_byte(addr)
    }

    pub fn show_registers(&self) {
        for i in 0..INT_REGISTER_SIZE {
            print!("x{: <2} 0x{:>04x} ", i, self.get_int_register(i));
            if i % 8 == 7 {
                println!("");
            }
        }
        // for i in 0..FLOAT_REGISTER_SIZE {
        //     print!("f{: <2} {} ", i, self.get_float_register(i));
        //     if i % 8 == 7 {
        //         println!("");
        //     }
        // }
    }

    pub fn show_memory(&self) {
        println!("memory");
        self.memory.show();
    }

    pub fn run(&mut self, verbose: bool, interval: u64) {
        let start_time = Instant::now();
        let mut inst_count = 0;
        loop {
            if verbose {
                println!("pc: {}", self.get_pc());
            }
            if interval != 0 {
                let interval_start_time = Instant::now();
                while interval_start_time.elapsed() < Duration::from_millis(interval) {}
            }
            let current_pc = self.get_pc();
            let mut inst: [MemoryValue; 4] = [0; 4];
            for i in 0..4 {
                inst[i] = self.load_ubyte(current_pc + i);
            }
            exec_instruction(self, inst, verbose);
            inst_count += 1;
            if verbose {
                self.show_registers();
            }
            if start_time.elapsed() > Duration::from_millis(1000) {
                println!("{}", inst_count);
                return;
            }
        }
    }
}
