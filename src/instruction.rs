use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTE, // greater than OR equal to
    LTE, // less than OR equal to
    JEQD,
    JEQ,
    IGL,
    NOP,
    ALOC,
    INC,
    DEC,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JEQD,
            16 => Opcode::JEQ,
            17 => Opcode::NOP,
            18 => Opcode::ALOC,
            19 => Opcode::INC,
            20 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

impl Into<u8> for &Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::HLT => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTE => 13,
            Opcode::LTE => 14,
            Opcode::JEQD => 15,
            Opcode::JEQ => 16,
            Opcode::NOP => 17,
            Opcode::ALOC => 18,
            Opcode::INC => 19,
            Opcode::DEC => 20,
            _ => 255,
        }
    }
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        (&self).into()
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match v {
            CompleteStr("eq") | CompleteStr("EQ") => Opcode::EQ,
            CompleteStr("gt") | CompleteStr("GT") => Opcode::GT,
            CompleteStr("lt") | CompleteStr("LT") => Opcode::LT,
            CompleteStr("add") | CompleteStr("ADD") => Opcode::ADD,
            CompleteStr("sub") | CompleteStr("SUB") => Opcode::SUB,
            CompleteStr("mul") | CompleteStr("MUL") => Opcode::MUL,
            CompleteStr("div") | CompleteStr("DIV") => Opcode::DIV,
            CompleteStr("hlt") | CompleteStr("HLT") => Opcode::HLT,
            CompleteStr("jmp") | CompleteStr("JMP") => Opcode::JMP,
            CompleteStr("neq") | CompleteStr("NEQ") => Opcode::NEQ,
            CompleteStr("gte") | CompleteStr("GTE") => Opcode::GTE,
            CompleteStr("lte") | CompleteStr("LTE") => Opcode::LTE,
            CompleteStr("jeq") | CompleteStr("JEQ") => Opcode::JEQ,
            CompleteStr("nop") | CompleteStr("NOP") => Opcode::NOP,
            CompleteStr("inc") | CompleteStr("INC") => Opcode::INC,
            CompleteStr("dec") | CompleteStr("DEC") => Opcode::DEC,
            CompleteStr("load") | CompleteStr("LOAD") => Opcode::LOAD,
            CompleteStr("aloc") | CompleteStr("ALOC") => Opcode::ALOC,
            CompleteStr("jmpf") | CompleteStr("JMPF") => Opcode::JMPF,
            CompleteStr("jmpb") | CompleteStr("JMPB") => Opcode::JMPB,
            CompleteStr("jeqd") | CompleteStr("JEQD") => Opcode::JEQD,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

// pub create_instruction(op: Opcode, r0, r1, r2) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
