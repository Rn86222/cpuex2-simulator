use std::collections::HashMap;
use std::fs::File;
// use std::io::BufRead;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::cache::*;
use crate::decoder::*;
use crate::fpu_emulator::*;
use crate::instruction::*;
use crate::instruction_memory::*;
use crate::memory::*;
use crate::register::*;
use crate::sld_loader::*;
use crate::types::*;
use crate::utils::*;

const INT_REGISTER_SIZE: usize = 32;
const FLOAT_REGISTER_SIZE: usize = 32;
const IO_ADDRESS: Address = 2147483648;

pub struct Core {
    memory: Memory,
    cache: Cache,
    memory_access_count: usize,
    cache_hit_count: usize,
    instruction_memory: InstructionMemory,
    instruction_memory_access_count: usize,
    instruction_count: InstructionCount,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
    int_registers_history: Vec<[IntRegister; INT_REGISTER_SIZE]>,
    float_registers_history: Vec<[FloatRegister; FLOAT_REGISTER_SIZE]>,
    instruction_count_history: Vec<InstructionCount>,
    pc_history: Vec<Address>,
    pc_stats: HashMap<Address, (Instruction, usize)>,
    inst_stats: HashMap<String, usize>,
    fetched_instruction: Option<InstructionValue>,
    decoded_instruction: Option<InstructionEnum>,
    instruction_in_exec_stage: Option<InstructionEnum>,
    instruction_in_memory_stage: Option<InstructionEnum>,
    instruction_in_write_back_stage: Option<InstructionEnum>,
    forwarding_int_source_map: HashMap<Rs, (InstructionCount, Int)>,
    forwarding_float_source_map: HashMap<Rs, (InstructionCount, FloatingPoint)>,
    inv_map: InvMap,
    sqrt_map: SqrtMap,
    sld_vec: Vec<String>,
    sld_counter: usize,
    output: Vec<u8>,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cache = Cache::new();
        let memory_access_count = 0;
        let cache_hit_count = 0;
        let instruction_memory = InstructionMemory::new();
        let instruction_memory_access_count = 0;
        let instruction_count = 0;
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        let pc = 0;
        let int_registers_history = Vec::new();
        let float_registers_history = Vec::new();
        let pc_history = Vec::new();
        let instruction_count_history = Vec::new();
        let pc_stats = HashMap::new();
        let inst_stats = HashMap::new();
        let fetched_instruction = None;
        let decoded_instruction = None;
        let instruction_in_exec_stage = None;
        let instruction_in_memory_stage = None;
        let instruction_in_write_back_stage = None;
        let forwarding_int_source_map = HashMap::new();
        let forwarding_float_source_map = HashMap::new();
        let inv_map = create_inv_map();
        let sqrt_map = create_sqrt_map();
        let sld_vec = vec![];
        let sld_counter = 0;
        let output = vec![];
        Core {
            memory,
            cache,
            memory_access_count,
            cache_hit_count,
            instruction_memory,
            instruction_memory_access_count,
            instruction_count,
            int_registers,
            float_registers,
            pc,
            int_registers_history,
            float_registers_history,
            pc_history,
            instruction_count_history,
            pc_stats,
            inst_stats,
            fetched_instruction,
            decoded_instruction,
            instruction_in_exec_stage,
            instruction_in_memory_stage,
            instruction_in_write_back_stage,
            forwarding_int_source_map,
            forwarding_float_source_map,
            inv_map,
            sqrt_map,
            sld_vec,
            sld_counter,
            output,
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

    pub fn get_inv_map(&self) -> &InvMap {
        &self.inv_map
    }

    pub fn get_sqrt_map(&self) -> &SqrtMap {
        &self.sqrt_map
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
        if let Some(inst) = self.get_instruction_in_exec_stage() {
            let jump_address = get_jump_address(inst);
            if let Some(jump_address) = jump_address {
                self.set_pc(jump_address);
                // flush instruction in IF and ID stage
                self.fetched_instruction = None;
                self.decoded_instruction = None;
            } else if !stalling {
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

    pub fn load_instruction(&mut self, addr: Address) -> InstructionValue {
        self.increment_instruction_memory_access_count();
        self.instruction_memory.load(addr)
    }

    pub fn store_instruction(&mut self, addr: Address, inst: InstructionValue) {
        self.instruction_memory.store(addr, inst);
    }

    pub fn get_int_register(&self, index: usize) -> Int {
        self.int_registers[index].get()
    }

    pub fn set_int_register(&mut self, index: usize, value: Int) {
        if index == ZERO {
            return; // zero register
        }
        self.int_registers[index].set(value);
    }

    pub fn get_float_register(&self, index: usize) -> FloatingPoint {
        self.float_registers[index].get()
    }

    pub fn set_float_register(&mut self, index: usize, value: FloatingPoint) {
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
        if let Some(evicted_line) = set_line_result {
            self.memory.set_cache_line(evicted_line);
        }
    }

    pub fn load_byte(&mut self, addr: Address) -> Byte {
        self.increment_memory_access_count();
        let cache_access = self.cache.get_ubyte(addr);
        match cache_access {
            CacheAccess::HitUByte(value) => {
                self.increment_cache_hit_count();
                u8_to_i8(value) as Byte
            }
            CacheAccess::Miss => {
                let value = self.memory.load_byte(addr);
                self.process_cache_miss(addr);
                value
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
                value
            }
            CacheAccess::Miss => {
                let value = self.memory.load_ubyte(addr);
                self.process_cache_miss(addr);
                value
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
                u16_to_i16(value)
            }
            CacheAccess::Miss => {
                let value = self.memory.load_half(addr);
                self.process_cache_miss(addr);
                value
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
                value
            }
            CacheAccess::Miss => {
                let value = self.memory.load_uhalf(addr);
                self.process_cache_miss(addr);
                value
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_word(&mut self, addr: Address) -> Word {
        if addr == IO_ADDRESS {
            let value = self.sld_vec[self.sld_counter].parse::<i32>().unwrap();
            self.sld_counter += 1;
            return value;
        }
        self.increment_memory_access_count();
        let cache_access = self.cache.get_word(addr);
        match cache_access {
            CacheAccess::HitWord(value) => {
                self.increment_cache_hit_count();
                value
            }
            CacheAccess::Miss => {
                let value = self.memory.load_word(addr);
                self.process_cache_miss(addr);
                value
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn load_word_fp(&mut self, addr: Address) -> Word {
        if addr == IO_ADDRESS {
            let value = self.sld_vec[self.sld_counter].parse::<f32>().unwrap();
            let fp = FloatingPoint::new_f32(value);
            self.sld_counter += 1;
            u32_to_i32(fp.get_32_bits())
        } else {
            self.load_word(addr)
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
        if addr == IO_ADDRESS {
            self.output.push(value as u8);
            return;
        }
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

    // pub fn load_data_file(&mut self, path: &str) {
    //     if let Ok(file) = File::open(path) {
    //         let reader = std::io::BufReader::new(file);
    //         for line in reader.lines().flatten() {
    //             let mut iter = line.split_whitespace();
    //             let addr = iter.next().unwrap().parse::<Address>().unwrap();
    //             let value = u32_to_i32(iter.next().unwrap().parse::<u32>().unwrap());
    //             self.memory.store_word(addr, value);
    //         }
    //     } else {
    //         eprintln!(
    //             "Warning: failed to open file for data section (dismiss if you don't need it)."
    //         );
    //     }
    // }

    pub fn move_instructions_to_next_stage(&mut self) {
        self.instruction_in_write_back_stage = self.instruction_in_memory_stage.clone();
        self.instruction_in_memory_stage = self.instruction_in_exec_stage.clone();
        self.instruction_in_exec_stage = self.decoded_instruction.clone();
        if let Some(fetched_instruction) = self.fetched_instruction {
            let decoded = decode_instruction(fetched_instruction);
            if let Instruction::Other = decoded {
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

    pub fn get_forwarding_int_source(&self, rs: Rs) -> Option<&(InstructionCount, Int)> {
        self.forwarding_int_source_map.get(&rs)
    }

    pub fn set_forwarding_int_source(&mut self, rs: Rs, inst_cnt: InstructionCount, value: Int) {
        if rs == 0 {
            return;
        }
        self.forwarding_int_source_map.insert(rs, (inst_cnt, value));
    }

    pub fn get_forwarding_float_source(
        &self,
        rs: Rs,
    ) -> Option<&(InstructionCount, FloatingPoint)> {
        self.forwarding_float_source_map.get(&rs)
    }

    pub fn set_forwarding_float_source(
        &mut self,
        rs: Rs,
        inst_cnt: InstructionCount,
        value: FloatingPoint,
    ) {
        if rs == 0 {
            return;
        }
        self.forwarding_float_source_map
            .insert(rs, (inst_cnt, value));
    }

    fn remove_forwarding_source_if_possible(&mut self) {
        if let Some(inst) = self.get_instruction_in_write_back_stage() {
            let current_inst_cnt = get_instruction_count(inst).unwrap();
            let rd = get_destination_register(inst);
            if let Some(rd) = rd {
                match rd {
                    RegisterId::Int(rd) => {
                        if rd == 0 {
                            return;
                        }
                        let int_source = self.forwarding_int_source_map.get(&rd);
                        if let Some((inst_cnt, _)) = int_source {
                            if *inst_cnt == current_inst_cnt {
                                self.forwarding_int_source_map.remove(&rd);
                            }
                        }
                    }
                    RegisterId::Float(rd) => {
                        let float_source = self.forwarding_float_source_map.get(&rd);
                        if let Some((inst_cnt, _)) = float_source {
                            if *inst_cnt == current_inst_cnt {
                                self.forwarding_float_source_map.remove(&rd);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_load_hazard(&self) -> bool {
        if self.decoded_instruction.is_none() || self.instruction_in_exec_stage.is_none() {
            return false;
        }
        let decoded_instruction = self.decoded_instruction.as_ref().unwrap();
        let instruction_in_exec_stage = self.instruction_in_exec_stage.as_ref().unwrap();
        if !is_load_instruction(instruction_in_exec_stage) {
            return false;
        }
        let rd = get_destination_register(instruction_in_exec_stage);
        let rss = get_source_registers(decoded_instruction);
        if let Some(rd) = rd {
            for rs in &rss {
                if *rs == rd {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_instruction_count(&self) -> InstructionCount {
        self.instruction_count
    }

    #[allow(dead_code)]
    pub fn show_registers(&self) {
        for i in 0..INT_REGISTER_SIZE {
            print!("x{: <2} 0x{:>08x} ", i, self.get_int_register(i));
            if i % 8 == 7 {
                println!();
            }
        }
        for i in 0..FLOAT_REGISTER_SIZE {
            print!(
                "f{: <2} 0x{:>08x} ",
                i,
                self.get_float_register(i).get_32_bits()
            );
            if i % 8 == 7 {
                println!();
            }
        }
    }

    #[allow(dead_code)]
    fn show_pipeline(&self) {
        // println!(
        //     "IF                  ID                  EX                  MEM                 WB"
        // );
        let if_string = if let Some(inst) = self.fetched_instruction {
            format!("{:>08x}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&if_string, 20);
        let id_string = if let Some(inst) = self.decoded_instruction.clone() {
            format!("{:?}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&id_string, 20);
        let ex_string = if let Some(inst) = self.instruction_in_exec_stage.clone() {
            format!("{:?}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&ex_string, 20);
        let mem_string = if let Some(inst) = self.instruction_in_memory_stage.clone() {
            format!("{:?}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&mem_string, 20);
        let wb_string = if let Some(inst) = self.instruction_in_write_back_stage.clone() {
            format!("{:?}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&wb_string, 0);
        println!();
    }

    // pub fn show_memory(&self) {
    //     println!("memory");
    //     self.memory.show();
    // }

    #[allow(dead_code)]
    fn add_int_registers_to_history(&mut self) {
        let mut int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        for (i, int_register) in int_registers.iter_mut().enumerate() {
            int_register.set(self.get_int_register(i));
        }
        self.int_registers_history.push(int_registers);
    }

    #[allow(dead_code)]
    fn add_float_registers_to_history(&mut self) {
        let mut float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        for (i, float_register) in float_registers.iter_mut().enumerate() {
            float_register.set(self.get_float_register(i));
        }
        self.float_registers_history.push(float_registers);
    }

    #[allow(dead_code)]
    fn add_registers_to_history(&mut self) {
        self.add_int_registers_to_history();
        self.add_float_registers_to_history();
    }

    #[allow(dead_code)]
    fn add_pc_to_history(&mut self) {
        self.pc_history.push(self.get_pc());
    }

    #[allow(dead_code)]
    fn add_instruction_count_to_history(&mut self) {
        self.instruction_count_history
            .push(self.get_instruction_count());
    }

    #[allow(dead_code)]
    fn update_pc_stats(&mut self) {
        if let Some(inst) = self.fetched_instruction {
            let decoded = decode_instruction(inst);
            if let Instruction::Other = decoded {
                return;
            }
            let pc = self.get_pc();
            self.pc_stats
                .entry(pc)
                .and_modify(|e| e.1 += 1)
                .or_insert((decoded, 1));
        }
    }

    fn show_pc_stats(&self) {
        println!("---------- pc stats ----------");
        let mut pc_stats = vec![];
        for (pc, (decoded, inst_count)) in &self.pc_stats {
            let inst = create_instruction_struct(*decoded);
            let inst_name = get_name(&inst);
            pc_stats.push((pc, inst_name, inst_count));
        }
        pc_stats.sort_by(|a, b| b.2.cmp(a.2));
        for pc_stat in &pc_stats {
            let pc_inst_string = format!("{:>08}({})", pc_stat.0, pc_stat.1);
            print_filled_with_space(&pc_inst_string, 25);
            println!("{}", pc_stat.2);
        }
    }

    fn update_inst_stats(&mut self) {
        if let Some(inst) = self.get_instruction_in_exec_stage() {
            self.inst_stats
                .entry(get_name(inst))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    fn show_inst_stats(&self) {
        println!("---------- inst stats ----------");
        let mut inst_stats = vec![];
        for (inst_name, inst_count) in &self.inst_stats {
            inst_stats.push((inst_name, inst_count));
        }
        inst_stats.sort_by(|a, b| b.1.cmp(a.1));
        for inst_stat in &inst_stats {
            print_filled_with_space(&inst_stat.0.to_string(), 8);
            println!(" {}", inst_stat.1);
        }
    }

    #[allow(dead_code)]
    fn show_instruction_count_history(&self) {
        print!("    ");
        for i in 0..self.instruction_count_history.len() {
            print!("{:>8}  ", self.instruction_count_history[i]);
        }
        println!();
    }

    #[allow(dead_code)]
    fn show_pc_history(&self) {
        print!("pc  ");
        for i in 0..self.pc_history.len() {
            print!("{:>8}  ", self.pc_history[i]);
        }
        println!();
    }

    #[allow(dead_code)]
    fn show_int_registers_history(&self) {
        let mut strings = vec![vec![]; INT_REGISTER_SIZE];
        for i in 0..self.int_registers_history.len() {
            for (j, string) in strings.iter_mut().enumerate() {
                string.push(format!("{:>08x}", self.int_registers_history[i][j].get()));
            }
        }
        let mut line = String::from("");
        for _ in 0..self.int_registers_history.len() {
            line += "-----"
        }
        for (i, string) in strings.iter().enumerate() {
            print!("x{: <2} ", i);
            let mut before_string = String::from("");
            for value in string {
                if before_string != *value {
                    print!("{} |", value);
                    before_string = value.clone();
                } else {
                    print!("---------|");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn show_float_registers_history(&self) {
        let mut strings = vec![vec![]; INT_REGISTER_SIZE];
        for i in 0..self.float_registers_history.len() {
            for (j, string) in strings.iter_mut().enumerate() {
                let value = self.float_registers_history[i][j].get();
                string.push(format!("{:>08x}", value.get_32_bits()));
            }
        }
        let mut line = String::from("");
        for _ in 0..self.float_registers_history.len() {
            line += "-----"
        }
        for (i, string) in strings.iter().enumerate() {
            print!("f{: <2} ", i);
            let mut before_string = String::from("");
            for value in string {
                if before_string != *value {
                    print!("{} |", value);
                    before_string = value.clone();
                } else {
                    print!("---------|");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn show_register_history(&self) {
        self.show_int_registers_history();
        self.show_float_registers_history();
    }

    fn show_memory_stats(&self) {
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

    fn output_pc_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let mut pc_count = 0;
        loop {
            let inst = self.instruction_memory.load(pc_count as Address);
            let decoded = decode_instruction(inst);
            match decoded {
                Instruction::Other => {
                    break;
                }
                _ => {
                    let inst = create_instruction_struct(decoded);
                    let inst_string = format!("{}: {}", pc_count, get_name(&inst));
                    file.write_all(inst_string.as_bytes()).unwrap();
                    file.write_all("\n".as_bytes()).unwrap();
                    pc_count += 4;
                }
            }
        }
    }

    fn show_current_state(&self) {
        eprint!(
            "\r{} {:>08} pc: {:>06} sp: {:>010}",
            self.instruction_count,
            self.output.len(),
            self.get_pc() - 4,
            self.get_int_register(2),
        );
        if let Some(inst) = self.get_instruction_in_exec_stage() {
            let inst_string = format!("{:?}", inst);
            eprint!("  {:?}         ", inst_string);
        } else {
            eprint!("               ");
        }
    }

    fn show_output_result(&self) {
        println!("---------- output ----------");
        for i in 0..self.output.len() {
            println!(
                "{} {} 0x{:>02x} {}",
                i, self.output[i], self.output[i], self.output[i] as char
            );
        }
    }

    fn load_sld_file(&mut self, path: &str) {
        self.sld_vec = load_sld_file(path);
    }

    pub fn run(
        &mut self,
        verbose: u32,
        interval: u64,
        // data_file_path: &str,
        ppm_file_path: &str,
        sld_file_path: &str,
        pc_file_path: &str,
    ) {
        let start_time = Instant::now();
        let mut will_stall = false;
        let mut stalling;
        let mut cycle_num: u128 = 0;

        let mut ppm_file = File::create(ppm_file_path).unwrap();
        let mut before_output_len = 0;

        self.output_pc_file(pc_file_path);

        // self.load_data_file(data_file_path);
        self.load_sld_file(sld_file_path);

        self.update_pc_stats();

        if verbose == 2 {
            self.show_registers();
            self.add_registers_to_history();
            self.add_pc_to_history();
        }

        // let guard = pprof::ProfilerGuardBuilder::default()
        //     .frequency(1000)
        //     .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        //     .build()
        //     .unwrap();

        loop {
            if verbose >= 1 {
                // colorized_println(&format!("pc: {}", self.get_pc()), BLUE);
                let pc_string = format!("pc: {}", self.get_pc());
                print_filled_with_space(&pc_string, 15);
            }
            cycle_num += 1;
            if interval != 0 {
                thread::sleep(Duration::from_millis(interval));
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
                self.update_inst_stats();
                self.update_pc_stats();
            } else {
                register_fetch(self);
            }

            self.set_next_pc(will_stall);

            self.remove_forwarding_source_if_possible();

            if cycle_num % 1000000 == 0 {
                self.show_current_state();
            }
            if before_output_len != self.output.len() {
                for i in before_output_len..self.output.len() {
                    let byte = [self.output[i]];
                    ppm_file.write_all(&byte).unwrap();
                }
                before_output_len = self.output.len();
            }
            if verbose >= 1 {
                self.show_pipeline();
            }
            if verbose == 2 {
                self.show_registers();
                self.add_registers_to_history();
                self.add_pc_to_history();
                self.add_instruction_count_to_history();
            }
        }

        // if let Ok(report) = guard.report().build() {
        //     let file = File::create("flamegraph_256.svg").unwrap();
        //     report.flamegraph(file).unwrap();
        // };

        println!(
            "inst_count: {}\nelapsed time: {:?}\n{:.2} MIPS",
            self.instruction_count,
            start_time.elapsed(),
            self.instruction_count as f64 / start_time.elapsed().as_micros() as f64
        );
        if verbose == 2 {
            self.show_instruction_count_history();
            self.show_pc_history();
            self.show_register_history();
        }
        self.show_memory_stats();
        self.show_output_result();
        self.show_inst_stats();
        self.show_pc_stats();
    }
}

pub fn disassemble(buf: &Vec<u8>, path: &str) {
    let mut inst_count = 0;
    let mut inst = 0;
    let mut core = Core::new();
    for &byte in buf {
        inst += (byte as u32) << ((inst_count % 4) * 8);
        inst_count += 1;
        if inst_count % 4 == 0 {
            core.store_instruction(inst_count - 4, inst);
            inst = 0;
        }
    }
    let mut file = File::create(path).unwrap();
    let mut pc_count = 0;
    loop {
        let inst = core.instruction_memory.load(pc_count as Address);
        let decoded = decode_instruction(inst);
        match decoded {
            Instruction::Other => {
                break;
            }
            _ => {
                let inst = create_instruction_struct(decoded);
                core.set_decoded_instruction(Some(inst));
                core.increment_pc();
                register_fetch(&mut core);
                let inst = core.decoded_instruction.clone().unwrap();
                let inst_string = format!("{}: {:?}", pc_count, inst);
                file.write_all(inst_string.as_bytes()).unwrap();
                file.write_all("\n".as_bytes()).unwrap();
                pc_count += 4;
            }
        }
    }
}
