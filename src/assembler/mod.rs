pub mod asm_parsers;

use nom::types::CompleteStr;

use crate::assembler::asm_parsers::program;
use crate::instruction::Opcode;

// PIE Magic numbers
pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Option<Token>,
    label: Option<Token>,
    directive: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl std::fmt::Display for AssemblerInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(Label: {:?} Opcode: {:?} Directive: {:?} Operand #1: {:?} Operand #2: {:?} Operand #3: {:?})",
            self.label, self.opcode, self.directive, self.operand1, self.operand2, self.operand3
        )
    }
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbol_tbl: &SymbolTable) -> Vec<u8> {
        let mut result = vec![];
        if let Some(token) = &self.opcode {
            match token {
                Token::Op { code } => {
                    result.push(code.into());
                }
                _ => {
                    println!(
                        "Non-opcode found in opcode field, AssemblerInstruction: `{:?}`",
                        self
                    );
                }
            }
        }

        for operand in [&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(Token::Register { reg_num }) => {
                    result.push(*reg_num as u8);
                }
                Some(Token::IntegerOperand { value }) => {
                    let upper = ((0xFF00 & *value) >> 8) as u8;
                    let lower = (0xFF & *value) as u8;
                    result.push(upper);
                    result.push(lower);
                }
                Some(Token::LabelUsage { name }) => {
                    let offset = symbol_tbl.symbol_value(name);
                    let offset = offset
                        .unwrap_or_else(|| panic!("LabelUsage token has no offset: `{:?}`", self));
                    let upper = ((0xFF00 & offset) >> 8) as u8;
                    let lower = (0xFF & offset) as u8;
                    result.push(upper);
                    result.push(lower);
                }
                Some(Token::Op { code: _ }) => {
                    panic!(
                        "operand should not contain opcode, AssemblerInstruction: `{:?}`",
                        self
                    );
                }
                Some(Token::Directive { name: _ }) => {
                    panic!(
                        "operand should not contain directive, AssemblerInstruction: `{:?}`",
                        self
                    );
                }
                Some(Token::LabelDeclaration { name: _ }) => {
                    panic!("operand should not contain label declaration, AssemblerInstruction: `{:?}`", self);
                }

                None => {}
            };
        }

        assert!(result.len() <= 4);

        while result.len() < 4 {
            result.push(0);
        }
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub program: Option<Program>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            program: None,
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_rem, program)) => {
                let mut assembled_program = self.write_pie_header();
                self.process_first_phase(&program);
                let mut body = self.process_second_phase(&program);

                self.program = Some(program);
                assembled_program.append(&mut body);
                Some(assembled_program)
            }
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }

    pub fn get_assembled_program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut pos = 0;
        for ins in p.instructions.iter() {
            match &ins.label {
                Some(Token::LabelDeclaration { name }) => {
                    let symbel = Symbol::new(name.clone(), pos, SymbolType::Label);
                    self.symbols.add_symbol(symbel);
                }
                _ => {}
            }
            pos += 4;
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        program
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in PIE_HEADER_PREFIX {
            header.push(byte);
        }

        while header.len() < PIE_HEADER_LENGTH {
            header.push(0);
        }
        header
    }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, offset: u32, symbol_type: SymbolType) -> Self {
        Symbol {
            name,
            offset,
            symbol_type,
        }
    }

    pub fn get_type(&self) -> SymbolType {
        self.symbol_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::vm::VM;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), 12, SymbolType::Label);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(v, Some(12));
        let v = sym.symbol_value("does_not_exist");
        assert!(v.is_none());
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = "load $0 #100\ntest: inc $2\nneq $0 $2\njeqd @test\nhlt";
        let program = asm.assemble(test_string).unwrap();
        let program = program[64..].to_vec();   // trim PIE header
        assert_eq!(program.len(), 20);

        let prog = asm.get_assembled_program().unwrap();
        let instructions = &prog.instructions;

        assert_eq!(
            instructions[0],
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                label: None,
                directive: None,
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 100 }),
                operand3: None
            }
        );

        assert_eq!(
            instructions[1],
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::INC }),
                label: Some(Token::LabelDeclaration {
                    name: "test".to_string()
                }),
                directive: None,
                operand1: Some(Token::Register { reg_num: 2 }),
                operand2: None,
                operand3: None
            }
        );

        assert_eq!(
            instructions[2],
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::NEQ }),
                label: None,
                directive: None,
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::Register { reg_num: 2 }),
                operand3: None
            }
        );

        assert_eq!(
            instructions[3],
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::JEQD }),
                label: None,
                directive: None,
                operand1: Some(Token::LabelUsage {
                    name: "test".to_string()
                }),
                operand2: None,
                operand3: None
            }
        );

        assert_eq!(
            instructions[4],
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::HLT }),
                label: None,
                directive: None,
                operand1: None,
                operand2: None,
                operand3: None
            }
        );

        // run vm
        let mut vm = VM::new();
        vm.add_bytes(program);
        vm.run();
        assert_eq!(vm.pc, 17);
        assert_eq!(vm.registers[0], vm.registers[2]);
    }
}
