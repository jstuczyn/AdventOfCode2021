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

use bitvec::prelude::*;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug)]
struct MalformedPacket;

const LITERAL_VAL_ID: u8 = 4;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Literal,
    Operator(u8),
}

impl From<u8> for Type {
    fn from(val: u8) -> Self {
        match val {
            n if n == LITERAL_VAL_ID => Type::Literal,
            n => Type::Operator(n),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Content {
    Literal(u32),
    Packets(Vec<Packet>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    header: Header,
    content: Content,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Header {
    version: u8,
    type_id: Type,
}

impl Header {
    fn from_bits(bits: &BitSlice<u8, Msb0>) -> Self {
        let version = bits[..3].load::<u8>();
        let type_id = Type::from(bits[3..6].load::<u8>());
        Header { version, type_id }
    }

    fn is_literal(&self) -> bool {
        self.type_id == Type::Literal
    }
}

fn parse_literal(bits: &BitSlice<u8, Msb0>) -> (usize, u32) {
    let mut i = 0;
    let mut literal_bits: BitVec<u8, Msb0> = BitVec::new();

    loop {
        literal_bits.push(bits[i + 1]);
        literal_bits.push(bits[i + 2]);
        literal_bits.push(bits[i + 3]);
        literal_bits.push(bits[i + 4]);

        if !bits[i] {
            break;
        }
        i += 5;
    }

    let mut recovered_literal = 0u32;
    recovered_literal.view_bits_mut::<Msb0>()[u32::BITS as usize - literal_bits.len()..]
        .clone_from_bitslice(&literal_bits);

    (i, recovered_literal)
}

impl FromStr for Packet {
    type Err = MalformedPacket;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = hex::decode(s).map_err(|_| MalformedPacket)?;
        let bits = BitVec::<u8, Msb0>::from_slice(&decoded);
        let bit_slice = bits.as_bitslice();

        let header = Header::from_bits(&bit_slice[..6]);

        if header.is_literal() {
            let (_used, literal) = parse_literal(&bit_slice[6..]);
            Ok(Packet {
                header,
                content: Content::Literal(literal),
            })
        } else {
            unimplemented!()
        }
    }
}

fn part1(packet: Packet) -> usize {
    0
}

fn part2(packet: Packet) -> usize {
    0
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_struct("input", read_parsed, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_packet_parsing() {
        let packet = "D2FE28".parse().unwrap();
        let expected = Packet {
            header: Header {
                version: 6,
                type_id: Type::Literal,
            },
            content: Content::Literal(2021),
        };

        assert_eq!(expected, packet);
    }

    // #[test]
    // fn part1_sample_input_1() {
    //     let packet = "8A004A801A8002F478".parse().unwrap();
    //     let expected = 16;
    //
    //     assert_eq!(expected, part1(packet));
    // }
    //
    // #[test]
    // fn part1_sample_input_2() {
    //     let packet = "620080001611562C8802118E34".parse().unwrap();
    //     let expected = 12;
    //
    //     assert_eq!(expected, part1(packet));
    // }
    //
    // #[test]
    // fn part1_sample_input_3() {
    //     let packet = "C0015000016115A2E0802F182340".parse().unwrap();
    //     let expected = 23;
    //
    //     assert_eq!(expected, part1(packet));
    // }
    //
    // #[test]
    // fn part1_sample_input_4() {
    //     let packet = "A0016C880162017C3686B18A3D4780".parse().unwrap();
    //     let expected = 31;
    //
    //     assert_eq!(expected, part1(packet));
    // }
}
