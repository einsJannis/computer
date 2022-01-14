use crate::{Address, AssemblyProgram, Flag, Instruction, Register, Value};

enum MacroArgument {
    Instruction(ExpandableInstruction),
    Identifier(String),
    Register(Register),
    Value(Value),
    Address(Address),
    Flag(Flag),
}

struct MacroArgumentDefinition { name: String }

enum MacroMapping<T: Sized> {
    MacroArgument(String),
    Literal(T)
}

impl MacroMapping<MacroArgument> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> MacroArgument {
        match self {
            MacroMapping::MacroArgument(name) =>
                macro_arguments[macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)],
            MacroMapping::Literal(macro_mapping) => macro_mapping,
        }
    }
}

impl MacroMapping<ExpandableInstruction> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> ExpandableInstruction {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Instruction(instruction) => instruction.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(instruction) => instruction
        }
    }
}

impl MacroMapping<String> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> String {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_,it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Identifier(identifier) => identifier.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(identifier) => identifier
        }
    }
}

impl MacroMapping<Register> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> Register {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Register(register) => register.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(register) => register
        }
    }
}

impl MacroMapping<Value> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> Value {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Value(value) => value.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(value) => value
        }
    }
}

impl MacroMapping<Address> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> Address {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Address(address) => address.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(address) => address
        }
    }
}

impl MacroMapping<Flag> {
    pub fn expand(self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro) -> Flag {
        match self {
            MacroMapping::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match &macro_arguments[i] {
                    MacroArgument::Flag(flag) => flag.clone(),
                    _ => panic!()
                }
            }
            MacroMapping::Literal(flag) => flag
        }
    }
}

enum MacroInstruction {
    MacroArgument(String),
    MacroCall(MacroMapping<String>, Vec<MacroMapping<MacroArgument>>),
    Label(MacroMapping<String>),
    NOP,
    MOV(MacroMapping<Register>, MacroMapping<Value>),
    LDW(MacroMapping<Register>, MacroMapping<Address>),
    STW(MacroMapping<Register>, MacroMapping<Address>),
    LDA(MacroMapping<Address>),
    PSH(MacroMapping<Value>),
    POP(MacroMapping<Register>),
    JMP(MacroMapping<Flag>, MacroMapping<Address>),
    ADD(MacroMapping<Register>, MacroMapping<Value>),
    SUB(MacroMapping<Register>, MacroMapping<Value>),
    AND(MacroMapping<Register>, MacroMapping<Value>),
    OR(MacroMapping<Register>, MacroMapping<Value>),
    INV(MacroMapping<Register>),
    CMP(MacroMapping<Register>, MacroMapping<Value>),
    SHL(MacroMapping<Register>, MacroMapping<Value>),
    SHR(MacroMapping<Register>, MacroMapping<Value>),
}

impl MacroInstruction {
    fn expand(&self, macro_arguments: &Vec<MacroArgument>, macro_def: &Macro, program: &ExpandableProgram) -> Vec<Instruction> {
        match self {
            MacroInstruction::MacroArgument(name) => {
                let i = macro_def.argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name).0;
                match &macro_arguments[i] {
                    MacroArgument::Instruction(instruction) => {
                        instruction.clone().expand(program)
                    }
                    _ => panic!()
                }
            }
            MacroInstruction::MacroCall(name, arguments) => {
                let macro_def = program.macro_definitions.iter()
                    .find(|it| it.name == name.expand(macro_arguments, macro_def))?;
                macro_def.expand(
                    arguments.iter()
                        .map(|it|it.expand(macro_arguments, macro_def))
                        .collect(),
                    program
                )
            }
            MacroInstruction::Label(identifier) =>
                vec![Instruction::LABEL(identifier.expand(macro_arguments, macro_def))],
            MacroInstruction::NOP => vec![Instruction::NOP],
            MacroInstruction::MOV(register, value) =>
                vec![Instruction::MOV(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::LDW(register, address) =>
                vec![Instruction::LDW(
                    register.expand(macro_arguments, macro_def),
                    address.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::STW(register, address) =>
                vec![Instruction::STW(
                    register.expand(macro_arguments, macro_def),
                    address.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::LDA(address) =>
                vec![Instruction::LDA(address.expand(macro_arguments, macro_def))],
            MacroInstruction::PSH(value) =>
                vec![Instruction::PSH(value.expand(macro_arguments, macro_def))],
            MacroInstruction::POP(register) =>
                vec![Instruction::POP(register.expand(macro_arguments, macro_def))],
            MacroInstruction::JMP(flag, address) =>
                vec![Instruction::JMP(
                    flag.expand(macro_arguments, macro_def),
                    address.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::ADD(register, value) =>
                vec![Instruction::ADD(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::SUB(register, value) =>
                vec![Instruction::SUB(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::AND(register, value) =>
                vec![Instruction::AND(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::OR(register, value) =>
                vec![Instruction::OR(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::INV(register) =>
                vec![Instruction::INV(
                    register.expand(macro_arguments, macro_def),
                )],
            MacroInstruction::CMP(register, value) =>
                vec![Instruction::CMP(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::SHL(register, value) =>
                vec![Instruction::SHL(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )],
            MacroInstruction::SHR(register, value) =>
                vec![Instruction::SHR(
                    register.expand(macro_arguments, macro_def),
                    value.expand(macro_arguments, macro_def)
                )]
        }
    }
}

struct Macro {
    name: String,
    argument_definitions: Vec<MacroArgumentDefinition>,
    instructions: Vec<MacroInstruction>
}

impl Macro {
    pub fn expand(&self, arguments: Vec<MacroArgument>, program: &ExpandableProgram) -> Vec<Instruction> {
        self.instructions.iter()
            .map(|it| it.expand(&arguments, self, program))
            .collect().concat()
    }
}

#[derive(Clone)]
enum ExpandableInstruction {
    MacroCall(String, Vec<MacroArgument>),
    SingleInstruction(Instruction),
}

impl ExpandableInstruction {
    fn expand(self, program: &ExpandableProgram) -> Vec<Instruction> {
        match self {
            ExpandableInstruction::MacroCall(name, arguments) =>
                program.macro_from_name(name)?.expand(arguments, program),
            ExpandableInstruction::SingleInstruction(instruction) => vec![instruction]
        }
    }
}

struct ExpandableProgram {
    macro_definitions: Vec<Macro>,
    instructions: Vec<ExpandableInstruction>
}

impl ExpandableProgram {
    fn expand(self) -> AssemblyProgram {
        AssemblyProgram {
            instructions: self.instructions.iter()
                .map(|it| it.expand(&self)).collect().concat()
        }
    }
    fn macro_from_name(&self, name: String) -> Option<&Macro> {
        self.macro_definitions.iter().find(|it| it.name == name)
    }
}
