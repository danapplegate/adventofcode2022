use crate::solvable::Solvable;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::fs;
use std::{error::Error, path::PathBuf};

const DEBUG: bool = false;

pub struct Solution {
    filepath: PathBuf,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PacketData {
    Integer(u32),
    List(Vec<PacketData>),
}
use PacketData::*;

impl PacketData {
    fn new_list() -> PacketData {
        PacketData::List(vec![])
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Integer(a) => f.write_str(&a.to_string()),
            List(l) => {
                f.write_char('[')?;
                f.write_str(
                    &l.iter()
                        .map(|pd| pd.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )?;
                f.write_char(']')
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if DEBUG {
            println!("Compare {} vs {}", self, other);
        }
        match (self, other) {
            (Integer(a), Integer(b)) => {
                if DEBUG {
                    println!(" - Integer({}) vs Integer({})", a, b);
                }
                Some(a.cmp(b))
            }
            (List(pda), List(pdb)) => {
                if DEBUG {
                    println!(" - List({:?}) vs List({:?})", pda, pdb);
                }
                for (left, right) in pda.iter().zip(pdb) {
                    if DEBUG {
                        println!(" - - {:?} vs {:?}", left, right);
                    }
                    match left.cmp(right) {
                        o @ Ordering::Less | o @ Ordering::Greater => {
                            return Some(o);
                        }
                        _ => {}
                    }
                }
                return Some(pda.len().cmp(&pdb.len()));
            }
            (a @ Integer(..), b @ List(..)) => Some(List(vec![a.clone()]).cmp(b)),
            (a @ List(..), b @ Integer(..)) => Some(a.cmp(&List(vec![b.clone()]))),
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Packet(PacketData);

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        let mut chars = input.chars().peekable();
        let mut list_stacks: Vec<PacketData> = vec![];
        loop {
            // if DEBUG {
            //     println!("List stacks:");
            //     for ls in &list_stacks {
            //         println!("{:?}", ls);
            //     }
            // }
            match chars.next() {
                Some('[') => list_stacks.push(PacketData::new_list()),
                Some(']') => {
                    let closed_stack = list_stacks.pop().unwrap();
                    if let Some(&mut PacketData::List(ref mut top_stack)) = list_stacks.last_mut() {
                        top_stack.push(closed_stack);
                    } else {
                        return Self(closed_stack);
                    }
                }
                Some(',') => {}
                Some(c @ '0'..='9') => {
                    let mut number = c.to_string();
                    loop {
                        match chars.peek().unwrap() {
                            d @ '0'..='9' => {
                                number.push(*d);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    if let Some(&mut PacketData::List(ref mut top_stack)) = list_stacks.last_mut() {
                        top_stack.push(PacketData::Integer(
                            number.parse::<u32>().expect("Unrecognized number sequence"),
                        ));
                    }
                }
                None => break,
                Some(c) => panic!("Unexpected character {}", c),
            }
        }
        panic!("Malformed input {}", input);
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solvable<13> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let pairs = input.split("\n\n");
        let mut in_order: Vec<usize> = vec![];
        for (i, pair) in pairs.enumerate() {
            let mut split = pair.splitn(2, "\n");
            let left = Packet::from(split.next().unwrap());
            let right = Packet::from(split.next().unwrap());
            if DEBUG {
                println!("Comparing {} vs {}", left, right);
            }
            if left < right {
                if DEBUG {
                    println!("In order");
                }
                in_order.push(i + 1);
            } else {
                if DEBUG {
                    println!("Out of order");
                }
            }
        }
        Ok(in_order.iter().sum::<usize>().to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut packets = input
            .lines()
            .filter_map(|l| match l.trim() {
                "" => None,
                p @ _ => Some(Packet::from(p)),
            })
            .collect::<Vec<Packet>>();
        let divider1 = Packet::from("[[2]]");
        let divider2 = Packet::from("[[6]]");
        packets.push(divider1.clone());
        packets.push(divider2.clone());
        packets.sort();
        let mut product = 1;
        for (i, p) in packets.iter().enumerate() {
            if p == &divider1 || p == &divider2 {
                product *= i + 1
            }
        }
        Ok(product.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "13");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "140");
        Ok(())
    }

    #[test]
    fn packet_from_empty_bracket_string() {
        assert_eq!(Packet::from("[]"), Packet(List(vec![])));
    }

    #[test]
    fn packet_from_list_of_integers() {
        assert_eq!(
            Packet::from("[1,2,3]"),
            Packet(List(vec![Integer(1), Integer(2), Integer(3)]))
        )
    }

    #[test]
    fn packet_from_list_of_single_list() {
        assert_eq!(Packet::from("[[]]"), Packet(List(vec![List(vec![])])))
    }

    #[test]
    fn packet_from_list_of_lists() {
        assert_eq!(
            Packet::from("[[],[1,2,3]]"),
            Packet(List(vec![
                List(vec![]),
                List(vec![Integer(1), Integer(2), Integer(3)])
            ]))
        );
    }

    #[test]
    fn packet_from_list_with_multiple_digits() {
        assert_eq!(
            Packet::from("[12,384,3]"),
            Packet(List(vec![Integer(12), Integer(384), Integer(3)]))
        );
    }

    #[test]
    fn packet_pair1_should_compare_correctly() {
        let left = Packet::from("[1,1,3,1,1]");
        let right = Packet::from("[1,1,5,1,1]");
        assert!(left < right);
    }

    #[test]
    fn packet_pair2_should_compare_correctly() {
        let left = Packet::from("[[1],[2,3,4]]");
        let right = Packet::from("[[1],4]");
        assert!(left < right);
    }

    #[test]
    fn packet_pair3_should_compare_correctly() {
        let left = Packet::from("[9]");
        let right = Packet::from("[[8,7,6]]");
        assert!(left > right);
    }

    #[test]
    fn packet_pair4_should_compare_correctly() {
        let left = Packet::from("[9]");
        let right = Packet::from("[[8,7,6]]");
        assert!(left > right);
    }
}
