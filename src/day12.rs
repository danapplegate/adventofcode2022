use crate::solvable::Solvable;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    r: usize,
    c: usize,
}

impl Pos {}

#[derive(Debug)]
struct HeightMap {
    start: Pos,
    goal: Pos,
    starting_points: Vec<Pos>,
    rows: usize,
    columns: usize,
    height_grid: Vec<Vec<char>>,
    distance_grid: Option<Vec<Vec<u32>>>,
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Start position: {:?}\n", self.start))?;
        f.write_fmt(format_args!("Goal position: {:?}\n", self.goal))?;
        f.write_str("Height Grid:\n")?;
        for row in &self.height_grid {
            f.write_fmt(format_args!(
                "{}\n",
                row.iter().cloned().collect::<String>()
            ))?;
        }
        if let Some(dg) = &self.distance_grid {
            f.write_str("\nDistance Grid:\n")?;
            for row in dg {
                for d in row {
                    f.write_fmt(format_args!("{:>3}", d))?;
                }
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

impl HeightMap {
    fn new(input: &str) -> Self {
        let mut height_grid = vec![];
        let mut start = Pos { r: 0, c: 0 };
        let mut goal = Pos { r: 0, c: 0 };
        let mut starting_points = vec![];
        for (r, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (c, char) in line.chars().enumerate() {
                row.push(match char {
                    'S' => {
                        start = Pos { r, c };
                        starting_points.push(Pos { r, c });
                        'a'
                    }
                    'E' => {
                        goal = Pos { r, c };
                        'z'
                    }
                    'a' => {
                        starting_points.push(Pos { r, c });
                        'a'
                    }
                    value @ 'b'..='z' => value,
                    _ => panic!("Unrecognized character {char}"),
                })
            }
            height_grid.push(row);
        }
        HeightMap {
            start,
            goal,
            starting_points,
            rows: height_grid.len(),
            columns: height_grid.first().unwrap().len(),
            height_grid,
            distance_grid: None,
        }
    }

    fn height_at(&self, p: &Pos) -> char {
        self.height_grid[p.r][p.c]
    }

    fn nearby(&self, p: &Pos) -> Vec<Pos> {
        let mut n = vec![];
        if p.r > 0 {
            n.push(Pos { r: p.r - 1, c: p.c });
        }
        if p.r < self.rows - 1 {
            n.push(Pos { r: p.r + 1, c: p.c });
        }
        if p.c > 0 {
            n.push(Pos { r: p.r, c: p.c - 1 });
        }
        if p.c < self.columns - 1 {
            n.push(Pos { r: p.r, c: p.c + 1 });
        }
        n
    }

    fn calculate_shortest(&mut self) -> u32 {
        let mut distance_grid: Vec<Vec<u32>> = Vec::with_capacity(self.rows);
        distance_grid.resize_with(self.rows, || {
            let mut v = Vec::with_capacity(self.columns);
            v.resize(self.columns, u32::MAX);
            v
        });

        distance_grid[self.goal.r][self.goal.c] = 0;
        let mut to_visit: VecDeque<Pos> = VecDeque::new();
        to_visit.push_back(self.goal.clone());

        while let Some(current) = to_visit.pop_front() {
            let current_height = self.height_at(&current) as u8;
            let next_distance = distance_grid[current.r][current.c] + 1;
            for nearby in self.nearby(&current) {
                if self.height_at(&nearby) as u8 >= current_height - 1
                    && next_distance < distance_grid[nearby.r][nearby.c]
                {
                    distance_grid[nearby.r][nearby.c] = next_distance;
                    to_visit.push_back(nearby);
                }
            }
        }

        let distance = distance_grid[self.start.r][self.start.c];
        self.distance_grid = Some(distance_grid);
        distance
    }

    fn distance_at(&self, p: &Pos) -> u32 {
        self.distance_grid.as_ref().unwrap()[p.r][p.c]
    }
}

impl Solvable<12> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut height_map = HeightMap::new(&input);
        let distance = height_map.calculate_shortest();
        Ok(distance.to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut height_map = HeightMap::new(&input);
        height_map.calculate_shortest();
        let distance = height_map
            .starting_points
            .iter()
            .map(|p| height_map.distance_at(p))
            .min()
            .unwrap();
        Ok(distance.to_string())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "31");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "29");
        Ok(())
    }
}
