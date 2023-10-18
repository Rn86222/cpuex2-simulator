use std::time::Duration;
use std::time::Instant;

use crate::instruction::exec_instruction;
use crate::memory::*;
use crate::register::*;
use crate::types::*;
use crate::utils::*;

const INT_REGISTER_SIZE: usize = 32;
const FLOAT_REGISTER_SIZE: usize = 32;

pub struct Core {
    memory: Memory,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
    int_registers_history: Vec<[IntRegister; INT_REGISTER_SIZE]>,
    pc_history: Vec<Address>,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        let pc = 0;
        let int_registers_history = Vec::new();
        let pc_history = Vec::new();
        Core {
            memory,
            int_registers,
            float_registers,
            pc,
            int_registers_history,
            pc_history,
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

    // pub fn get_memory_byte(&self, addr: Address) -> String {
    //     self.memory.get_byte(addr)
    // }

    pub fn show_registers(&self) {
        for i in 0..INT_REGISTER_SIZE {
            print!("x{: <2} 0x{:>04x} ", i, self.get_int_register(i));
            if i % 8 == 7 {
                println!("");
            }
        }
        for i in 0..FLOAT_REGISTER_SIZE {
            print!("f{: <2} {:>10.5} ", i, self.get_float_register(i));
            if i % 8 == 7 {
                println!("");
            }
        }
    }

    pub fn show_memory(&self) {
        println!("memory");
        self.memory.show();
    }

    fn save_int_registers(&mut self) {
        let mut int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        for i in 0..INT_REGISTER_SIZE {
            int_registers[i].set(self.get_int_register(i));
        }
        self.int_registers_history.push(int_registers);
    }

    fn save_pc(&mut self) {
        self.pc_history.push(self.get_pc());
    }

    fn show_pc_buffer(&self) {
        print!("pc  ");
        for i in 0..self.pc_history.len() {
            print!("{:>8}  ", self.pc_history[i]);
        }
        println!("");
    }

    fn show_int_registers_buffer(&self) {
        let mut strings = vec![vec![]; INT_REGISTER_SIZE];
        for i in 0..self.int_registers_history.len() {
            for j in 0..INT_REGISTER_SIZE {
                strings[j].push(format!("{:>08x}", self.int_registers_history[i][j].get()));
            }
        }
        let mut line = String::from("");
        for _ in 0..self.int_registers_history.len() {
            line += "-----"
        }
        for i in 0..INT_REGISTER_SIZE {
            // println!("{}", line);
            print!("x{: <2} ", i);
            let mut before_string = String::from("");
            for j in 0..strings[i].len() {
                if before_string != strings[i][j] {
                    print!("{} |", strings[i][j]);
                    before_string = strings[i][j].clone();
                } else {
                    print!("---------|");
                }
            }
            println!("");
        }
    }

    pub fn run(&mut self, verbose: bool, interval: u64) {
        // let start_time = Instant::now();
        let mut inst_count = 0;
        let mut before_pc = std::usize::MAX;
        let mut same_pc_cnt = 0;
        let same_pc_limit = 5;
        self.save_pc();
        self.save_int_registers();
        if verbose {
            println!("");
            self.show_registers();
        }
        loop {
            if verbose {
                // colorized_println(&format!("pc: {}", self.get_pc()), BLUE);
                println!("pc: {} ({})", self.get_pc(), inst_count);
            }
            if interval != 0 {
                let interval_start_time = Instant::now();
                while interval_start_time.elapsed() < Duration::from_millis(interval) {}
            }
            let current_pc = self.get_pc();
            if current_pc == before_pc {
                same_pc_cnt += 1;
            } else {
                same_pc_cnt = 0;
                before_pc = current_pc;
            }
            let mut inst: [MemoryValue; 4] = [0; 4];
            for i in 0..4 {
                inst[i] = self.load_ubyte(current_pc + i);
            }
            exec_instruction(self, inst, verbose);
            inst_count += 1;
            if verbose {
                self.show_registers();
            }
            // if start_time.elapsed() > Duration::from_millis(1000) {
            //     println!("instruction counts: {}", inst_count);
            //     return;
            // }
            if same_pc_cnt >= same_pc_limit {
                println!("infinite loop detected.");
                break;
            }
            self.save_pc();
            self.save_int_registers();
        }
        print!("    ");
        for i in 0..self.pc_history.len() {
            print!("{:>8}  ", i);
        }
        println!("");
        self.show_pc_buffer();
        self.show_int_registers_buffer();
    }
}
