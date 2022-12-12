#[allow(dead_code)]
use crate::solvable::Solvable;
use std::collections::VecDeque;
use std::mem;
use std::{error::Error, path::PathBuf};

#[allow(dead_code)]
pub struct Solution<'a> {
    filepath: PathBuf,
    starting_state: Vec<Monkey<'a>>,
}

#[derive(Clone)]
struct Monkey<'a> {
    items: VecDeque<u128>,
    operation: &'a dyn Fn(u128) -> u128,
    divisor: u128,
    true_target: usize,
    false_target: usize,
    num_inspections: u128,
}

impl<'a> Monkey<'a> {
    fn new(
        items: Vec<u128>,
        operation: &'a dyn Fn(u128) -> u128,
        divisor: u128,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Monkey {
            items: VecDeque::from(items),
            num_inspections: 0,
            operation,
            divisor,
            true_target,
            false_target,
        }
    }
}

fn setup_test1<'a>() -> Vec<Monkey<'a>> {
    vec![
        Monkey::new(vec![79, 98], &|x| x * 19, 23, 2, 3),
        Monkey::new(vec![54, 65, 75, 74], &|x| x + 6, 19, 2, 0),
        Monkey::new(vec![79, 60, 97], &|x| x * x, 13, 1, 3),
        Monkey::new(vec![74], &|x| x + 3, 17, 0, 1),
    ]
}

fn setup_input<'a>() -> Vec<Monkey<'a>> {
    vec![
        Monkey::new(vec![59, 65, 86, 56, 74, 57, 56], &|x| x * 17, 3, 3, 6),
        Monkey::new(vec![63, 83, 50, 63, 56], &|x| x + 2, 13, 3, 0),
        Monkey::new(vec![93, 79, 74, 55], &|x| x + 1, 2, 0, 1),
        Monkey::new(vec![86, 61, 67, 88, 94, 69, 56, 91], &|x| x + 7, 11, 6, 7),
        Monkey::new(vec![76, 50, 51], &|x| x * x, 19, 2, 5),
        Monkey::new(vec![77, 76], &|x| x + 8, 17, 2, 1),
        Monkey::new(vec![74], &|x| x * 2, 5, 4, 7),
        Monkey::new(vec![86, 85, 52, 86, 91, 95], &|x| x + 6, 7, 4, 5),
    ]
}

#[allow(dead_code)]
fn print_state(state: &Vec<Monkey>) {
    for (i, m) in state.iter().enumerate() {
        println!("Monkey {i}: {:?}", m.items);
    }
}

impl<'a> Solution<'a> {
    fn solve(&self, divide_by_3: bool, n_rounds: u128) -> u128 {
        let mut state = self.starting_state.clone();
        let m_count = state.len();
        let product: u128 = self.starting_state.iter().map(|m| m.divisor).product();
        #[allow(unused_variables)]
        for r in 0..n_rounds {
            for i in 0..m_count {
                let monkey = state.get_mut(i).unwrap();
                let mut items = mem::take(&mut monkey.items);
                monkey.num_inspections += items.len() as u128;
                let operation = monkey.operation.clone();
                let divisor = monkey.divisor.clone();
                let true_target = monkey.true_target;
                let false_target = monkey.false_target;
                while let Some(item) = items.pop_front() {
                    let mut new_value = (*operation)(item);
                    if divide_by_3 {
                        new_value /= 3;
                    } else {
                        new_value %= product;
                    }
                    if new_value % divisor == 0 {
                        state
                            .get_mut(true_target)
                            .unwrap()
                            .items
                            .push_back(new_value);
                    } else {
                        state
                            .get_mut(false_target)
                            .unwrap()
                            .items
                            .push_back(new_value);
                    }
                }
            }
            // println!("After round {}:", r + 1);
            // print_state(&state);
            // println!();
        }

        let mut inspections = state
            .iter()
            .map(|m| m.num_inspections.clone())
            .collect::<Vec<u128>>();
        inspections.sort();
        inspections.reverse();
        &inspections[0] * &inspections[1]
    }
}

impl<'a> Solvable<11> for Solution<'a> {
    fn new(filename: &str) -> Self {
        Solution {
            starting_state: match filename {
                "test1.txt" => setup_test1(),
                "input.txt" => setup_input(),
                _ => panic!("Unknown starting state configuration {filename}"),
            },
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.solve(true, 20).to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.solve(false, 10_000).to_string())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "10605");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "2713310158");
        Ok(())
    }
}
