use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pub pc: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn add_byte(&mut self, v: u8) {
        self.program.push(v);
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1] as u16);
        self.pc += 2;
        result
    }

    pub fn run(&mut self) {
        let mut no_err = true;
        while no_err {
            no_err = self.execute_instruction();
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                self.registers[r0] = self.registers[r1] + self.registers[r2];
            }
            Opcode::SUB => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                self.registers[r0] = self.registers[r1] - self.registers[r2];
            }
            Opcode::MUL => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                self.registers[r0] = self.registers[r1] * self.registers[r2];
            }
            Opcode::DIV => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                let r2 = self.next_8_bits() as usize;
                self.registers[r0] = self.registers[r1] / self.registers[r2];
                self.remainder = (self.registers[r1] % self.registers[r2]) as u32;
            }
            Opcode::JMP => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let register = self.next_8_bits() as usize;
                let offset = self.registers[register] as usize;
                self.pc += offset;
            }
            Opcode::JMPB => {
                let register = self.next_8_bits() as usize;
                let offset = self.registers[register] as usize;
                self.pc -= offset;
            }
            Opcode::EQ => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] == self.registers[r1];
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] != self.registers[r1];
                self.next_8_bits();
            }
            Opcode::GT => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] > self.registers[r1];
                self.next_8_bits();
            }
            Opcode::LT => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] < self.registers[r1];
                self.next_8_bits();
            }
            Opcode::GTE => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] >= self.registers[r1];
                self.next_8_bits();
            }
            Opcode::LTE => {
                let r0 = self.next_8_bits() as usize;
                let r1 = self.next_8_bits() as usize;
                self.equal_flag = self.registers[r0] <= self.registers[r1];
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JNEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        VM::new()
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![Opcode::HLT.into(), 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![Opcode::LOAD.into(), 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 255;
        test_vm.program = vec![Opcode::JMP.into(), 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 255);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![Opcode::JMPF.into(), 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![Opcode::JMPB.into(), 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.program = vec![Opcode::EQ.into(), 0, 1, 0, Opcode::EQ.into(), 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 3;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.program = vec![Opcode::NEQ.into(), 0, 1, 0, Opcode::NEQ.into(), 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 3;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_cmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.registers[1] = 2;
        test_vm.registers[2] = 2;
        test_vm.program = vec![
            Opcode::GT.into(),
            0,
            1,
            0,
            Opcode::LT.into(),
            0,
            1,
            0,
            Opcode::GTE.into(),
            2,
            1,
            0,
            Opcode::LTE.into(),
            2,
            1,
            0,
        ];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 4;
        test_vm.equal_flag = true;
        test_vm.program = vec![Opcode::JEQ.into(), 0, 0, 0, Opcode::JNEQ.into(), 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
        test_vm.equal_flag = false;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }
}
