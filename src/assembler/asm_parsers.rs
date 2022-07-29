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

named!(operand <CompleteStr, Token>,
    alt!(
        integer_operand |
        register        |
        label_usage
    )
);

named!(directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive{ name: name.to_string()}
        )
    )
);

named!(directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            l: opt!(label_declaration) >>
            name: directive_declaration >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            o3: opt!(operand) >>
            (
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(name),
                    label: l,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            directive_combined
        ) >>
        (
            ins
        )
    )
);

// parse a declaration of user-defined label, such as `label1: LOAD $1 100`
named!(pub label_declaration<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (
                Token::LabelDeclaration {name: name.to_string()}
            )
        )
    )
);

// parse a usage of user-defined label, such as `JMP @label1`
named!(pub label_usage<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (
                Token::LabelUsage {name: name.to_string()}
            )
        )
    )
);

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode_parser >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction {
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt! (
            instruction_combined |
            directive
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
    fn test_parse_pure_instruction() {
        // one opcode instruction
        let result = instruction(CompleteStr("hlt\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::HLT }),
                label: None,
                directive: None,
                operand1: None,
                operand2: None,
                operand3: None
            }
        );

        // two registers instruction
        let result = instruction(CompleteStr("load $0 #12345\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                label: None,
                directive: None,
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 12345 }),
                operand3: None
            }
        );

        // three registers instruction
        let result = instruction(CompleteStr("add $0 $1 $2\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::ADD }),
                label: None,
                directive: None,
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::Register { reg_num: 1 }),
                operand3: Some(Token::Register { reg_num: 2 }),
            }
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

    // #[test]
    // fn test_program_to_bytes() {
    //     let result = program(CompleteStr("load $2 #12345\n"));
    //     assert_eq!(result.is_ok(), true);
    //     let (_, program) = result.unwrap();
    //     let bytecode = program.to_bytes();
    //     // println!("{:?}", bytecode);
    //     assert_eq!(bytecode.len(), 4);
    //     assert_eq!(bytecode[0], Opcode::LOAD.into());
    //     assert_eq!(bytecode[1], 2);
    //     let num = ((bytecode[2] as i32) << 8) + (bytecode[3] as i32);
    //     assert_eq!(num, 12345);
    // }

    #[test]
    fn test_parse_directive_instruction() {
        // three registers instruction
        let result = instruction(CompleteStr(".data\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: None,
                label: None,
                directive: Some(Token::Directive {
                    name: "data".to_string()
                }),
                operand1: None,
                operand2: None,
                operand3: None,
            }
        );
    }

    // #[test]
    // fn test_string_directive() {
    //     let result = directive_combined(CompleteStr("test: .asciiz 'Hello'"));
    //     assert_eq!(result.is_ok(), true);
    //     let (_, directive) = result.unwrap();

    //     // Yes, this is the what the result should be
    //     let correct_instruction = AssemblerInstruction {
    //         opcode: None,
    //         label: Some(Token::LabelDeclaration {
    //             name: "test".to_string(),
    //         }),
    //         directive: Some(Token::Directive {
    //             name: "asciiz".to_string(),
    //         }),
    //         operand1: Some(Token::IrString {
    //             name: "Hello".to_string(),
    //         }),
    //         operand2: None,
    //         operand3: None,
    //     };

    //     assert_eq!(directive, correct_instruction);
    // }

    #[test]
    fn test_parse_label_declaration_instruction() {
        // three registers instruction
        let result = instruction(CompleteStr("test: LOAD $1 #12345\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                label: Some(Token::LabelDeclaration {
                    name: "test".to_string()
                }),
                directive: None,
                operand1: Some(Token::Register { reg_num: 1 }),
                operand2: Some(Token::IntegerOperand { value: 12345 }),
                operand3: None,
            }
        );
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage(CompleteStr("@test"));
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );
        let result = label_usage(CompleteStr("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_label_usage_instruction() {
        // three registers instruction
        let result = instruction(CompleteStr("jmp @test1\n"));
        let (_, ins) = result.unwrap();
        assert_eq!(
            ins,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::JMP }),
                label: None,
                directive: None,
                operand1: Some(Token::LabelUsage {
                    name: "test1".to_string()
                }),
                operand2: None,
                operand3: None,
            }
        );
    }
}
