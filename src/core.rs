use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use crate::cache::*;
use crate::decoder::*;
use crate::instruction::*;
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
    // instruction_cache: InstructionCache,
    instruction_memory_access_count: usize,
    instruction_count: InstructionCount,
    // instruction_cache_hit_count: usize,
    // instruction_maps: InstructionMaps,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
    int_registers_history: Vec<[IntRegister; INT_REGISTER_SIZE]>,
    pc_history: Vec<Address>,
    fetched_instruction: Option<InstructionValue>,
    decoded_instruction: Option<InstructionEnum>,
    instruction_in_exec_stage: Option<InstructionEnum>,
    instruction_in_memory_stage: Option<InstructionEnum>,
    instruction_in_write_back_stage: Option<InstructionEnum>,
    forwarding_source_map: HashMap<Rs, (InstructionCount, Int)>,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cache = Cache::new();
        let memory_access_count = 0;
        let cache_hit_count = 0;
        let instruction_memory = InstructionMemory::new();
        // let instruction_cache = InstructionCache::new();
        let instruction_memory_access_count = 0;
        let instruction_count = 0;
        // let instruction_cache_hit_count = 0;
        // let instruction_maps = InstructionMaps::new();
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        let pc = 0;
        let int_registers_history = Vec::new();
        let pc_history = Vec::new();
        let fetched_instruction = None;
        let decoded_instruction = None;
        let instruction_in_exec_stage = None;
        let instruction_in_memory_stage = None;
        let instruction_in_write_back_stage = None;
        let forwarding_source_map = HashMap::new();
        Core {
            memory,
            cache,
            memory_access_count,
            cache_hit_count,
            instruction_memory,
            // instruction_cache,
            instruction_memory_access_count,
            instruction_count,
            // instruction_cache_hit_count,
            // instruction_maps,
            int_registers,
            float_registers,
            pc,
            int_registers_history,
            pc_history,
            fetched_instruction,
            decoded_instruction,
            instruction_in_exec_stage,
            instruction_in_memory_stage,
            instruction_in_write_back_stage,
            forwarding_source_map,
        }
    }

    pub fn get_decoded_instruction(&self) -> &Option<InstructionEnum> {
        &self.decoded_instruction
    }

    pub fn set_decoded_instruction(&mut self, inst: Option<InstructionEnum>) {
        self.decoded_instruction = inst;
    }

    pub fn get_instruction_in_exec_stage(&self) -> &Option<InstructionEnum> {
        &self.instruction_in_exec_stage
    }

    pub fn set_instruction_in_exec_stage(&mut self, inst: Option<InstructionEnum>) {
        self.instruction_in_exec_stage = inst;
    }

    pub fn get_instruction_in_memory_stage(&self) -> &Option<InstructionEnum> {
        &self.instruction_in_memory_stage
    }

    pub fn set_instruction_in_memory_stage(&mut self, inst: Option<InstructionEnum>) {
        self.instruction_in_memory_stage = inst;
    }

    pub fn get_instruction_in_write_back_stage(&self) -> &Option<InstructionEnum> {
        &self.instruction_in_write_back_stage
    }

    fn fetch_instruction(&mut self) {
        let current_pc = self.get_pc();
        self.fetched_instruction = Some(self.load_instruction(current_pc));
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

    fn set_next_pc(&mut self, stalling: bool) {
        if self.instruction_in_exec_stage.is_some() {
            let jump_address = get_jump_address(&self.instruction_in_exec_stage.clone().unwrap());
            if jump_address.is_some() {
                self.set_pc(jump_address.unwrap());
                self.fetched_instruction = None;
                self.decoded_instruction = None;
            } else {
                self.increment_pc();
            }
        } else if !stalling {
            self.increment_pc();
        }
    }

    fn increment_instruction_memory_access_count(&mut self) {
        self.instruction_memory_access_count += 1;
    }

    fn increment_instruction_count(&mut self) {
        self.instruction_count += 1;
    }

    // fn increment_instruction_cache_hit_count(&mut self) {
    //     self.instruction_cache_hit_count += 1;
    // }

    // fn process_instruction_cache_miss(&mut self, addr: Address) {
    //     let line_addr = addr & !((1 << self.instruction_cache.get_offset_bit_num()) - 1);
    //     let line = self.instruction_memory.get_cache_line(line_addr);
    //     let set_line_result = self.instruction_cache.set_line(line_addr, line);
    //     if set_line_result.is_some() {
    //         let evicted_line = set_line_result.unwrap();
    //         self.instruction_memory.set_cache_line(evicted_line);
    //     }
    // }

    pub fn load_instruction(&mut self, addr: Address) -> InstructionValue {
        self.increment_instruction_memory_access_count();
        self.instruction_memory.load(addr)

        // let cache_access = self.instruction_cache.get(addr);
        // match cache_access {
        //     InstructionCacheAccess::HitGet(value) => {
        //         self.increment_instruction_cache_hit_count();
        //         return value;
        //     }
        //     InstructionCacheAccess::Miss => {
        //         let value = self.instruction_memory.load(addr);
        //         self.process_instruction_cache_miss(addr);
        //         return value;
        //     }
        //     _ => {
        //         panic!("invalid cache access");
        //     }
        // }
    }

    pub fn store_instruction(&mut self, addr: Address, inst: InstructionValue) {
        // self.increment_instruction_memory_access_count();
        self.instruction_memory.store(addr, inst);

        // let cache_access = self.instruction_cache.set(addr, inst);
        // match cache_access {
        //     InstructionCacheAccess::HitSet => {
        //         println!("store hit");
        //         self.increment_instruction_cache_hit_count();
        //     }
        //     InstructionCacheAccess::Miss => {
        //         println!("store miss");
        //         self.instruction_memory.store(addr, inst);
        //         self.process_instruction_cache_miss(addr);
        //     }
        //     _ => {
        //         panic!("invalid cache access");
        //     }
        // }
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

    pub fn move_instructions_to_next_stage(&mut self) {
        self.instruction_in_write_back_stage = self.instruction_in_memory_stage.clone();
        self.instruction_in_memory_stage = self.instruction_in_exec_stage.clone();
        self.instruction_in_exec_stage = self.decoded_instruction.clone();
        if self.fetched_instruction.is_some() {
            let decoded = decode_instruction(self.fetched_instruction.unwrap());
            if let Instruction::OtherInstruction = decoded {
                self.decoded_instruction = None;
                self.fetched_instruction = None;
                return;
            } else {
                let decoded_inst_struct = create_instruction_struct(decoded);
                self.decoded_instruction = Some(decoded_inst_struct);
            }
        } else {
            self.decoded_instruction = None;
        }
        self.fetched_instruction = None;
    }

    pub fn move_instructions_to_next_stage_stalling(&mut self) {
        self.instruction_in_write_back_stage = self.instruction_in_memory_stage.clone();
        self.instruction_in_memory_stage = self.instruction_in_exec_stage.clone();
        self.instruction_in_exec_stage = None;
    }

    pub fn get_forwarding_source(&self, rs: Rs) -> Option<&(InstructionCount, Int)> {
        self.forwarding_source_map.get(&rs)
    }

    pub fn set_forwarding_source(&mut self, rs: Rs, inst_cnt: InstructionCount, value: Int) {
        self.forwarding_source_map.insert(rs, (inst_cnt, value));
    }

    fn remove_forwarding_source_if_possible_sub(&mut self, inst: &InstructionEnum) {
        let current_inst_cnt = get_instruction_count(inst).unwrap();
        let rd = get_destination_register(inst);
        if rd.is_some() {
            let rd = rd.unwrap();
            let source = self.forwarding_source_map.get(&rd);
            match source {
                Some((inst_cnt, _)) => {
                    if *inst_cnt == current_inst_cnt {
                        self.forwarding_source_map.remove(&rd);
                    }
                }
                None => {
                    panic!();
                }
            }
        }
    }

    fn remove_forwarding_source_if_possible(&mut self) {
        if self.get_instruction_in_write_back_stage().is_some() {
            let inst = self.get_instruction_in_write_back_stage().clone().unwrap();
            self.remove_forwarding_source_if_possible_sub(&inst);
        }
    }

    pub fn check_load_hazard(&self) -> bool {
        if self.decoded_instruction.is_none() || self.instruction_in_exec_stage.is_none() {
            return false;
        }
        let decoded_instruction = self.decoded_instruction.clone().unwrap();
        let instruction_in_exec_stage = self.instruction_in_exec_stage.clone().unwrap();
        if !is_load_instruction(&instruction_in_exec_stage) {
            return false;
        }
        let rd = get_destination_register(&instruction_in_exec_stage);
        let rss = get_source_registers(&decoded_instruction);
        if rd.is_some() {
            let rd = rd.unwrap();
            for i in 0..rss.len() {
                if rss[i] == rd {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn get_instruction_count(&self) -> InstructionCount {
        self.instruction_count
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
        // for i in 0..FLOAT_REGISTER_SIZE {
        //     print!("f{: <2} {:>10.5} ", i, self.get_float_register(i));
        //     if i % 8 == 7 {
        //         println!("");
        //     }
        // }
    }

    fn show_pipeline(&self) {
        // println!(
        //     "IF                  ID                  EX                  MEM                 WB"
        // );
        let if_string = if self.fetched_instruction.is_some() {
            format!("{:>08x}", self.fetched_instruction.clone().unwrap())
        } else {
            format!("None")
        };
        print_filled_with_space(&if_string, 20);
        let id_string = if self.decoded_instruction.is_some() {
            format!("{:?}", self.decoded_instruction.clone().unwrap())
        } else {
            format!("None")
        };
        print_filled_with_space(&id_string, 20);
        let ex_string = if self.instruction_in_exec_stage.is_some() {
            format!("{:?}", self.instruction_in_exec_stage.clone().unwrap())
        } else {
            format!("None")
        };
        print_filled_with_space(&ex_string, 20);
        let mem_string = if self.instruction_in_memory_stage.is_some() {
            format!("{:?}", self.instruction_in_memory_stage.clone().unwrap())
        } else {
            format!("None")
        };
        print_filled_with_space(&mem_string, 20);
        let wb_string = if self.instruction_in_write_back_stage.is_some() {
            format!(
                "{:?}",
                self.instruction_in_write_back_stage.clone().unwrap()
            )
        } else {
            format!("None")
        };
        print_filled_with_space(&wb_string, 0);
        println!("");
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

    fn save_pc(&mut self, stalling: bool) {
        if stalling {
            return;
        }
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
        // println!(
        //     "instruction cache hit count: {}",
        //     self.instruction_cache_hit_count
        // );
        // println!(
        //     "instruction cache hit rate: {:.5}%",
        //     self.instruction_cache_hit_count as f64 / self.instruction_memory_access_count as f64
        //         * 100.0
        // );
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
            let inst = decode_instruction(
                self.instruction_memory
                    .load(pc_stats_with_index[i].0 as Address),
            );
            if let Instruction::OtherInstruction = inst {
                continue;
            }
            let inst = create_instruction_struct(inst);
            let pc_inst_string =
                format!("pc: {:>08x}({})", pc_stats_with_index[i].0, get_name(&inst));
            print_filled_with_space(&pc_inst_string, 25);
            println!("{}", pc_stats_with_index[i].1);
        }
    }

    pub fn run(&mut self, verbose: bool, interval: u64) {
        let start_time = Instant::now();
        let mut will_stall = false;
        let mut stalling;
        self.save_pc(false);
        self.save_int_registers();
        if verbose {
            println!("");
            self.show_registers();
        }
        loop {
            if verbose {
                // colorized_println(&format!("pc: {}", self.get_pc()), BLUE);
                let pc_string = format!("pc: {}", self.get_pc());
                print_filled_with_space(&pc_string, 15);
            }
            if interval != 0 {
                let interval_start_time = Instant::now();
                while interval_start_time.elapsed() < Duration::from_millis(interval) {}
            }
            if self.get_pc() >= INSTRUCTION_MEMORY_SIZE as Address {
                self.pc_history.pop();
                println!("End of program.");
                break;
            }

            stalling = false;

            if !will_stall {
                self.move_instructions_to_next_stage();
            } else {
                self.move_instructions_to_next_stage_stalling();
                stalling = true;
                will_stall = false;
            }

            if self.check_load_hazard() {
                will_stall = true;
                // if verbose {
                //     println!("stalling");
                // }
            }

            write_back(self);
            memory_access(self);

            if !stalling {
                exec_instruction(self);
                self.increment_instruction_count();
                if !will_stall {
                    register_fetch(self);
                }
                self.fetch_instruction();
            } else {
                register_fetch(self);
            }

            self.set_next_pc(stalling);

            self.remove_forwarding_source_if_possible();

            if verbose {
                self.show_pipeline();
                // self.show_registers();
            }

            self.save_pc(stalling);
            self.save_int_registers();
        }

        println!(
            "inst_count: {}\nelapsed time: {:?}\n{:.2} MIPS",
            self.instruction_count,
            start_time.elapsed(),
            self.instruction_count as f64 / start_time.elapsed().as_micros() as f64
        );
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
