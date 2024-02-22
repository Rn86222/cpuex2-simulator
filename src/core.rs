use fxhash::FxHashMap;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::Instant;

// use crate::cache::*;
use crate::decoder::*;
use crate::fpu_emulator::*;
use crate::instruction::*;
use crate::instruction_memory::*;
use crate::memory::*;
use crate::pseudo_lru_cache::*;
use crate::register::*;
use crate::sld_loader::*;
use crate::types::*;
use crate::utils::*;

const INT_REGISTER_SIZE: usize = 64;
const FLOAT_REGISTER_SIZE: usize = 64;

pub struct Core {
    memory: Memory,
    cache: PseudoLRUCache,
    memory_access_count: usize,
    cache_hit_count: usize,
    instruction_memory: InstructionMemory,
    instruction_memory_access_count: usize,
    instruction_count: InstructionCount,
    cycle_count: u128,
    int_registers: [IntRegister; INT_REGISTER_SIZE],
    float_registers: [FloatRegister; FLOAT_REGISTER_SIZE],
    pc: Address,
    int_registers_history: Vec<[IntRegister; INT_REGISTER_SIZE]>,
    float_registers_history: Vec<[FloatRegister; FLOAT_REGISTER_SIZE]>,
    cycle_count_history: Vec<InstructionCount>,
    pc_history: Vec<Address>,
    pc_stats: FxHashMap<Address, (Instruction, usize)>,
    inst_stats: FxHashMap<String, usize>,
    instruction_in_fetch_stage: Option<InstructionValue>,
    instruction_in_fetch_stage_2: Option<InstructionValue>,
    instruction_in_decode_stage: Option<InstructionEnum>,
    instruction_in_exec_stage: Option<InstructionEnum>,
    instruction_in_memory_stage: Option<InstructionEnum>,
    instruction_in_write_back_stage: Option<InstructionEnum>,
    forwarding_int_sources: [Option<(InstructionCount, Int)>; INT_REGISTER_SIZE],
    forwarding_float_sources: [Option<(InstructionCount, FloatingPoint)>; FLOAT_REGISTER_SIZE],
    inv_map: InvMap,
    sqrt_map: SqrtMap,
    sld_vec: Vec<String>,
    sld_counter: usize,
    output: Vec<u8>,
}

