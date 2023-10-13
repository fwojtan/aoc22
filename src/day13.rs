use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day13;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Value(u64),
}

fn parse_packet(input: &str) -> Packet {
    let mut input = input.chars().peekable();
    let mut current = Packet::List(Vec::new());
    let mut stack = Vec::new();
    let mut value = None;

    input.next(); // skip the first [

    while let Some(c) = input.next() {
        if let Some(inner_value) = value {
            match current {
                Packet::List(ref mut list) => {
                    list.push(Packet::Value(inner_value));
                    value = None;
                }
                Packet::Value(_) => {
                    panic!("Current should only ever be a list variant")
                }
            }
        }
        match c {
            '[' => {
                stack.push(current.clone());
                current = Packet::List(Vec::new());
            }
            ']' => {
                // If packet has a parent, pop it off the stack and push the current packet to it
                if let Some(mut parent) = stack.pop() {
                    match parent {
                        Packet::List(ref mut list) => {
                            list.push(current);
                        }
                        Packet::Value(_) => {
                            panic!("Should only ever be in a list when hitting ]...")
                        }
                    }
                    current = parent;
                }
            }
            '0'..='9' => {
                if c == '1' && input.peek() == Some(&'0') {
                    input.next();
                    value = Some(10);
                } else {
                    value = Some(c.to_digit(10).unwrap() as u64);
                }
            }
            ',' => {}
            _ => panic!("Invalid character in packet"),
        }
    }
    current
}

impl Solution for Day13 {
    type ParsedInput = (HashMap<usize, (Packet, Packet)>, Vec<Packet>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut data1: HashMap<usize, (Packet, Packet)> = HashMap::new();
        let mut data2: Vec<Packet> = Vec::new();
        for (idx, packet_pair) in input_lines.split("\n\n").enumerate() {
            let mut packet_pair = packet_pair.split("\n");
            let packet_1 = parse_packet(packet_pair.next().unwrap());
            let packet_2 = parse_packet(packet_pair.next().unwrap());
            data1.insert(idx + 1, (packet_1.clone(), packet_2.clone()));
            data2.push(packet_1);
            data2.push(packet_2);
        }
        data2.push(parse_packet("[[2]]"));
        data2.push(parse_packet("[[6]]"));

        (data1, data2)
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let mut correct_idxs = vec![];
        for (k, (packet_1, packet_2)) in input.0.iter() {
            // println!("Comparing:");
            // println!("{:?}", packet_1);
            // println!("{:?}", packet_2);
            let res = compare_packets(packet_1, packet_2);
            if res.is_some_and(|res| res) {
                // println!("Packets with index {} are in correct order!", k);
                correct_idxs.push(*k);
            }
        }
        correct_idxs.iter().sum::<usize>().to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut packets = input.1.clone();
        let first_marker = parse_packet("[[2]]");
        let second_marker = parse_packet("[[6]]");
        packets.sort();
        let mut first = 0;
        let mut second = 0;
        for (position, packet) in packets.iter().enumerate() {
            if packet == &first_marker {
                first = position + 1;
            }
            if packet == &second_marker {
                second = position + 1;
            }
        }

        (first * second).to_string()
    }
}

fn compare_packets(left: &Packet, right: &Packet) -> Option<bool> {
    match (left, right) {
        (Packet::Value(left), Packet::Value(right)) => {
            // println!("Both items are integers, comparing...");
            if left < right {
                return Some(true);
            } else if left > right {
                return Some(false);
            }
        }
        (Packet::List(left), Packet::List(right)) => {
            // println!("Both items are lists, comparing item-by-item...");
            let mut left_it = left.iter();
            let mut right_it = right.iter();
            loop {
                let left_p = left_it.next();
                let right_p = right_it.next();
                // println!("Comparing list items... \n{:?}\n{:?}\n", left_p, right_p);

                match (left_p, right_p) {
                    (None, None) => break,
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (Some(l), Some(r)) => {
                        // println!("Both items are exist, comparing...");
                        if let Some(res) = compare_packets(l, r) {
                            return Some(res);
                        }
                    }
                }
            }
        }
        (Packet::Value(left), Packet::List(_)) => {
            // println!("Converting left to list and comparing...");
            let left = Packet::List(vec![Packet::Value(*left)]);
            if let Some(res) = compare_packets(&left, right) {
                return Some(res);
            }
        }
        (Packet::List(_), Packet::Value(right)) => {
            // println!("Converting right to list and comparing...");
            let right = Packet::List(vec![Packet::Value(*right)]);
            if let Some(res) = compare_packets(left, &right) {
                return Some(res);
            }
        }
    }
    None
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left_before_right = compare_packets(self, other);
        left_before_right.map(|res| {
            if res {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left_before_right = compare_packets(self, other);
        left_before_right
            .map(|res| {
                if res {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(
            Day13::solve_part_one(
                r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            "13".to_string()
        )
    }

    // #[test]
    // fn check_day13_part2_case1() {
    //     assert_eq!(Day13::solve_part_two(""), "0".to_string())
    // }

    // #[test]
    // fn check_day13_both_case1() {
    //     assert_eq!(Day13::solve("", false), ("0".to_string(), "0".to_string()))
    // }

    #[test]
    fn day13_parse_basic() {
        assert_eq!(
            parse_packet("[1,2,3]"),
            Packet::List(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)])
        );
    }

    #[test]
    fn day13_parse_nested() {
        assert_eq!(
            parse_packet("[1,[2,3]]"),
            Packet::List(vec![
                Packet::Value(1),
                Packet::List(vec![Packet::Value(2), Packet::Value(3)])
            ])
        );
    }

    #[test]
    fn day13_parse_nested2() {
        assert_eq!(
            parse_packet("[[2,3],1]"),
            Packet::List(vec![
                Packet::List(vec![Packet::Value(2), Packet::Value(3)]),
                Packet::Value(1)
            ])
        );
    }

    #[test]
    fn day13_parse_empty() {
        assert_eq!(
            parse_packet("[[],1]"),
            Packet::List(vec![Packet::List(vec![]), Packet::Value(1)])
        );
    }

    #[test]
    fn day13_parse_single() {
        assert_eq!(parse_packet("[1]"), Packet::List(vec![Packet::Value(1)]));
    }

    #[test]
    fn day13_compare_true1() {
        assert!(compare_packets(
            &Packet::List(vec![
                Packet::List(vec![Packet::Value(1)]),
                Packet::List(vec![Packet::Value(2), Packet::Value(3), Packet::Value(4)])
            ]),
            &Packet::List(vec![Packet::List(vec![Packet::Value(1)]), Packet::Value(4)])
        )
        .unwrap());
    }

    #[test]
    fn day13_compare_false1() {
        assert!(!compare_packets(
            &Packet::List(vec![Packet::Value(9)]),
            &Packet::List(vec![Packet::List(vec![
                Packet::Value(8),
                Packet::Value(7),
                Packet::Value(6)
            ])])
        )
        .unwrap());
    }
}
