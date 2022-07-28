pub mod asm_parsers;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        match &self.opcode {
            Token::Op { code } => {
                result.push(code.into());
            }
            _ => {
                panic!("Non-opcode found in opcode field");
                // std::process::exit(1);
            }
        }

        for op in [&self.operand1, &self.operand2, &self.operand3] {
            match op {
                Some(op) => {
                    match op {
                        Token::Op { code: _ } => {
                            panic!("operan should not contain opcode");
                        }
                        Token::Register { reg_num } => {
                            result.push(*reg_num as u8);
                        }
                        Token::IntegerOperand { value } => {
                            let upper = ((0xFF00 & *value) >> 8) as u8;
                            let lower = (0xFF & *value) as u8;
                            result.push(upper);
                            result.push(lower);
                        }
                    };
                }
                None => {},
            };
        }
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

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}
