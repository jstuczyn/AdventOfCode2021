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

use anyhow::bail;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Var {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Var {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            x if x == "w" => Ok(Var::W),
            x if x == "x" => Ok(Var::X),
            x if x == "y" => Ok(Var::Y),
            x if x == "z" => Ok(Var::Z),
            _ => bail!("not a valid variable"),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Var::W => write!(f, "w"),
            Var::X => write!(f, "x"),
            Var::Y => write!(f, "y"),
            Var::Z => write!(f, "z"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Operand {
    Variable(Var),
    Number(isize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Variable(var) => var.fmt(f),
            Operand::Number(num) => num.fmt(f),
        }
    }
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // try to parse it as a variable, otherwise fallback to a number
        if let Ok(var) = Var::from_str(s) {
            Ok(Operand::Variable(var))
        } else {
            Ok(Operand::Number(s.parse()?))
        }
    }
}
