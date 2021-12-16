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
use bitvec::view::BitView;
use std::str::FromStr;
use utils::execution::execute_struct;
use utils::input_read::read_parsed;

#[derive(Debug)]
struct MalformedPacket;

const LITERAL_VAL_ID: u64 = 4;

fn bits_to_u64(bits: &BitSlice<u8, Msb0>) -> u64 {
    let mut res = 0u64;
    res.view_bits_mut::<Msb0>()[u64::BITS as usize - bits.len()..].clone_from_bitslice(bits);
    res
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Type {
    Literal,
    Operator(u64),
}

impl From<u64> for Type {
    fn from(val: u64) -> Self {
        match val {
            n if n == LITERAL_VAL_ID => Type::Literal,
            n => Type::Operator(n),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Header {
    version: u64,
    type_id: Type,
}

impl Header {
    const LEN: usize = 6;

    fn from_bits(bits: &BitSlice<u8, Msb0>) -> Self {
        let version = bits_to_u64(&bits[..3]);
        let type_id_u64 = bits_to_u64(&bits[3..6]);
        let type_id = Type::from(type_id_u64);

        Header { version, type_id }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Content {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl Content {
    fn parse_literal_value(bits: &BitSlice<u8, Msb0>) -> (Self, usize) {
        let mut i = 0;
        let mut literal_bits: BitVec<u8, Msb0> = BitVec::new();

        loop {
            literal_bits.push(bits[i + 1]);
            literal_bits.push(bits[i + 2]);
            literal_bits.push(bits[i + 3]);
            literal_bits.push(bits[i + 4]);

            i += 5;

            if !bits[i - 5] {
                break;
            }
        }

        (Content::Literal(bits_to_u64(&literal_bits)), i)
    }

    fn parse_operator_length_type_1(bits: &BitSlice<u8, Msb0>) -> (Self, usize) {
        let mut sub_packets = Vec::new();
        // The next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
        let num_packets = bits_to_u64(&bits[..11]);
        let mut i = 11;

        for _ in 0..num_packets {
            let (inner_packet, used_bytes) = Packet::from_bits(&bits[i..]);
            sub_packets.push(inner_packet);
            i += used_bytes;
        }

        (Content::Operator(sub_packets), i)
    }

    fn parse_operator_length_type_0(bits: &BitSlice<u8, Msb0>) -> (Self, usize) {
        let mut sub_packets = Vec::new();
        // The next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
        let subpackets_len = bits_to_u64(&bits[..15]);
        let mut bytes_left = subpackets_len as usize;
        let mut i = 15;
        while bytes_left > 0 {
            let (inner_packet, used_bytes) = Packet::from_bits(&bits[i..]);
            sub_packets.push(inner_packet);

            i += used_bytes;
            bytes_left -= used_bytes;
        }
        (Content::Operator(sub_packets), i)
    }

    fn from_bits(bits: &BitSlice<u8, Msb0>, typ: Type) -> (Self, usize) {
        if typ == Type::Literal {
            Self::parse_literal_value(bits)
        } else {
            let length_type_id = bits[0];
            if length_type_id {
                let (content, used_bytes) = Self::parse_operator_length_type_1(&bits[1..]);
                (content, used_bytes + 1)
            } else {
                let (content, used_bytes) = Self::parse_operator_length_type_0(&bits[1..]);
                (content, used_bytes + 1)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    header: Header,
    content: Content,
}

impl Packet {
    fn version_sum(&self) -> usize {
        match &self.content {
            Content::Literal(_) => self.header.version as usize,
            Content::Operator(operands) => {
                self.header.version as usize
                    + operands
                        .iter()
                        .map(|packet| packet.version_sum())
                        .sum::<usize>()
            }
        }
    }
}

impl FromStr for Packet {
    type Err = MalformedPacket;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = hex::decode(s).map_err(|_| MalformedPacket)?;
        let bits = BitVec::<u8, Msb0>::from_slice(&decoded);
        let bit_slice = bits.as_bitslice();
        let (packet, _) = Packet::from_bits(bit_slice);
        Ok(packet)
    }
}

impl Packet {
    fn from_bits(bits: &BitSlice<u8, Msb0>) -> (Self, usize) {
        let header = Header::from_bits(&bits[..6]);
        let (content, bytes_used) = Content::from_bits(&bits[6..], header.type_id);
        let packet = Packet { header, content };
        (packet, bytes_used + Header::LEN)
    }
}

fn part1(packet: Packet) -> usize {
    packet.version_sum()
}

fn part2(_packet: Packet) -> usize {
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

    #[test]
    fn operator_type0_packet_parsing() {
        let packet = "38006F45291200".parse().unwrap();
        let expected = Packet {
            header: Header {
                version: 1,
                type_id: Type::Operator(6),
            },
            content: Content::Operator(vec![
                Packet {
                    header: Header {
                        version: 6,
                        type_id: Type::Literal,
                    },
                    content: Content::Literal(10),
                },
                Packet {
                    header: Header {
                        version: 2,
                        type_id: Type::Literal,
                    },
                    content: Content::Literal(20),
                },
            ]),
        };

        assert_eq!(expected, packet);
    }

    #[test]
    fn operator_type1_packet_parsing() {
        let packet = "EE00D40C823060".parse().unwrap();
        let expected = Packet {
            header: Header {
                version: 7,
                type_id: Type::Operator(3),
            },
            content: Content::Operator(vec![
                Packet {
                    header: Header {
                        version: 2,
                        type_id: Type::Literal,
                    },
                    content: Content::Literal(1),
                },
                Packet {
                    header: Header {
                        version: 4,
                        type_id: Type::Literal,
                    },
                    content: Content::Literal(2),
                },
                Packet {
                    header: Header {
                        version: 1,
                        type_id: Type::Literal,
                    },
                    content: Content::Literal(3),
                },
            ]),
        };

        assert_eq!(expected, packet);
    }

    #[test]
    fn part1_sample_input_1() {
        let packet = "8A004A801A8002F478".parse().unwrap();
        let expected = 16;

        assert_eq!(expected, part1(packet));
    }

    #[test]
    fn part1_sample_input_2() {
        let packet = "620080001611562C8802118E34".parse().unwrap();
        let expected = 12;

        assert_eq!(expected, part1(packet));
    }

    #[test]
    fn part1_sample_input_3() {
        let packet = "C0015000016115A2E0802F182340".parse().unwrap();
        let expected = 23;

        assert_eq!(expected, part1(packet));
    }

    #[test]
    fn part1_sample_input_4() {
        let packet = "A0016C880162017C3686B18A3D4780".parse().unwrap();
        let expected = 31;

        assert_eq!(expected, part1(packet));
    }
}
