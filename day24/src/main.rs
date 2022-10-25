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
use crate::chunk::Chunk;
use std::collections::HashSet;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;

mod alu;
mod chunk;

const DIGITS_ASC: &[isize] = &[1isize, 2, 3, 4, 5, 6, 7, 8, 9];
const DIGITS_DESC: &[isize] = &[9isize, 8, 7, 6, 5, 4, 3, 2, 1];

#[derive(Copy, Clone)]
enum SolutionType {
    Largest,
    Smallest,
}

// simple bruteforce with pruning
fn check_chunks(
    dead_ends: &mut HashSet<(isize, usize)>,
    input_z: isize,
    chunks: &[Chunk],
    prefix: usize,
    solution_type: SolutionType,
) -> (usize, bool) {
    // have we already seen this input z at this depth?
    if dead_ends.contains(&(input_z, chunks.len())) {
        // not worth following
        return (prefix, false);
    }

    // we have reached the final chunk
    if chunks.is_empty() {
        return (prefix, input_z == 0);
    }

    let ws = match solution_type {
        SolutionType::Smallest => DIGITS_ASC,
        SolutionType::Largest => DIGITS_DESC,
    };

    for &w in ws {
        let output_z = chunks[0].execute(w, input_z);

        let (val, found_valid_solution) = check_chunks(
            dead_ends,
            output_z,
            &chunks[1..],
            10 * prefix + w as usize,
            solution_type,
        );
        // we're done, propagate the answer to the top
        if found_valid_solution {
            return (val, true);
        }
    }

    // nothing useful in this branch
    dead_ends.insert((input_z, chunks.len()));
    (prefix, false)
}

fn bruteforce(chunks: &[Chunk], solution_type: SolutionType) -> usize {
    let mut dead_ends = HashSet::new();
    let (solution, is_solution_valid) = check_chunks(&mut dead_ends, 0, chunks, 0, solution_type);
    assert!(is_solution_valid);
    solution
}

fn part1(instructions: &[Instruction]) -> usize {
    let chunks = instructions
        .chunks_exact(18)
        .map(Chunk::from_instructions)
        .collect::<Vec<_>>();

    bruteforce(&chunks, SolutionType::Largest)
}

fn part2(instructions: &[Instruction]) -> usize {
    let chunks = instructions
        .chunks_exact(18)
        .map(Chunk::from_instructions)
        .collect::<Vec<_>>();

    bruteforce(&chunks, SolutionType::Smallest)
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}
