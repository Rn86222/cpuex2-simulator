// instruction cache
pub struct Core {
    memory: Memory,
    cache: Cache,
    memory_access_count: usize,
    cache_hit_count: usize,
    instruction_memory: InstructionMemory,
    instruction_cache: InstructionCache,
    instruction_memory_access_count: usize,
    instruction_count: InstructionCount,
    instruction_cache_hit_count: usize,
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
    forwarding_int_source_map: HashMap<Rs, (InstructionCount, Int)>,
    forwarding_float_source_map: HashMap<Rs, (InstructionCount, FloatingPoint)>,
    inv_map: InvMap,
    sqrt_map: SqrtMap,
    sld_vec: Vec<String>,
    sld_counter: usize,
    output: Vec<u8>,
}

impl Core {
    fn increment_instruction_cache_hit_count(&mut self) {
        self.instruction_cache_hit_count += 1;
    }

    fn process_instruction_cache_miss(&mut self, addr: Address) {
        let line_addr = addr & !((1 << self.instruction_cache.get_offset_bit_num()) - 1);
        let line = self.instruction_memory.get_cache_line(line_addr);
        let set_line_result = self.instruction_cache.set_line(line_addr, line);
        if set_line_result.is_some() {
            let evicted_line = set_line_result.unwrap();
            self.instruction_memory.set_cache_line(evicted_line);
        }
    }

    pub fn load_instruction(&mut self, addr: Address) -> InstructionValue {
        self.increment_instruction_memory_access_count();
        let cache_access = self.instruction_cache.get(addr);
        match cache_access {
            InstructionCacheAccess::HitGet(value) => {
                self.increment_instruction_cache_hit_count();
                return value;
            }
            InstructionCacheAccess::Miss => {
                let value = self.instruction_memory.load(addr);
                self.process_instruction_cache_miss(addr);
                return value;
            }
            _ => {
                panic!("invalid cache access");
            }
        }
    }

    pub fn store_instruction(&mut self, addr: Address, inst: InstructionValue) {
        self.increment_instruction_memory_access_count();
        let cache_access = self.instruction_cache.set(addr, inst);
        match cache_access {
            InstructionCacheAccess::HitSet => {
                self.increment_instruction_cache_hit_count();
            }
            InstructionCacheAccess::Miss => {
                self.instruction_memory.store(addr, inst);
                self.process_instruction_cache_miss(addr);
            }
            _ => {
                panic!("invalid cache access");
            }
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
        println!(
            "instruction cache hit count: {}",
            self.instruction_cache_hit_count
        );
        println!(
            "instruction cache hit rate: {:.5}%",
            self.instruction_cache_hit_count as f64 / self.instruction_memory_access_count as f64
                * 100.0
        );
    }
}
