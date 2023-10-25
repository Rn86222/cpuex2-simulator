use std::time::Duration;
use std::time::Instant;

use crate::cache::*;
use crate::instruction::exec_instruction;
use crate::instruction_memory::*;
use crate::memory::*;
use crate::register::*;
use crate::types::*;
use crate::utils::*;

const INT_REGISTER_SIZE: usize = 32;
const FLOAT_REGISTER_SIZE: usize = 32;

pub struct Core {
    memory: Memory,
    cache: Cache,
    memory_access_count: usize,
    cache_hit_count: usize,
    instruction_memory: InstructionMemory,
    instruction_memory_access_count: usize,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
    int_registers_history: Vec<[IntRegister; INT_REGISTER_SIZE]>,
    pc_history: Vec<Address>,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cache = Cache::new();
        let memory_access_count = 0;
        let cache_hit_count = 0;
        let instruction_memory = InstructionMemory::new();
        let instruction_memory_access_count = 0;
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        let pc = 0;
        let int_registers_history = Vec::new();
        let pc_history = Vec::new();
        Core {
            memory,
            cache,
            memory_access_count,
            cache_hit_count,
            instruction_memory,
            instruction_memory_access_count,
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

    fn increment_instruction_memory_access_count(&mut self) {
        self.instruction_memory_access_count += 1;
    }

    pub fn get_instruction(&mut self, addr: Address) -> InstructionValue {
        self.increment_instruction_memory_access_count();
        self.instruction_memory.load(addr)
    }

    pub fn store_instruction(&mut self, addr: Address, inst: InstructionValue) {
        self.increment_instruction_memory_access_count();
        self.instruction_memory.store(addr, inst);
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

    fn increment_memory_access_count(&mut self) {
        self.memory_access_count += 1;
    }

    fn increment_cache_hit_count(&mut self) {
        self.cache_hit_count += 1;
    }

    fn process_cache_miss(&mut self, addr: Address) {
        let line_addr = addr & !((1 << self.cache.get_offset_bit_num()) - 1);
        let line = self.memory.get_cache_line(line_addr);
        let set_line_result = self.cache.set_line(line_addr, line);
        if set_line_result.is_some() {
            let evicted_line = set_line_result.unwrap();
            self.memory.set_cache_line(evicted_line);
        }
    }

    pub fn load_byte(&mut self, addr: Address) -> Byte {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_ubyte(addr);
        match cache_access {
            CacheAccess::HitUByte(value) => {
                self.increment_cache_hit_count();
                return u8_to_i8(value) as Byte;
            }
            CacheAccess::Miss => {
                let value = self.memory.load_byte(addr);
                self.process_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_ubyte(&mut self, addr: Address) -> UByte {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_ubyte(addr);
        match cache_access {
            CacheAccess::HitUByte(value) => {
                self.increment_cache_hit_count();
                return value;
            }
            CacheAccess::Miss => {
                let value = self.memory.load_ubyte(addr);
                self.process_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn store_byte(&mut self, addr: Address, value: Byte) {
        self.increment_memory_access_count();
        let cache_access = self.cache.set_ubyte(addr, i8_to_u8(value));
        match cache_access {
            CacheAccess::HitSet => {
                self.increment_cache_hit_count();
            }
            CacheAccess::Miss => {
                self.memory.store_byte(addr, value);
                self.process_cache_miss(addr);
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_half(&mut self, addr: Address) -> Half {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_uhalf(addr);
        match cache_access {
            CacheAccess::HitUHalf(value) => {
                self.increment_cache_hit_count();
                return u16_to_i16(value);
            }
            CacheAccess::Miss => {
                let value = self.memory.load_half(addr);
                self.process_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_uhalf(&mut self, addr: Address) -> UHalf {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_uhalf(addr);
        match cache_access {
            CacheAccess::HitUHalf(value) => {
                self.increment_cache_hit_count();
                return value;
            }
            CacheAccess::Miss => {
                let value = self.memory.load_uhalf(addr);
                self.process_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_word(&mut self, addr: Address) -> Word {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_word(addr);
        match cache_access {
            CacheAccess::HitWord(value) => {
                self.increment_cache_hit_count();
                return value;
            }
            CacheAccess::Miss => {
                let value = self.memory.load_word(addr);
                self.process_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn store_half(&mut self, addr: Address, value: Half) {
        self.increment_memory_access_count();
        let cache_access = self.cache.set_uhalf(addr, i16_to_u16(value));
        match cache_access {
            CacheAccess::HitSet => {
                self.increment_cache_hit_count();
            }
            CacheAccess::Miss => {
                self.memory.store_half(addr, value);
                self.process_cache_miss(addr);
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn store_word(&mut self, addr: Address, value: Word) {
        self.increment_memory_access_count();
        let cache_access = self.cache.set_word(addr, value);
        match cache_access {
            CacheAccess::HitSet => {
                self.increment_cache_hit_count();
            }
            CacheAccess::Miss => {
                self.memory.store_word(addr, value);
                self.process_cache_miss(addr);
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    // pub fn get_memory_byte(&self, addr: Address) -> String {
    //     self.memory.get_byte(addr)
    // }

    pub fn show_registers(&self) {
        for i in 0..INT_REGISTER_SIZE {
            print!("x{: <2} 0x{:>08x} ", i, self.get_int_register(i));
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

    // pub fn show_memory(&self) {
    //     println!("memory");
    //     self.memory.show();
    // }

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

    fn show_memory_access_info(&self) {
        println!("memory access count: {}", self.memory_access_count);
        println!("cache hit count: {}", self.cache_hit_count);
        println!(
            "cache hit rate: {:.5}%",
            self.cache_hit_count as f64 / self.memory_access_count as f64 * 100.0
        );
        println!(
            "instruction memory access count: {}",
            self.instruction_memory_access_count
        );
    }

    fn show_pc_stats(&self) {
        let mut pc_stats = vec![0; 1 << 20];
        for i in 0..self.pc_history.len() {
            pc_stats[self.pc_history[i] as usize] += 1;
        }
        let mut pc_stats_with_index = vec![];
        for i in 0..pc_stats.len() {
            pc_stats_with_index.push((i, pc_stats[i]));
        }
        pc_stats_with_index.sort_by(|a, b| b.1.cmp(&a.1));
        for i in 0..pc_stats_with_index.len() {
            if pc_stats_with_index[i].1 == 0 {
                break;
            }
            println!(
                "pc: {:>08x} count: {}",
                pc_stats_with_index[i].0, pc_stats_with_index[i].1
            );
        }
    }

    pub fn run(&mut self, verbose: bool, interval: u64) {
        // let start_time = Instant::now();
        let mut inst_count = 0;
        let mut before_pc = std::u32::MAX;
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
            // for i in 0..4 {
            //     inst[i] = self.load_ubyte(current_pc + i as Address);
            // }
            let instruction = self.get_instruction(current_pc);
            for i in 0..4 {
                inst[i] = ((instruction >> (i * 8)) & 0xff) as MemoryValue;
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
        if verbose {
            print!("    ");
            for i in 0..self.pc_history.len() {
                print!("{:>8}  ", i);
            }
            println!("");
            self.show_pc_buffer();
            self.show_int_registers_buffer();
        }
        self.show_memory_access_info();
        self.show_pc_stats();
    }
}
