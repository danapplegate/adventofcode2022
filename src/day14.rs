use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::solvable::Solvable;
use std::collections::HashSet;
use std::fs;
use std::mem::swap;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

static LINE_REGEX: &Lazy<Regex> = regex!(r"(\d+,\d+)");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let mut parts = input.split(',');
        let x = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Malformed x coord");
        let y = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Malformed y coord");
        Point { x, y }
    }
}

fn drop_sand(
    cave: &HashSet<Point>,
    start: &Point,
    y_bound: usize,
    with_floor: bool,
) -> Option<Point> {
    let mut pos = start.clone();
    let floor = if with_floor { y_bound + 2 } else { y_bound + 1 };
    for y in start.y..=floor {
        if with_floor && y == floor {
            pos.y = floor - 1;
            return Some(pos);
        }
        if cave.contains(&Point { x: pos.x, y }) {
            if !cave.contains(&Point { x: pos.x - 1, y }) {
                pos.x -= 1;
            } else if !cave.contains(&Point { x: pos.x + 1, y }) {
                pos.x += 1;
            } else {
                pos.y = y - 1;
                return Some(pos);
            }
        }
    }
    None
}

impl Solution {
    fn solve(&self, input: &str, with_floor: bool) -> i32 {
        let mut cave: HashSet<Point> = HashSet::new();
        let mut highest_y = 0;
        for line in input.lines() {
            let captures: &[Captures] =
                &LINE_REGEX.captures_iter(line).collect::<Vec<Captures>>()[..];
            for points in captures.windows(2) {
                let (mut from, mut to) = (
                    Point::from(points[0].get(0).unwrap().as_str()),
                    Point::from(points[1].get(0).unwrap().as_str()),
                );
                if from.y > highest_y {
                    highest_y = from.y;
                }
                if to.y > highest_y {
                    highest_y = to.y;
                }
                if from.x == to.x {
                    if from.y > to.y {
                        swap(&mut from, &mut to);
                    }
                    for y in from.y..=to.y {
                        cave.insert(Point { x: from.x, y });
                    }
                } else {
                    if from.x > to.x {
                        swap(&mut from, &mut to);
                    }
                    for x in from.x..=to.x {
                        cave.insert(Point { x, y: from.y });
                    }
                }
            }
        }
        let sand_start = Point { x: 500, y: 0 };
        let mut num_sand = 0;
        while let Some(sand_stop) = drop_sand(&cave, &sand_start, highest_y, with_floor) {
            cave.insert(sand_stop);
            num_sand += 1;
            if with_floor && sand_stop == sand_start {
                return num_sand;
            }
        }
        num_sand
    }
}

impl Solvable<14> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(self.solve(&input, false).to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(self.solve(&input, true).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "24");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "93");
        Ok(())
    }
}
