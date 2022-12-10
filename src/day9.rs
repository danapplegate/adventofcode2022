use crate::solvable::Solvable;
use std::collections::HashSet;
use std::fs;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn update(&mut self, h_pos: &Self) {
        if h_pos.x - 1 <= self.x
            && self.x <= h_pos.x + 1
            && h_pos.y - 1 <= self.y
            && self.y <= h_pos.y + 1
        {
            return;
        }

        if h_pos.x < self.x {
            self.x -= 1;
        } else if h_pos.x > self.x {
            self.x += 1;
        }
        if h_pos.y < self.y {
            self.y -= 1;
        } else if h_pos.y > self.y {
            self.y += 1;
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;
impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Unknown direction character {c}"),
        }
    }
}

fn process(input: &str, num_knots: usize) -> Result<usize, Box<dyn Error>> {
    let mut h_pos = Pos::default();
    let mut knots = vec![];
    knots.resize(num_knots, Pos::default());
    let mut t_visited: HashSet<Pos> = HashSet::new();
    t_visited.insert(knots.last().unwrap().clone());
    for instruction in input.lines() {
        let direction = Direction::from_char(instruction.chars().nth(0).unwrap());
        let steps = instruction[2..].parse::<u32>()?;
        for _ in 0..steps {
            h_pos = match direction {
                Up => Pos {
                    x: h_pos.x,
                    y: h_pos.y + 1,
                },
                Down => Pos {
                    x: h_pos.x,
                    y: h_pos.y - 1,
                },
                Left => Pos {
                    x: h_pos.x - 1,
                    y: h_pos.y,
                },
                Right => Pos {
                    x: h_pos.x + 1,
                    y: h_pos.y,
                },
            };

            knots.get_mut(0).unwrap().update(&h_pos);
            for i in 1..num_knots {
                let prev_knot = knots.get(i - 1).unwrap().clone();
                knots.get_mut(i).unwrap().update(&prev_knot);
            }
            t_visited.insert(knots.last().unwrap().clone());
        }
    }
    Ok(t_visited.len())
}

impl Solvable<9> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(process(&input, 1)?.to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(process(&input, 9)?.to_string())
    }
}

#[allow(unused_imports)]
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
        assert_eq!(solution.answer2()?, "1");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test2() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test2.txt");
        assert_eq!(solution.answer2()?, "36");
        Ok(())
    }
}
