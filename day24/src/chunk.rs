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

use crate::alu::Instruction;

// It turns out the input is in the form of the following chunks repeat 14 times:
// inp w
// mul x 0
// add x z
// mod x 26
// div z z_div
// add x x_add
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y y_add
// mul y x
// add z y
// the only thing linking chunks together is the value of `z`. Both `x` and `y` are irrelevant (and `w` is always overwritten with input)

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub(crate) struct Chunk {
    pub(crate) z_div: isize,
    pub(crate) x_add: isize,
    pub(crate) y_add: isize,
}

impl Chunk {
    pub(crate) fn from_instructions(instructions: &[Instruction]) -> Self {
        assert_eq!(instructions.len(), 18, "invalid instructions provided");
        let z_div = if let Instruction::Div(_, op) = instructions[4] {
            op.get_number().expect("invalid instructions provided")
        } else {
            panic!("invalid instructions provided")
        };

        let x_add = if let Instruction::Add(_, op) = instructions[5] {
            op.get_number().expect("invalid instructions provided")
        } else {
            panic!("invalid instructions provided")
        };

        let y_add = if let Instruction::Add(_, op) = instructions[15] {
            op.get_number().expect("invalid instructions provided")
        } else {
            panic!("invalid instructions provided")
        };

        Chunk {
            z_div,
            x_add,
            y_add,
        }
    }

    pub(crate) fn execute(&self, w: isize, input_z: isize) -> isize {
        let x = input_z % 26;
        let z = input_z / self.z_div;

        if x + self.x_add != w {
            26 * z + w + self.y_add
        } else {
            z
        }
    }
}
