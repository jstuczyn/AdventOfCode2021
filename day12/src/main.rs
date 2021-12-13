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

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use utils::execute_slice;
use utils::input_read::read_parsed_line_input;

#[derive(Debug)]
struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn construct(raw_edges: &[Edge]) -> Self {
        let mut edges: HashMap<_, Vec<_>> = HashMap::new();
        for edge in raw_edges.iter().cloned() {
            edges
                .entry(edge.from.clone())
                .or_default()
                .push(edge.to.clone());
            edges.entry(edge.to).or_default().push(edge.from);
        }

        Graph { edges }
    }
}

#[derive(Debug)]
struct MalformedEdge;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    name: String,
    is_big: bool,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name, f)
    }
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.to_owned(),
            is_big: name.to_ascii_uppercase() == name,
        }
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn count_paths(&self, graph: &Graph, mut visited: HashSet<Node>, double_visit: bool) -> usize {
        if self.is_end() {
            return 1;
        }
        visited.insert(self.clone());

        let mut paths = 0;
        for node in graph.edges.get(self).unwrap() {
            if node.is_big || !visited.contains(node) {
                paths += node.count_paths(graph, visited.clone(), double_visit)
            } else if double_visit && !node.is_end() && !node.is_start() {
                paths += node.count_paths(graph, visited.clone(), false)
            }
        }
        paths
    }
}

#[derive(Debug, Clone)]
struct Edge {
    from: Node,
    to: Node,
}

impl FromStr for Edge {
    type Err = MalformedEdge;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = s.split('-');
        let from = Node::new(nodes.next().ok_or(MalformedEdge)?);
        let to = Node::new(nodes.next().ok_or(MalformedEdge)?);
        Ok(Edge { from, to })
    }
}

fn part1(input: &[Edge]) -> usize {
    let graph = Graph::construct(input);
    let start = Node {
        name: "start".to_owned(),
        is_big: false,
    };
    start.count_paths(&graph, HashSet::new(), false)
}

fn part2(input: &[Edge]) -> usize {
    let graph = Graph::construct(input);
    let start = Node {
        name: "start".to_owned(),
        is_big: false,
    };
    start.count_paths(&graph, HashSet::new(), true)
}

#[cfg(not(tarpaulin))]
fn main() {
    execute_slice("input", read_parsed_line_input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input1() {
        let input = vec![
            "start-A".parse().unwrap(),
            "start-b".parse().unwrap(),
            "A-c".parse().unwrap(),
            "A-b".parse().unwrap(),
            "b-d".parse().unwrap(),
            "A-end".parse().unwrap(),
            "b-end".parse().unwrap(),
        ];

        let expected = 10;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part1_sample_input2() {
        let input = vec![
            "dc-end".parse().unwrap(),
            "HN-start".parse().unwrap(),
            "start-kj".parse().unwrap(),
            "dc-start".parse().unwrap(),
            "dc-HN".parse().unwrap(),
            "LN-dc".parse().unwrap(),
            "HN-end".parse().unwrap(),
            "kj-sa".parse().unwrap(),
            "kj-HN".parse().unwrap(),
            "kj-dc".parse().unwrap(),
        ];

        let expected = 19;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part1_sample_input3() {
        let input = vec![
            "fs-end".parse().unwrap(),
            "he-DX".parse().unwrap(),
            "fs-he".parse().unwrap(),
            "start-DX".parse().unwrap(),
            "pj-DX".parse().unwrap(),
            "end-zg".parse().unwrap(),
            "zg-sl".parse().unwrap(),
            "zg-pj".parse().unwrap(),
            "pj-he".parse().unwrap(),
            "RW-he".parse().unwrap(),
            "fs-DX".parse().unwrap(),
            "pj-RW".parse().unwrap(),
            "zg-RW".parse().unwrap(),
            "start-pj".parse().unwrap(),
            "he-WI".parse().unwrap(),
            "zg-he".parse().unwrap(),
            "pj-fs".parse().unwrap(),
            "start-RW".parse().unwrap(),
        ];

        let expected = 226;

        assert_eq!(expected, part1(&input))
    }

    #[test]
    fn part2_sample_input1() {
        let input = vec![
            "start-A".parse().unwrap(),
            "start-b".parse().unwrap(),
            "A-c".parse().unwrap(),
            "A-b".parse().unwrap(),
            "b-d".parse().unwrap(),
            "A-end".parse().unwrap(),
            "b-end".parse().unwrap(),
        ];

        let expected = 36;

        assert_eq!(expected, part2(&input))
    }

    #[test]
    fn part2_sample_input2() {
        let input = vec![
            "dc-end".parse().unwrap(),
            "HN-start".parse().unwrap(),
            "start-kj".parse().unwrap(),
            "dc-start".parse().unwrap(),
            "dc-HN".parse().unwrap(),
            "LN-dc".parse().unwrap(),
            "HN-end".parse().unwrap(),
            "kj-sa".parse().unwrap(),
            "kj-HN".parse().unwrap(),
            "kj-dc".parse().unwrap(),
        ];

        let expected = 103;

        assert_eq!(expected, part2(&input))
    }

    #[test]
    fn part2_sample_input3() {
        let input = vec![
            "fs-end".parse().unwrap(),
            "he-DX".parse().unwrap(),
            "fs-he".parse().unwrap(),
            "start-DX".parse().unwrap(),
            "pj-DX".parse().unwrap(),
            "end-zg".parse().unwrap(),
            "zg-sl".parse().unwrap(),
            "zg-pj".parse().unwrap(),
            "pj-he".parse().unwrap(),
            "RW-he".parse().unwrap(),
            "fs-DX".parse().unwrap(),
            "pj-RW".parse().unwrap(),
            "zg-RW".parse().unwrap(),
            "start-pj".parse().unwrap(),
            "he-WI".parse().unwrap(),
            "zg-he".parse().unwrap(),
            "pj-fs".parse().unwrap(),
            "start-RW".parse().unwrap(),
        ];

        let expected = 3509;

        assert_eq!(expected, part2(&input))
    }
}
