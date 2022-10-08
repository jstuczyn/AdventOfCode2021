// Copyright 2022 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod instruction;

mod operand;

pub(crate) use instruction::Instruction;
pub(crate) use operand::{Operand, Variable};
use std::collections::VecDeque;

#[derive(Default, Debug)]
pub(crate) struct Alu {
    input: VecDeque<isize>,
    pub(crate) w: isize,
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
}

impl Alu {
    fn read_variable(&self, variable: Variable) -> isize {
        match variable {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn store_variable(&mut self, variable: Variable, value: isize) {
        match variable {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
        }
    }

    fn resolve_operand(&self, operand: Operand) -> isize {
        match operand {
            Operand::Var(var) => self.read_variable(var),
            Operand::Number(val) => val,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Input(var) => {
                let input = self
                    .input
                    .pop_front()
                    .expect("the input has been exhausted!");
                self.store_variable(var, input);
            }
            Instruction::Add(var, op) => {
                self.store_variable(var, self.read_variable(var) + self.resolve_operand(op))
            }
            Instruction::Mul(var, op) => {
                self.store_variable(var, self.read_variable(var) * self.resolve_operand(op))
            }
            Instruction::Div(var, op) => {
                self.store_variable(var, self.read_variable(var) / self.resolve_operand(op))
            }
            Instruction::Mod(var, op) => {
                self.store_variable(var, self.read_variable(var) % self.resolve_operand(op))
            }
            Instruction::Equal(var, op) => {
                println!("{:?}", self);
                println!("{} == {} ?", var, op);
                self.store_variable(
                    var,
                    isize::from(self.read_variable(var) == self.resolve_operand(op)),
                )
            }
        }
    }

    pub(crate) fn execute_program(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(*instruction)
        }
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_input(mut self, input: VecDeque<isize>) -> Self {
        self.input = input;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alu::Alu;

    #[test]
    fn example1() {
        let instructions = vec!["inp x".parse().unwrap(), "mul x -1".parse().unwrap()];

        let mut alu = Alu::new().with_input(vec![42].into());
        alu.execute_program(&instructions);

        assert_eq!(alu.x, -42);

        let mut alu = Alu::new().with_input(vec![-42].into());
        alu.execute_program(&instructions);

        assert_eq!(alu.x, 42);
    }

    #[test]
    fn example2() {
        let instructions = vec![
            "inp z".parse().unwrap(),
            "inp x".parse().unwrap(),
            "mul z 3".parse().unwrap(),
            "eql z x".parse().unwrap(),
        ];

        let mut alu = Alu::new().with_input(vec![1, 1].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);

        let mut alu = Alu::new().with_input(vec![1, 2].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);

        let mut alu = Alu::new().with_input(vec![1, 3].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 1);
    }

    #[test]
    fn example3() {
        let instructions = vec![
            "inp w".parse().unwrap(),
            "add z w".parse().unwrap(),
            "mod z 2".parse().unwrap(),
            "div w 2".parse().unwrap(),
            "add y w".parse().unwrap(),
            "mod y 2".parse().unwrap(),
            "div w 2".parse().unwrap(),
            "add x w".parse().unwrap(),
            "mod x 2".parse().unwrap(),
            "div w 2".parse().unwrap(),
            "mod w 2".parse().unwrap(),
        ];

        let mut alu = Alu::new().with_input(vec![0].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 0);

        let mut alu = Alu::new().with_input(vec![1].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 0);

        let mut alu = Alu::new().with_input(vec![2].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 0);

        let mut alu = Alu::new().with_input(vec![14].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.w, 1);

        let mut alu = Alu::new().with_input(vec![15].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.x, 1);
        assert_eq!(alu.w, 1);

        let mut alu = Alu::new().with_input(vec![16].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 0);

        let mut alu = Alu::new().with_input(vec![17].into());
        alu.execute_program(&instructions);
        assert_eq!(alu.z, 1);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.x, 0);
        assert_eq!(alu.w, 0);
    }
}
