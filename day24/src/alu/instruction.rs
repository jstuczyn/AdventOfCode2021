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

use crate::alu::{Operand, Variable};
use anyhow::{anyhow, bail};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const INPUT: &str = "inp";
const ADD: &str = "add";
const MUL: &str = "mul";
const DIV: &str = "div";
const MOD: &str = "mod";
const EQUAL: &str = "eql";

#[derive(Debug, Copy, Clone)]
pub(crate) enum Instruction {
    Input(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Equal(Variable, Operand),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr_operands = s.split_ascii_whitespace();

        let ins = instr_operands
            .next()
            .ok_or_else(|| anyhow!("no instruction present"))?;

        let op1 = instr_operands
            .next()
            .ok_or_else(|| anyhow!("no operand 1 present"))?
            .parse()?;

        if ins == INPUT {
            return Ok(Instruction::Input(op1));
        }

        let op2 = instr_operands
            .next()
            .ok_or_else(|| anyhow!("no operand 2 present"))?
            .parse()?;

        match ins {
            ins if ins == ADD => Ok(Instruction::Add(op1, op2)),
            ins if ins == MUL => Ok(Instruction::Mul(op1, op2)),
            ins if ins == DIV => Ok(Instruction::Div(op1, op2)),
            ins if ins == MOD => Ok(Instruction::Mod(op1, op2)),
            ins if ins == EQUAL => Ok(Instruction::Equal(op1, op2)),
            x => bail!("{} is not a valid instruction", x),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(op1) => write!(f, "{} {}", INPUT, op1),
            Instruction::Add(op1, op2) => write!(f, "{} {} {}", ADD, op1, op2),
            Instruction::Mul(op1, op2) => write!(f, "{} {} {}", MUL, op1, op2),
            Instruction::Div(op1, op2) => write!(f, "{} {} {}", DIV, op1, op2),
            Instruction::Mod(op1, op2) => write!(f, "{} {} {}", MOD, op1, op2),
            Instruction::Equal(op1, op2) => write!(f, "{} {} {}", EQUAL, op1, op2),
        }
    }
}
