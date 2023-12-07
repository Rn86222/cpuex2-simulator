#[derive(Clone)]
pub enum LoadInstructionEnum {
    Lb(Lb),
    Lh(Lh),
    Lw(Lw),
    Lbu(Lbu),
    Lhu(Lhu),
    Flw(Flw),
}

impl Debug for LoadInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadInstructionEnum::Lb(instruction) => write!(f, "{:?}", instruction),
            LoadInstructionEnum::Lh(instruction) => write!(f, "{:?}", instruction),
            LoadInstructionEnum::Lw(instruction) => write!(f, "{:?}", instruction),
            LoadInstructionEnum::Lbu(instruction) => write!(f, "{:?}", instruction),
            LoadInstructionEnum::Lhu(instruction) => write!(f, "{:?}", instruction),
            LoadInstructionEnum::Flw(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for LoadInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.register_fetch(core),
            LoadInstructionEnum::Lh(instruction) => instruction.register_fetch(core),
            LoadInstructionEnum::Lw(instruction) => instruction.register_fetch(core),
            LoadInstructionEnum::Lbu(instruction) => instruction.register_fetch(core),
            LoadInstructionEnum::Lhu(instruction) => instruction.register_fetch(core),
            LoadInstructionEnum::Flw(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.exec(core),
            LoadInstructionEnum::Lh(instruction) => instruction.exec(core),
            LoadInstructionEnum::Lw(instruction) => instruction.exec(core),
            LoadInstructionEnum::Lbu(instruction) => instruction.exec(core),
            LoadInstructionEnum::Lhu(instruction) => instruction.exec(core),
            LoadInstructionEnum::Flw(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.memory(core),
            LoadInstructionEnum::Lh(instruction) => instruction.memory(core),
            LoadInstructionEnum::Lw(instruction) => instruction.memory(core),
            LoadInstructionEnum::Lbu(instruction) => instruction.memory(core),
            LoadInstructionEnum::Lhu(instruction) => instruction.memory(core),
            LoadInstructionEnum::Flw(instruction) => instruction.memory(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.write_back(core),
            LoadInstructionEnum::Lh(instruction) => instruction.write_back(core),
            LoadInstructionEnum::Lw(instruction) => instruction.write_back(core),
            LoadInstructionEnum::Lbu(instruction) => instruction.write_back(core),
            LoadInstructionEnum::Lhu(instruction) => instruction.write_back(core),
            LoadInstructionEnum::Flw(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.get_source_registers(),
            LoadInstructionEnum::Lh(instruction) => instruction.get_source_registers(),
            LoadInstructionEnum::Lw(instruction) => instruction.get_source_registers(),
            LoadInstructionEnum::Lbu(instruction) => instruction.get_source_registers(),
            LoadInstructionEnum::Lhu(instruction) => instruction.get_source_registers(),
            LoadInstructionEnum::Flw(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.get_destination_register(),
            LoadInstructionEnum::Lh(instruction) => instruction.get_destination_register(),
            LoadInstructionEnum::Lw(instruction) => instruction.get_destination_register(),
            LoadInstructionEnum::Lbu(instruction) => instruction.get_destination_register(),
            LoadInstructionEnum::Lhu(instruction) => instruction.get_destination_register(),
            LoadInstructionEnum::Flw(instruction) => instruction.get_destination_register(),
        }
    }

    fn is_load_instruction(&self) -> bool {
        true
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.get_instruction_count(),
            LoadInstructionEnum::Lh(instruction) => instruction.get_instruction_count(),
            LoadInstructionEnum::Lw(instruction) => instruction.get_instruction_count(),
            LoadInstructionEnum::Lbu(instruction) => instruction.get_instruction_count(),
            LoadInstructionEnum::Lhu(instruction) => instruction.get_instruction_count(),
            LoadInstructionEnum::Flw(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            LoadInstructionEnum::Lb(instruction) => instruction.get_name(),
            LoadInstructionEnum::Lh(instruction) => instruction.get_name(),
            LoadInstructionEnum::Lw(instruction) => instruction.get_name(),
            LoadInstructionEnum::Lbu(instruction) => instruction.get_name(),
            LoadInstructionEnum::Lhu(instruction) => instruction.get_name(),
            LoadInstructionEnum::Flw(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum StoreInstructionEnum {
    Sb(Sb),
    Sh(Sh),
    Sw(Sw),
    Fsw(Fsw),
}

impl Debug for StoreInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreInstructionEnum::Sb(instruction) => write!(f, "{:?}", instruction),
            StoreInstructionEnum::Sh(instruction) => write!(f, "{:?}", instruction),
            StoreInstructionEnum::Sw(instruction) => write!(f, "{:?}", instruction),
            StoreInstructionEnum::Fsw(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for StoreInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.register_fetch(core),
            StoreInstructionEnum::Sh(instruction) => instruction.register_fetch(core),
            StoreInstructionEnum::Sw(instruction) => instruction.register_fetch(core),
            StoreInstructionEnum::Fsw(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.exec(core),
            StoreInstructionEnum::Sh(instruction) => instruction.exec(core),
            StoreInstructionEnum::Sw(instruction) => instruction.exec(core),
            StoreInstructionEnum::Fsw(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.memory(core),
            StoreInstructionEnum::Sh(instruction) => instruction.memory(core),
            StoreInstructionEnum::Sw(instruction) => instruction.memory(core),
            StoreInstructionEnum::Fsw(instruction) => instruction.memory(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.get_source_registers(),
            StoreInstructionEnum::Sh(instruction) => instruction.get_source_registers(),
            StoreInstructionEnum::Sw(instruction) => instruction.get_source_registers(),
            StoreInstructionEnum::Fsw(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.get_instruction_count(),
            StoreInstructionEnum::Sh(instruction) => instruction.get_instruction_count(),
            StoreInstructionEnum::Sw(instruction) => instruction.get_instruction_count(),
            StoreInstructionEnum::Fsw(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            StoreInstructionEnum::Sb(instruction) => instruction.get_name(),
            StoreInstructionEnum::Sh(instruction) => instruction.get_name(),
            StoreInstructionEnum::Sw(instruction) => instruction.get_name(),
            StoreInstructionEnum::Fsw(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum ImmInstructionEnum {
    Addi(Addi),
    Slli(Slli),
    Slti(Slti),
    Sltiu(Sltiu),
    Xori(Xori),
    Srli(Srli),
    Srai(Srai),
    Ori(Ori),
    Andi(Andi),
    Auipc(Auipc),
    Lui(Lui),
}

impl Debug for ImmInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmInstructionEnum::Addi(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Slli(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Slti(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Sltiu(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Xori(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Srli(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Srai(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Ori(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Andi(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Auipc(instruction) => write!(f, "{:?}", instruction),
            ImmInstructionEnum::Lui(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for ImmInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Slli(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Slti(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Sltiu(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Xori(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Srli(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Srai(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Ori(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Andi(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Auipc(instruction) => instruction.register_fetch(core),
            ImmInstructionEnum::Lui(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.exec(core),
            ImmInstructionEnum::Slli(instruction) => instruction.exec(core),
            ImmInstructionEnum::Slti(instruction) => instruction.exec(core),
            ImmInstructionEnum::Sltiu(instruction) => instruction.exec(core),
            ImmInstructionEnum::Xori(instruction) => instruction.exec(core),
            ImmInstructionEnum::Srli(instruction) => instruction.exec(core),
            ImmInstructionEnum::Srai(instruction) => instruction.exec(core),
            ImmInstructionEnum::Ori(instruction) => instruction.exec(core),
            ImmInstructionEnum::Andi(instruction) => instruction.exec(core),
            ImmInstructionEnum::Auipc(instruction) => instruction.exec(core),
            ImmInstructionEnum::Lui(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Slli(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Slti(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Sltiu(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Xori(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Srli(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Srai(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Ori(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Andi(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Auipc(instruction) => instruction.write_back(core),
            ImmInstructionEnum::Lui(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Slli(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Slti(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Sltiu(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Xori(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Srli(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Srai(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Ori(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Andi(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Auipc(instruction) => instruction.get_source_registers(),
            ImmInstructionEnum::Lui(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Slli(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Slti(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Sltiu(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Xori(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Srli(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Srai(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Ori(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Andi(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Auipc(instruction) => instruction.get_destination_register(),
            ImmInstructionEnum::Lui(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Slli(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Slti(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Sltiu(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Xori(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Srli(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Srai(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Ori(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Andi(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Auipc(instruction) => instruction.get_instruction_count(),
            ImmInstructionEnum::Lui(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            ImmInstructionEnum::Addi(instruction) => instruction.get_name(),
            ImmInstructionEnum::Slli(instruction) => instruction.get_name(),
            ImmInstructionEnum::Slti(instruction) => instruction.get_name(),
            ImmInstructionEnum::Sltiu(instruction) => instruction.get_name(),
            ImmInstructionEnum::Xori(instruction) => instruction.get_name(),
            ImmInstructionEnum::Srli(instruction) => instruction.get_name(),
            ImmInstructionEnum::Srai(instruction) => instruction.get_name(),
            ImmInstructionEnum::Ori(instruction) => instruction.get_name(),
            ImmInstructionEnum::Andi(instruction) => instruction.get_name(),
            ImmInstructionEnum::Auipc(instruction) => instruction.get_name(),
            ImmInstructionEnum::Lui(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum IntRInstructionEnum {
    Add(Add),
    Sub(Sub),
    Sll(Sll),
    Slt(Slt),
    Sltu(Sltu),
    Xor(Xor),
    Srl(Srl),
    Sra(Sra),
    Or(Or),
    And(And),
}

impl Debug for IntRInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntRInstructionEnum::Add(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Sub(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Sll(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Slt(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Sltu(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Xor(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Srl(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Sra(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::Or(instruction) => write!(f, "{:?}", instruction),
            IntRInstructionEnum::And(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for IntRInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Sub(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Sll(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Slt(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Sltu(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Xor(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Srl(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Sra(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::Or(instruction) => instruction.register_fetch(core),
            IntRInstructionEnum::And(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.exec(core),
            IntRInstructionEnum::Sub(instruction) => instruction.exec(core),
            IntRInstructionEnum::Sll(instruction) => instruction.exec(core),
            IntRInstructionEnum::Slt(instruction) => instruction.exec(core),
            IntRInstructionEnum::Sltu(instruction) => instruction.exec(core),
            IntRInstructionEnum::Xor(instruction) => instruction.exec(core),
            IntRInstructionEnum::Srl(instruction) => instruction.exec(core),
            IntRInstructionEnum::Sra(instruction) => instruction.exec(core),
            IntRInstructionEnum::Or(instruction) => instruction.exec(core),
            IntRInstructionEnum::And(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Sub(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Sll(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Slt(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Sltu(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Xor(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Srl(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Sra(instruction) => instruction.write_back(core),
            IntRInstructionEnum::Or(instruction) => instruction.write_back(core),
            IntRInstructionEnum::And(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Sub(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Sll(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Slt(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Sltu(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Xor(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Srl(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Sra(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::Or(instruction) => instruction.get_source_registers(),
            IntRInstructionEnum::And(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Sub(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Sll(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Slt(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Sltu(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Xor(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Srl(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Sra(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::Or(instruction) => instruction.get_destination_register(),
            IntRInstructionEnum::And(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Sub(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Sll(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Slt(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Sltu(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Xor(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Srl(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Sra(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::Or(instruction) => instruction.get_instruction_count(),
            IntRInstructionEnum::And(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            IntRInstructionEnum::Add(instruction) => instruction.get_name(),
            IntRInstructionEnum::Sub(instruction) => instruction.get_name(),
            IntRInstructionEnum::Sll(instruction) => instruction.get_name(),
            IntRInstructionEnum::Slt(instruction) => instruction.get_name(),
            IntRInstructionEnum::Sltu(instruction) => instruction.get_name(),
            IntRInstructionEnum::Xor(instruction) => instruction.get_name(),
            IntRInstructionEnum::Srl(instruction) => instruction.get_name(),
            IntRInstructionEnum::Sra(instruction) => instruction.get_name(),
            IntRInstructionEnum::Or(instruction) => instruction.get_name(),
            IntRInstructionEnum::And(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum BranchInstructionEnum {
    Beq(Beq),
    Bne(Bne),
    Blt(Blt),
    Bge(Bge),
    Bltu(Bltu),
    Bgeu(Bgeu),
}

impl Debug for BranchInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchInstructionEnum::Beq(instruction) => write!(f, "{:?}", instruction),
            BranchInstructionEnum::Bne(instruction) => write!(f, "{:?}", instruction),
            BranchInstructionEnum::Blt(instruction) => write!(f, "{:?}", instruction),
            BranchInstructionEnum::Bge(instruction) => write!(f, "{:?}", instruction),
            BranchInstructionEnum::Bltu(instruction) => write!(f, "{:?}", instruction),
            BranchInstructionEnum::Bgeu(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for BranchInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.register_fetch(core),
            BranchInstructionEnum::Bne(instruction) => instruction.register_fetch(core),
            BranchInstructionEnum::Blt(instruction) => instruction.register_fetch(core),
            BranchInstructionEnum::Bge(instruction) => instruction.register_fetch(core),
            BranchInstructionEnum::Bltu(instruction) => instruction.register_fetch(core),
            BranchInstructionEnum::Bgeu(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.exec(core),
            BranchInstructionEnum::Bne(instruction) => instruction.exec(core),
            BranchInstructionEnum::Blt(instruction) => instruction.exec(core),
            BranchInstructionEnum::Bge(instruction) => instruction.exec(core),
            BranchInstructionEnum::Bltu(instruction) => instruction.exec(core),
            BranchInstructionEnum::Bgeu(instruction) => instruction.exec(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.get_source_registers(),
            BranchInstructionEnum::Bne(instruction) => instruction.get_source_registers(),
            BranchInstructionEnum::Blt(instruction) => instruction.get_source_registers(),
            BranchInstructionEnum::Bge(instruction) => instruction.get_source_registers(),
            BranchInstructionEnum::Bltu(instruction) => instruction.get_source_registers(),
            BranchInstructionEnum::Bgeu(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.get_jump_address(),
            BranchInstructionEnum::Bne(instruction) => instruction.get_jump_address(),
            BranchInstructionEnum::Blt(instruction) => instruction.get_jump_address(),
            BranchInstructionEnum::Bge(instruction) => instruction.get_jump_address(),
            BranchInstructionEnum::Bltu(instruction) => instruction.get_jump_address(),
            BranchInstructionEnum::Bgeu(instruction) => instruction.get_jump_address(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.get_instruction_count(),
            BranchInstructionEnum::Bne(instruction) => instruction.get_instruction_count(),
            BranchInstructionEnum::Blt(instruction) => instruction.get_instruction_count(),
            BranchInstructionEnum::Bge(instruction) => instruction.get_instruction_count(),
            BranchInstructionEnum::Bltu(instruction) => instruction.get_instruction_count(),
            BranchInstructionEnum::Bgeu(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            BranchInstructionEnum::Beq(instruction) => instruction.get_name(),
            BranchInstructionEnum::Bne(instruction) => instruction.get_name(),
            BranchInstructionEnum::Blt(instruction) => instruction.get_name(),
            BranchInstructionEnum::Bge(instruction) => instruction.get_name(),
            BranchInstructionEnum::Bltu(instruction) => instruction.get_name(),
            BranchInstructionEnum::Bgeu(instruction) => instruction.get_name(),
        }
    }

    fn is_branch_instruction(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub enum JumpInstructionEnum {
    Jalr(Jalr),
    Jal(Jal),
}

impl Debug for JumpInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpInstructionEnum::Jalr(instruction) => write!(f, "{:?}", instruction),
            JumpInstructionEnum::Jal(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for JumpInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.register_fetch(core),
            JumpInstructionEnum::Jal(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.exec(core),
            JumpInstructionEnum::Jal(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.write_back(core),
            JumpInstructionEnum::Jal(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.get_source_registers(),
            JumpInstructionEnum::Jal(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.get_jump_address(),
            JumpInstructionEnum::Jal(instruction) => instruction.get_jump_address(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.get_destination_register(),
            JumpInstructionEnum::Jal(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.get_instruction_count(),
            JumpInstructionEnum::Jal(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            JumpInstructionEnum::Jalr(instruction) => instruction.get_name(),
            JumpInstructionEnum::Jal(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum MulDivInstructionEnum {
    Mul(Mul),
    Mulh(Mulh),
    Mulhsu(Mulhsu),
    Mulhu(Mulhu),
    Div(Div),
    Divu(Divu),
    Rem(Rem),
    Remu(Remu),
}

impl Debug for MulDivInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MulDivInstructionEnum::Mul(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Mulh(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Mulhsu(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Mulhu(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Div(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Divu(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Rem(instruction) => write!(f, "{:?}", instruction),
            MulDivInstructionEnum::Remu(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for MulDivInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Mulh(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Div(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Divu(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Rem(instruction) => instruction.register_fetch(core),
            MulDivInstructionEnum::Remu(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Mulh(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Div(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Divu(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Rem(instruction) => instruction.exec(core),
            MulDivInstructionEnum::Remu(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Mulh(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Div(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Divu(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Rem(instruction) => instruction.write_back(core),
            MulDivInstructionEnum::Remu(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Mulh(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Div(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Divu(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Rem(instruction) => instruction.get_source_registers(),
            MulDivInstructionEnum::Remu(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Mulh(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Div(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Divu(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Rem(instruction) => instruction.get_destination_register(),
            MulDivInstructionEnum::Remu(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Mulh(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Div(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Divu(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Rem(instruction) => instruction.get_instruction_count(),
            MulDivInstructionEnum::Remu(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            MulDivInstructionEnum::Mul(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Mulh(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Mulhsu(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Mulhu(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Div(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Divu(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Rem(instruction) => instruction.get_name(),
            MulDivInstructionEnum::Remu(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum FloatRInstructionEnum {
    Fadd(Fadd),
    Fsub(Fsub),
    Fmul(Fmul),
    Fdiv(Fdiv),
    Fsqrt(Fsqrt),
    Fsgnj(Fsgnj),
    Fsgnjn(Fsgnjn),
    Fsgnjx(Fsgnjx),
}

impl Debug for FloatRInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fsub(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fmul(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fdiv(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fsqrt(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fsgnj(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fsgnjn(instruction) => write!(f, "{:?}", instruction),
            FloatRInstructionEnum::Fsgnjx(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for FloatRInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fsub(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fmul(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.register_fetch(core),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fsub(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fmul(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.exec(core),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fsub(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fmul(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.write_back(core),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fsub(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fmul(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.get_source_registers(),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fsub(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fmul(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.get_destination_register(),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fsub(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fmul(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.get_instruction_count(),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            FloatRInstructionEnum::Fadd(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fsub(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fmul(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fdiv(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fsqrt(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fsgnj(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fsgnjn(instruction) => instruction.get_name(),
            FloatRInstructionEnum::Fsgnjx(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum FloatCmpInstructionEnum {
    Fmin(Fmin),
    Fmax(Fmax),
    Feq(Feq),
    Flt(Flt),
    Fle(Fle),
}

impl Debug for FloatCmpInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => write!(f, "{:?}", instruction),
            FloatCmpInstructionEnum::Fmax(instruction) => write!(f, "{:?}", instruction),
            FloatCmpInstructionEnum::Feq(instruction) => write!(f, "{:?}", instruction),
            FloatCmpInstructionEnum::Flt(instruction) => write!(f, "{:?}", instruction),
            FloatCmpInstructionEnum::Fle(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for FloatCmpInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.register_fetch(core),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.register_fetch(core),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.register_fetch(core),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.register_fetch(core),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.exec(core),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.exec(core),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.exec(core),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.exec(core),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.write_back(core),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.write_back(core),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.write_back(core),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.write_back(core),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.get_source_registers(),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.get_source_registers(),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.get_source_registers(),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.get_source_registers(),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.get_destination_register(),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.get_destination_register(),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.get_destination_register(),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.get_destination_register(),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.get_instruction_count(),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.get_instruction_count(),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.get_instruction_count(),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.get_instruction_count(),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            FloatCmpInstructionEnum::Fmin(instruction) => instruction.get_name(),
            FloatCmpInstructionEnum::Fmax(instruction) => instruction.get_name(),
            FloatCmpInstructionEnum::Feq(instruction) => instruction.get_name(),
            FloatCmpInstructionEnum::Flt(instruction) => instruction.get_name(),
            FloatCmpInstructionEnum::Fle(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum FloatOnlyRInstructionEnum {
    FcvtWS(FcvtWS),
    FcvtWuS(FcvtWuS),
    FcvtSW(FcvtSW),
    FcvtSWu(FcvtSWu),
    FmvXW(FmvXW),
    FmvWX(FmvWX),
}

impl Debug for FloatOnlyRInstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => write!(f, "{:?}", instruction),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => write!(f, "{:?}", instruction),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => write!(f, "{:?}", instruction),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => write!(f, "{:?}", instruction),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => write!(f, "{:?}", instruction),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for FloatOnlyRInstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.register_fetch(core),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.register_fetch(core),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.register_fetch(core),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.register_fetch(core),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.register_fetch(core),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.exec(core),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.exec(core),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.exec(core),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.exec(core),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.exec(core),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.exec(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.write_back(core),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.write_back(core),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.write_back(core),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.write_back(core),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.write_back(core),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.get_source_registers(),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.get_source_registers(),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.get_source_registers(),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.get_source_registers(),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.get_source_registers(),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => {
                instruction.get_destination_register()
            }
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => {
                instruction.get_destination_register()
            }
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => {
                instruction.get_destination_register()
            }
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => {
                instruction.get_destination_register()
            }
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.get_destination_register(),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.get_instruction_count(),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.get_instruction_count(),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.get_instruction_count(),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.get_instruction_count(),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.get_instruction_count(),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            FloatOnlyRInstructionEnum::FcvtWS(instruction) => instruction.get_name(),
            FloatOnlyRInstructionEnum::FcvtWuS(instruction) => instruction.get_name(),
            FloatOnlyRInstructionEnum::FcvtSW(instruction) => instruction.get_name(),
            FloatOnlyRInstructionEnum::FcvtSWu(instruction) => instruction.get_name(),
            FloatOnlyRInstructionEnum::FmvXW(instruction) => instruction.get_name(),
            FloatOnlyRInstructionEnum::FmvWX(instruction) => instruction.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum InstructionEnum {
    Load(LoadInstructionEnum),
    Store(StoreInstructionEnum),
    Imm(ImmInstructionEnum),
    IntR(IntRInstructionEnum),
    Branch(BranchInstructionEnum),
    Jump(JumpInstructionEnum),
    MulDiv(MulDivInstructionEnum),
    FloatR(FloatRInstructionEnum),
    FloatCmp(FloatCmpInstructionEnum),
    FloatOnlyR(FloatOnlyRInstructionEnum),
}

impl Debug for InstructionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionEnum::Load(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Store(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Imm(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::IntR(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Branch(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::Jump(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::MulDiv(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::FloatR(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::FloatCmp(instruction) => write!(f, "{:?}", instruction),
            InstructionEnum::FloatOnlyR(instruction) => write!(f, "{:?}", instruction),
        }
    }
}

impl InstructionTrait for InstructionEnum {
    fn register_fetch(&mut self, core: &Core) {
        match self {
            InstructionEnum::Load(instruction) => instruction.register_fetch(core),
            InstructionEnum::Store(instruction) => instruction.register_fetch(core),
            InstructionEnum::Imm(instruction) => instruction.register_fetch(core),
            InstructionEnum::IntR(instruction) => instruction.register_fetch(core),
            InstructionEnum::Branch(instruction) => instruction.register_fetch(core),
            InstructionEnum::Jump(instruction) => instruction.register_fetch(core),
            InstructionEnum::MulDiv(instruction) => instruction.register_fetch(core),
            InstructionEnum::FloatR(instruction) => instruction.register_fetch(core),
            InstructionEnum::FloatCmp(instruction) => instruction.register_fetch(core),
            InstructionEnum::FloatOnlyR(instruction) => instruction.register_fetch(core),
        }
    }

    fn exec(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Load(instruction) => instruction.exec(core),
            InstructionEnum::Store(instruction) => instruction.exec(core),
            InstructionEnum::Imm(instruction) => instruction.exec(core),
            InstructionEnum::IntR(instruction) => instruction.exec(core),
            InstructionEnum::Branch(instruction) => instruction.exec(core),
            InstructionEnum::Jump(instruction) => instruction.exec(core),
            InstructionEnum::MulDiv(instruction) => instruction.exec(core),
            InstructionEnum::FloatR(instruction) => instruction.exec(core),
            InstructionEnum::FloatCmp(instruction) => instruction.exec(core),
            InstructionEnum::FloatOnlyR(instruction) => instruction.exec(core),
        }
    }

    fn memory(&mut self, core: &mut Core) {
        match self {
            InstructionEnum::Load(instruction) => instruction.memory(core),
            InstructionEnum::Store(instruction) => instruction.memory(core),
            InstructionEnum::Imm(instruction) => instruction.memory(core),
            InstructionEnum::IntR(instruction) => instruction.memory(core),
            InstructionEnum::Branch(instruction) => instruction.memory(core),
            InstructionEnum::Jump(instruction) => instruction.memory(core),
            InstructionEnum::MulDiv(instruction) => instruction.memory(core),
            InstructionEnum::FloatR(instruction) => instruction.memory(core),
            InstructionEnum::FloatCmp(instruction) => instruction.memory(core),
            InstructionEnum::FloatOnlyR(instruction) => instruction.memory(core),
        }
    }

    fn write_back(&self, core: &mut Core) {
        match self {
            InstructionEnum::Load(instruction) => instruction.write_back(core),
            InstructionEnum::Store(instruction) => instruction.write_back(core),
            InstructionEnum::Imm(instruction) => instruction.write_back(core),
            InstructionEnum::IntR(instruction) => instruction.write_back(core),
            InstructionEnum::Branch(instruction) => instruction.write_back(core),
            InstructionEnum::Jump(instruction) => instruction.write_back(core),
            InstructionEnum::MulDiv(instruction) => instruction.write_back(core),
            InstructionEnum::FloatR(instruction) => instruction.write_back(core),
            InstructionEnum::FloatCmp(instruction) => instruction.write_back(core),
            InstructionEnum::FloatOnlyR(instruction) => instruction.write_back(core),
        }
    }

    fn get_source_registers(&self) -> Vec<Rs> {
        match self {
            InstructionEnum::Load(instruction) => instruction.get_source_registers(),
            InstructionEnum::Store(instruction) => instruction.get_source_registers(),
            InstructionEnum::Imm(instruction) => instruction.get_source_registers(),
            InstructionEnum::IntR(instruction) => instruction.get_source_registers(),
            InstructionEnum::Branch(instruction) => instruction.get_source_registers(),
            InstructionEnum::Jump(instruction) => instruction.get_source_registers(),
            InstructionEnum::MulDiv(instruction) => instruction.get_source_registers(),
            InstructionEnum::FloatR(instruction) => instruction.get_source_registers(),
            InstructionEnum::FloatCmp(instruction) => instruction.get_source_registers(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.get_source_registers(),
        }
    }

    fn get_destination_register(&self) -> Option<Rd> {
        match self {
            InstructionEnum::Load(instruction) => instruction.get_destination_register(),
            InstructionEnum::Store(instruction) => instruction.get_destination_register(),
            InstructionEnum::Imm(instruction) => instruction.get_destination_register(),
            InstructionEnum::IntR(instruction) => instruction.get_destination_register(),
            InstructionEnum::Branch(instruction) => instruction.get_destination_register(),
            InstructionEnum::Jump(instruction) => instruction.get_destination_register(),
            InstructionEnum::MulDiv(instruction) => instruction.get_destination_register(),
            InstructionEnum::FloatR(instruction) => instruction.get_destination_register(),
            InstructionEnum::FloatCmp(instruction) => instruction.get_destination_register(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.get_destination_register(),
        }
    }

    fn get_instruction_count(&self) -> Option<InstructionCount> {
        match self {
            InstructionEnum::Load(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Store(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Imm(instruction) => instruction.get_instruction_count(),
            InstructionEnum::IntR(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Branch(instruction) => instruction.get_instruction_count(),
            InstructionEnum::Jump(instruction) => instruction.get_instruction_count(),
            InstructionEnum::MulDiv(instruction) => instruction.get_instruction_count(),
            InstructionEnum::FloatR(instruction) => instruction.get_instruction_count(),
            InstructionEnum::FloatCmp(instruction) => instruction.get_instruction_count(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.get_instruction_count(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            InstructionEnum::Load(instruction) => instruction.get_name(),
            InstructionEnum::Store(instruction) => instruction.get_name(),
            InstructionEnum::Imm(instruction) => instruction.get_name(),
            InstructionEnum::IntR(instruction) => instruction.get_name(),
            InstructionEnum::Branch(instruction) => instruction.get_name(),
            InstructionEnum::Jump(instruction) => instruction.get_name(),
            InstructionEnum::MulDiv(instruction) => instruction.get_name(),
            InstructionEnum::FloatR(instruction) => instruction.get_name(),
            InstructionEnum::FloatCmp(instruction) => instruction.get_name(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.get_name(),
        }
    }

    fn get_jump_address(&self) -> Option<Address> {
        match self {
            InstructionEnum::Load(instruction) => instruction.get_jump_address(),
            InstructionEnum::Store(instruction) => instruction.get_jump_address(),
            InstructionEnum::Imm(instruction) => instruction.get_jump_address(),
            InstructionEnum::IntR(instruction) => instruction.get_jump_address(),
            InstructionEnum::Branch(instruction) => instruction.get_jump_address(),
            InstructionEnum::Jump(instruction) => instruction.get_jump_address(),
            InstructionEnum::MulDiv(instruction) => instruction.get_jump_address(),
            InstructionEnum::FloatR(instruction) => instruction.get_jump_address(),
            InstructionEnum::FloatCmp(instruction) => instruction.get_jump_address(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.get_jump_address(),
        }
    }

    fn is_branch_instruction(&self) -> bool {
        match self {
            InstructionEnum::Load(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Store(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Imm(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::IntR(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Branch(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::Jump(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::MulDiv(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::FloatR(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::FloatCmp(instruction) => instruction.is_branch_instruction(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.is_branch_instruction(),
        }
    }

    fn is_load_instruction(&self) -> bool {
        match self {
            InstructionEnum::Load(instruction) => instruction.is_load_instruction(),
            InstructionEnum::Store(instruction) => instruction.is_load_instruction(),
            InstructionEnum::Imm(instruction) => instruction.is_load_instruction(),
            InstructionEnum::IntR(instruction) => instruction.is_load_instruction(),
            InstructionEnum::Branch(instruction) => instruction.is_load_instruction(),
            InstructionEnum::Jump(instruction) => instruction.is_load_instruction(),
            InstructionEnum::MulDiv(instruction) => instruction.is_load_instruction(),
            InstructionEnum::FloatR(instruction) => instruction.is_load_instruction(),
            InstructionEnum::FloatCmp(instruction) => instruction.is_load_instruction(),
            InstructionEnum::FloatOnlyR(instruction) => instruction.is_load_instruction(),
        }
    }
}

fn create_i_instruction_struct(
    imm: Imm12,
    rs1: Rs1,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        3 => match funct3 {
            0b000 => InstructionEnum::Load(LoadInstructionEnum::Lb(Lb::new(imm, rs1, rd))),
            0b001 => InstructionEnum::Load(LoadInstructionEnum::Lh(Lh::new(imm, rs1, rd))),
            0b010 => InstructionEnum::Load(LoadInstructionEnum::Lw(Lw::new(imm, rs1, rd))),
            0b100 => InstructionEnum::Load(LoadInstructionEnum::Lbu(Lbu::new(imm, rs1, rd))),
            0b101 => InstructionEnum::Load(LoadInstructionEnum::Lhu(Lhu::new(imm, rs1, rd))),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        19 => match funct3 {
            0b000 => InstructionEnum::Imm(ImmInstructionEnum::Addi(Addi::new(imm, rs1, rd))),
            0b001 => InstructionEnum::Imm(ImmInstructionEnum::Slli(Slli::new(imm, rs1, rd))),
            0b010 => InstructionEnum::Imm(ImmInstructionEnum::Slti(Slti::new(imm, rs1, rd))),
            0b011 => InstructionEnum::Imm(ImmInstructionEnum::Sltiu(Sltiu::new(imm, rs1, rd))),
            0b100 => InstructionEnum::Imm(ImmInstructionEnum::Xori(Xori::new(imm, rs1, rd))),
            0b101 => {
                let funct7 = (imm >> 5) & 0b1111111;
                match funct7 {
                    0b0000000 => {
                        InstructionEnum::Imm(ImmInstructionEnum::Srli(Srli::new(imm, rs1, rd)))
                    }
                    0b0100000 => {
                        InstructionEnum::Imm(ImmInstructionEnum::Srai(Srai::new(imm, rs1, rd)))
                    }
                    _ => {
                        panic!("unexpected funct7: {}", funct7);
                    }
                }
            }
            0b110 => InstructionEnum::Imm(ImmInstructionEnum::Ori(Ori::new(imm, rs1, rd))),
            0b111 => InstructionEnum::Imm(ImmInstructionEnum::Andi(Andi::new(imm, rs1, rd))),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        103 => match funct3 {
            0b000 => InstructionEnum::Jump(JumpInstructionEnum::Jalr(Jalr::new(imm, rs1, rd))),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        7 => match funct3 {
            0b010 => InstructionEnum::Load(LoadInstructionEnum::Flw(Flw::new(imm, rs1, rd))),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_r_instruction_struct(
    funct7: Funct7,
    rs2: Rs2,
    rs1: Rs2,
    funct3: Funct3,
    rd: Rd,
    op: Op,
) -> InstructionEnum {
    match op {
        // TODO: swap instructions
        51 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Add(Add::new(rs2, rs1, rd)))
                }
                0b0100000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Sub(Sub::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Mul(Mul::new(rs2, rs1, rd)))
                }
                // 0b0110000 => {
                //     // absdiff
                // }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b001 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Sll(Sll::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Mulh(Mulh::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b010 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Slt(Slt::new(rs2, rs1, rd)))
                }
                0b0000001 => InstructionEnum::MulDiv(MulDivInstructionEnum::Mulhsu(Mulhsu::new(
                    rs2, rs1, rd,
                ))),
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b011 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Sltu(Sltu::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Mulhu(Mulhu::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b100 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Xor(Xor::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Div(Div::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b101 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Srl(Srl::new(rs2, rs1, rd)))
                }
                0b0100000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::Sra(Sra::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Divu(Divu::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b110 => match funct7 {
                0b0000000 => InstructionEnum::IntR(IntRInstructionEnum::Or(Or::new(rs2, rs1, rd))),
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Rem(Rem::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            0b111 => match funct7 {
                0b0000000 => {
                    InstructionEnum::IntR(IntRInstructionEnum::And(And::new(rs2, rs1, rd)))
                }
                0b0000001 => {
                    InstructionEnum::MulDiv(MulDivInstructionEnum::Remu(Remu::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct7: {}", funct7);
                }
            },
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        83 => match funct7 {
            0b0000000 => {
                InstructionEnum::FloatR(FloatRInstructionEnum::Fadd(Fadd::new(rs2, rs1, rd)))
            }
            0b0000100 => {
                InstructionEnum::FloatR(FloatRInstructionEnum::Fsub(Fsub::new(rs2, rs1, rd)))
            }
            0b0001000 => {
                InstructionEnum::FloatR(FloatRInstructionEnum::Fmul(Fmul::new(rs2, rs1, rd)))
            }
            0b0001100 => {
                InstructionEnum::FloatR(FloatRInstructionEnum::Fdiv(Fdiv::new(rs2, rs1, rd)))
            }
            0b0101100 => {
                InstructionEnum::FloatR(FloatRInstructionEnum::Fsqrt(Fsqrt::new(rs2, rs1, rd)))
            }
            0b0010000 => match funct3 {
                0b000 => {
                    InstructionEnum::FloatR(FloatRInstructionEnum::Fsgnj(Fsgnj::new(rs2, rs1, rd)))
                }
                0b001 => InstructionEnum::FloatR(FloatRInstructionEnum::Fsgnjn(Fsgnjn::new(
                    rs2, rs1, rd,
                ))),
                0b010 => InstructionEnum::FloatR(FloatRInstructionEnum::Fsgnjx(Fsgnjx::new(
                    rs2, rs1, rd,
                ))),
                _ => {
                    panic!("unexpected funct3: {}", funct3)
                }
            },
            0b0010100 => match funct3 {
                0b000 => InstructionEnum::FloatCmp(FloatCmpInstructionEnum::Fmin(Fmin::new(
                    rs2, rs1, rd,
                ))),
                0b001 => InstructionEnum::FloatCmp(FloatCmpInstructionEnum::Fmax(Fmax::new(
                    rs2, rs1, rd,
                ))),
                _ => {
                    panic!("unexpected funct3: {}", funct3)
                }
            },
            0b1010000 => match funct3 {
                0b010 => {
                    InstructionEnum::FloatCmp(FloatCmpInstructionEnum::Feq(Feq::new(rs2, rs1, rd)))
                }
                0b001 => {
                    InstructionEnum::FloatCmp(FloatCmpInstructionEnum::Flt(Flt::new(rs2, rs1, rd)))
                }
                0b000 => {
                    InstructionEnum::FloatCmp(FloatCmpInstructionEnum::Fle(Fle::new(rs2, rs1, rd)))
                }
                _ => {
                    panic!("unexpected funct3: {}", funct3)
                }
            },
            // 0b11100 => match funct3 {
            //     0b001 => {
            //         // fclass
            //     }
            //     _ => {
            //         panic!("unexpected funct3: {}", funct3)
            //     }
            // },
            0b1100000 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FcvtWS(
                FcvtWS::new(rs2, rs1, rd),
            )),
            0b1100001 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FcvtWuS(
                FcvtWuS::new(rs2, rs1, rd),
            )),
            0b1101000 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FcvtSW(
                FcvtSW::new(rs2, rs1, rd),
            )),
            0b1101001 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FcvtSWu(
                FcvtSWu::new(rs2, rs1, rd),
            )),
            0b1110000 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FmvXW(FmvXW::new(
                rs2, rs1, rd,
            ))),
            0b1111000 => InstructionEnum::FloatOnlyR(FloatOnlyRInstructionEnum::FmvWX(FmvWX::new(
                rs2, rs1, rd,
            ))),
            _ => {
                panic!("unexpected funct7: {}", funct7)
            }
        },
        // 52 => {
        // match funct3 {
        // 0b000 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swapw
        //         }
        //         _ => {
        //             panic!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // 0b001 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swaph
        //         }
        //         _ => {
        //             panic!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // 0b010 => {
        //     match funct7 {
        //         0b0000000 => {
        //             // swapb
        //         }
        //         _ => {
        //             panic!("unexpected funct7: {}", funct7)
        //         }
        //     }
        // }
        // _ => {
        //     panic!("unexpected funct3: {}", funct3)
        // }
        // }
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_s_instruction_struct(
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        35 => match funct3 {
            0b000 => InstructionEnum::Store(StoreInstructionEnum::Sb(Sb::new(imm, rs2, rs1))),
            0b001 => InstructionEnum::Store(StoreInstructionEnum::Sh(Sh::new(imm, rs2, rs1))),
            0b010 => InstructionEnum::Store(StoreInstructionEnum::Sw(Sw::new(imm, rs2, rs1))),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        39 => match funct3 {
            0b010 => InstructionEnum::Store(StoreInstructionEnum::Fsw(Fsw::new(imm, rs2, rs1))),
            _ => {
                panic!("unexpected funct3: {}", funct3)
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_b_instruction_struct(
    imm: Imm12,
    rs2: Rs2,
    rs1: Rs1,
    funct3: Funct3,
    op: Op,
) -> InstructionEnum {
    match op {
        99 => match funct3 {
            0b000 => InstructionEnum::Branch(BranchInstructionEnum::Beq(Beq::new(imm, rs2, rs1))),
            0b001 => InstructionEnum::Branch(BranchInstructionEnum::Bne(Bne::new(imm, rs2, rs1))),
            0b100 => InstructionEnum::Branch(BranchInstructionEnum::Blt(Blt::new(imm, rs2, rs1))),
            0b101 => InstructionEnum::Branch(BranchInstructionEnum::Bge(Bge::new(imm, rs2, rs1))),
            0b110 => InstructionEnum::Branch(BranchInstructionEnum::Bltu(Bltu::new(imm, rs2, rs1))),
            0b111 => InstructionEnum::Branch(BranchInstructionEnum::Bgeu(Bgeu::new(imm, rs2, rs1))),
            _ => {
                panic!("unexpected funct3: {}", funct3);
            }
        },
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_j_instruction_struct(imm: Imm20, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        111 => InstructionEnum::Jump(JumpInstructionEnum::Jal(Jal::new(imm, rd))),
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_u_instruction_struct(imm: Imm20, rd: Rd, op: Op) -> InstructionEnum {
    match op {
        23 => InstructionEnum::Imm(ImmInstructionEnum::Auipc(Auipc::new(imm, rd))),
        55 => InstructionEnum::Imm(ImmInstructionEnum::Lui(Lui::new(imm, rd))),
        _ => {
            panic!("unexpected op: {}", op);
        }
    }
}

fn create_r4_instruction_struct(
    _fs3: Fs3,
    _funct2: Funct2,
    _fs2: Fs2,
    _fs1: Fs1,
    _funct3: Funct3,
    _rd: Rd,
    op: Op,
) -> InstructionEnum {
    // match op {
    //     // 67 => {
    //     //     // fmadd
    //     // }
    //     // 71 => {
    //     //     // fmsub
    //     // }
    //     // 75 => {
    //     //     // fnmsub
    //     // }
    //     // 79 => {
    //     //     // fnmadd
    //     // }
    //     _ => {
    //         panic!("unexpected op: {}", op);
    //     }
    // }
    {
        panic!("unexpected op: {}", op);
    }
}
