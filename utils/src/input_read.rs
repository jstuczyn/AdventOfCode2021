// Copyright 2021 Jedrzej Stuczynski
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

use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_input_lines<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn read_input_lines_with_parser<T, F, P>(path: P, parser: F) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    F: Fn(String) -> io::Result<T>,
{
    read_input_lines(path)?
        .into_iter()
        .map(parser)
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("input could not be parsed into desired type - {:?}", err),
            )
        })
}

/// Reads the file as lines, parsing each of them into desired type.
pub fn read_parsed_line_input<T, P>(path: P) -> io::Result<Vec<T>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    read_input_lines(path)?
        .into_iter()
        .map(|line| line.parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("input could not be parsed into desired type - {:?}", err),
            )
        })
}
