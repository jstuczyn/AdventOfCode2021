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
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn execute_slice_with_timing<F, T, U>(func: F, args: &[T]) -> (U, Duration)
where
    F: Fn(&[T]) -> U,
{
    let start = Instant::now();
    let res = func(args);
    let time_taken = Instant::now() - start;
    (res, time_taken)
}

pub fn execute_struct_with_timing<F, T, U>(func: F, args: T) -> (U, Duration)
where
    F: Fn(T) -> U,
{
    let start = Instant::now();
    let res = func(args);
    let time_taken = Instant::now() - start;
    (res, time_taken)
}

// We'll see how it evolves with variety of inputs we get
pub fn execute_slice<P, T, F, G, H, U, S>(input_file: P, input_parser: F, part1_fn: G, part2_fn: H)
where
    P: AsRef<Path>,
    F: Fn(P) -> io::Result<Vec<T>>,
    G: Fn(&[T]) -> U,
    H: Fn(&[T]) -> S,
    U: Display,
    S: Display,
{
    let parsing_start = Instant::now();
    let input = input_parser(input_file).expect("failed to read input file");
    let parsing_time_taken = parsing_start.elapsed();

    let (part1_result, part1_time_taken) = execute_slice_with_timing(part1_fn, &input);
    let (part2_result, part2_time_taken) = execute_slice_with_timing(part2_fn, &input);

    println!("It took {:?} to parse the input", parsing_time_taken);
    println!();
    println!(
        "Part 1 result is {}\nIt took {:?} to compute",
        part1_result, part1_time_taken
    );
    println!();
    println!(
        "Part 2 result is {}\nIt took {:?} to compute",
        part2_result, part2_time_taken
    );
}

pub fn execute_struct<P, T, F, G, H, U, S>(input_file: P, input_parser: F, part1_fn: G, part2_fn: H)
where
    P: AsRef<Path>,
    F: Fn(P) -> io::Result<T>,
    G: Fn(T) -> U,
    H: Fn(T) -> S,
    U: Display,
    S: Display,
    T: Clone,
{
    let parsing_start = Instant::now();
    let input = input_parser(input_file).expect("failed to read input file");
    let parsing_time_taken = parsing_start.elapsed();

    let (part1_result, part1_time_taken) = execute_struct_with_timing(part1_fn, input.clone());
    let (part2_result, part2_time_taken) = execute_struct_with_timing(part2_fn, input);

    println!("It took {:?} to parse the input", parsing_time_taken);
    println!();
    println!(
        "Part 1 result is {}\nIt took {:?} to compute",
        part1_result, part1_time_taken
    );
    println!();
    println!(
        "Part 2 result is {}\nIt took {:?} to compute",
        part2_result, part2_time_taken
    );
}
