use crate::assembler::{AssemblerInstruction, Program, Token};
use crate::instruction::Opcode;

use nom::types::CompleteStr;
use nom::*;

named!(pub opcode_parser <CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >>
        (
            Token::Op{code: Opcode::from(opcode)}
        )
    )
);

named!(register <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (
                Token::Register{
                    reg_num: reg_num.parse::<u8>().unwrap()
                }
            )
        )
    )
);

named!(integer_operand <CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            value: digit >>
            (
                Token::IntegerOperand{
                    value: value.parse::<i32>().unwrap()
                }
            )
        )
    )
);

named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_parser >>
        opt!(multispace) >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1:None,
                operand2:None,
                operand3:None
            }
        )
    )
);

named!(pub instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_parser >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1:Some(r),
                operand2:Some(i),
                operand3:None
            }
        )
    )
);

named!(pub instruction_three<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_parser >>
        r1: register >>
        r2: register >>
        r3: register >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1:Some(r1),
                operand2:Some(r2),
                operand3:Some(r3),
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt! (
            instruction_three |
            instruction_two   |
            instruction_one
        ) >> 
        (
            ins
        )
    )
);

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction) >>
        (
            Program {
                instructions: instructions
            }
        )
    )
);

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_opcode_parser() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_parser(CompleteStr("load"));
        assert!(result.is_ok());

        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        // Tests that an invalid opcode recognized as IGL
        let result = opcode_parser(CompleteStr("xxxilg"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Register { reg_num: 0 });

        let result = register(CompleteStr("0"));
        assert!(result.is_err());
        let result = register(CompleteStr("$a"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_integer() {
        let result = integer_operand(CompleteStr("#566"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::IntegerOperand { value: 566 });

        let result = integer_operand(CompleteStr("10"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two(CompleteStr("load $0 #12345\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 12345 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_three(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction() {
        // one opcode instruction
        let result = instruction(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );

        // two registers instruction
        let result = instruction(CompleteStr("load $0 #12345\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 12345 }),
                    operand3: None
                }
            ))
        );

        // three registers instruction
        let result = instruction(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 }),
                }
            ))
        );
    }

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert_eq!(result.is_ok(), true);
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
        // println!("{:?}", p);
        // TODO: Figure out an ergonomic way to test the AssemblerInstruction returned
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $2 #12345\n"));
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        // println!("{:?}", bytecode);
        assert_eq!(bytecode.len(), 4);
        assert_eq!(bytecode[0], Opcode::LOAD.into());
        assert_eq!(bytecode[1], 2);
        let num = ((bytecode[2] as i32) << 8) + (bytecode[3] as i32);
        assert_eq!(num, 12345);
    }
}