impl Core {
    pub fn new() -> Self {
        let memory = Memory::new();
        let cache = PseudoLRUCache::new();
        let memory_access_count = 0;
        let cache_hit_count = 0;
        let instruction_memory = InstructionMemory::new();
        let instruction_memory_access_count = 0;
        let instruction_count = 0;
        let cycle_count = 0;
        let int_registers = [IntRegister::new(); INT_REGISTER_SIZE];
        let float_registers = [FloatRegister::new(); FLOAT_REGISTER_SIZE];
        let pc = 0;
        let int_registers_history = Vec::new();
        let float_registers_history = Vec::new();
        let pc_history = Vec::new();
        let instruction_count_history = Vec::new();
        let pc_stats = FxHashMap::default();
        let inst_stats = FxHashMap::default();
        let instruction_in_fetch_stage = None;
        let instruction_in_fetch_stage_2 = None;
        let decoded_instruction = None;
        let instruction_in_exec_stage = None;
        let instruction_in_memory_stage = None;
        let instruction_in_write_back_stage = None;
        let forwarding_int_sources = [None; INT_REGISTER_SIZE];
        let forwarding_float_sources = [None; FLOAT_REGISTER_SIZE];
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
            cycle_count,
            int_registers,
            float_registers,
            pc,
            int_registers_history,
            float_registers_history,
            pc_history,
            cycle_count_history: instruction_count_history,
            pc_stats,
            inst_stats,
            instruction_in_fetch_stage,
            instruction_in_fetch_stage_2,
            instruction_in_decode_stage: decoded_instruction,
            instruction_in_exec_stage,
            instruction_in_memory_stage,
            instruction_in_write_back_stage,
            forwarding_int_sources,
            forwarding_float_sources,
            inv_map,
            sqrt_map,
            sld_vec,
            sld_counter,
            output,
        }
    }

    pub fn get_decoded_instruction(&self) -> &Option<InstructionEnum> {
        &self.instruction_in_decode_stage
    }

    pub fn set_decoded_instruction(&mut self, inst: Option<InstructionEnum>) {
        self.instruction_in_decode_stage = inst;
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
        let inst = self.load_instruction(current_pc);
        if inst == 0 {
            return;
        }
        self.instruction_in_fetch_stage = Some(inst);
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
                // flush instruction in IF, IF2, and ID stage
                self.instruction_in_fetch_stage = None;
                self.instruction_in_fetch_stage_2 = None;
                self.instruction_in_decode_stage = None;
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

    #[inline]
    fn increment_cycle_count(&mut self) {
        self.cycle_count += 1;
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
        if index == ZERO {
            return; // zero register
        }
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

    pub fn read_int(&mut self) -> Word {
        let value = self.sld_vec[self.sld_counter].parse::<i32>().unwrap();
        self.sld_counter += 1;
        value
    }

    pub fn read_float(&mut self) -> Word {
        let value = self.sld_vec[self.sld_counter].parse::<f32>().unwrap();
        let fp = FloatingPoint::new_f32(value);
        self.sld_counter += 1;
        u32_to_i32(fp.get_32_bits())
    }

    pub fn load_word(&mut self, addr: Address) -> Word {
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

    pub fn print_char(&mut self, value: Word) {
        self.output.push(value as u8);
    }

    pub fn print_int(&mut self, value: Word) {
        self.output
            .append(&mut value.to_string().as_bytes().to_vec());
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
        self.instruction_in_exec_stage = self.instruction_in_decode_stage.clone();
        if let Some(fetched_instruction) = self.instruction_in_fetch_stage_2 {
            let decoded = decode_instruction(fetched_instruction);
            if let Instruction::Other = decoded {
                self.instruction_in_decode_stage = None;
                self.instruction_in_fetch_stage_2 = None;
                self.instruction_in_fetch_stage = None;
                return;
            }
            let decoded_inst_struct = create_instruction_struct(decoded);
            self.instruction_in_decode_stage = Some(decoded_inst_struct);
        } else {
            self.instruction_in_decode_stage = None;
        }
        self.instruction_in_fetch_stage_2 = self.instruction_in_fetch_stage;
        self.instruction_in_fetch_stage = None;
    }

    pub fn move_instructions_to_next_stage_stalling(&mut self) {
        self.instruction_in_write_back_stage = self.instruction_in_memory_stage.clone();
        self.instruction_in_memory_stage = self.instruction_in_exec_stage.clone();
        self.instruction_in_exec_stage = None;
    }

    pub fn get_forwarding_int_source(&self, rs: Rs) -> Option<&(InstructionCount, Int)> {
        self.forwarding_int_sources[rs as usize].as_ref()
    }

    pub fn set_forwarding_int_source(&mut self, rs: Rs, inst_cnt: InstructionCount, value: Int) {
        if rs == 0 {
            return;
        }
        self.forwarding_int_sources[rs as usize] = Some((inst_cnt, value));
    }

    pub fn get_forwarding_float_source(
        &self,
        rs: Rs,
    ) -> Option<&(InstructionCount, FloatingPoint)> {
        self.forwarding_float_sources[rs as usize].as_ref()
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
        self.forwarding_float_sources[rs as usize] = Some((inst_cnt, value));
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
                        let int_source = self.forwarding_int_sources[rd as usize];
                        if let Some((inst_cnt, _)) = int_source {
                            if inst_cnt == current_inst_cnt {
                                self.forwarding_int_sources[rd as usize] = None;
                            }
                        }
                    }
                    RegisterId::Float(rd) => {
                        if rd == 0 {
                            return;
                        }
                        let float_source = self.forwarding_float_sources[rd as usize];
                        if let Some((inst_cnt, _)) = float_source {
                            if inst_cnt == current_inst_cnt {
                                self.forwarding_float_sources[rd as usize] = None;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn check_load_hazard(&self) -> bool {
        if self.instruction_in_decode_stage.is_none() || self.instruction_in_exec_stage.is_none() {
            return false;
        }
        let decoded_instruction = self.instruction_in_decode_stage.as_ref().unwrap();
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

    pub fn get_cycle_count(&self) -> InstructionCount {
        self.cycle_count
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
        let if_string = if let Some(inst) = self.instruction_in_fetch_stage {
            format!("{:>08x}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&if_string, 20);
        let if2_string = if let Some(inst) = self.instruction_in_fetch_stage_2 {
            format!("{:>08x}", inst)
        } else {
            "None".to_string()
        };
        print_filled_with_space(&if2_string, 20);
        let id_string = if let Some(inst) = self.instruction_in_decode_stage.clone() {
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
    fn add_cycle_count_to_history(&mut self) {
        self.cycle_count_history.push(self.get_cycle_count());
    }

    #[allow(dead_code)]
    fn update_pc_stats(&mut self) {
        if let Some(inst) = self.instruction_in_fetch_stage {
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
        if let Some(inst) = &self.instruction_in_write_back_stage {
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
        for i in 0..self.cycle_count_history.len() {
            print!("{:>8}  ", self.cycle_count_history[i]);
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

    #[allow(dead_code)]
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
        eprint!("\x1B[2K\x1B[1000D");
        std::io::stdout().flush().unwrap();
        eprint!(
            "\rcycle: {} output: {:>08} pc: {:>06} sp: {:>010}",
            self.cycle_count,
            self.output.len(),
            self.get_pc() - 4,
            self.get_int_register(2),
        );
        if let Some(inst) = self.get_instruction_in_exec_stage() {
            let inst_string = format!("{:?}", inst);
            eprint!("  {}", inst_string);
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

    pub fn end(&mut self) {
        self.pc = INSTRUCTION_MEMORY_SIZE as Address;
    }

    pub fn run(&mut self, verbose: u32, interval: u64, ppm_file_path: &str, sld_file_path: &str) {
        let start_time = Instant::now();
        let mut will_stall = false;
        let mut stalling;

        let mut ppm_file = File::create(ppm_file_path).unwrap();
        let mut before_output_len = 0;

        self.load_sld_file(sld_file_path);

        self.update_pc_stats();

        if verbose == 2 {
            self.show_registers();
            self.add_registers_to_history();
            self.add_pc_to_history();
        }

        if verbose >= 1 {
            println!(
                "               IF                  IF2                 ID                  EX                  MEM                 WB"
            );
        }

        loop {
            if verbose >= 1 {
                let pc_string = format!("pc: {}", self.get_pc());
                print_filled_with_space(&pc_string, 15);
            }
            self.increment_cycle_count();
            if interval != 0 {
                thread::sleep(Duration::from_millis(interval));
            }
            if self.get_pc() >= INSTRUCTION_MEMORY_SIZE as Address {
                self.pc_history.pop();
                eprintln!();
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

            if self.instruction_in_write_back_stage.is_some() {
                self.increment_instruction_count();
            }

            write_back(self);
            memory_access(self);

            if !stalling {
                exec_instruction(self);
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

            if self.cycle_count % 1000000 == 0 {
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
                self.add_cycle_count_to_history();
            }
        }

        println!(
            "executed instruction count: {}\nelapsed time: {:?}\n{:.2} MIPS",
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
    core.increment_pc();
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
                let inst = core.instruction_in_decode_stage.clone().unwrap();
                let inst_string = format!("{}: {:?}", pc_count, inst);
                file.write_all(inst_string.as_bytes()).unwrap();
                file.write_all("\n".as_bytes()).unwrap();
                pc_count += 4;
            }
        }
    }
}
