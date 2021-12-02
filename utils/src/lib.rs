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

use std::fmt::Display;

pub mod execution;
pub mod input_read;

pub use execution::execute_slice_with_timing;

// We'll see how it evolves with variety of inputs we get
pub fn execute<T, F, G, U, S>(input: &[T], part1_fn: F, part2_fn: G)
where
    F: Fn(&[T]) -> U,
    G: Fn(&[T]) -> S,
    U: Display,
    S: Display,
{
    let (part1_result, part1_time_taken) = execute_slice_with_timing(part1_fn, input);
    let (part2_result, part2_time_taken) = execute_slice_with_timing(part2_fn, input);

    println!(
        "Part 1 result is {}\nIt took {:?} to compute",
        part1_result, part1_time_taken
    );

    println!(
        "Part 2 result is {}\nIt took {:?} to compute",
        part2_result, part2_time_taken
    );
}
