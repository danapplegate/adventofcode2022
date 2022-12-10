use crate::solvable::Solvable;
use std::collections::HashSet;
use std::fs;
use std::ops::{Index, IndexMut};
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

#[derive(Debug)]
struct TreeGrid {
    rows: usize,
    columns: usize,
    grid: Vec<u8>,
}

impl TreeGrid {
    fn from_vec(v: Vec<u8>, columns: usize) -> Self {
        let rows = v.len() / columns;
        Self {
            rows,
            columns,
            grid: v,
        }
    }
}

impl Index<usize> for TreeGrid {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        let row_offset = self.columns * index;
        &self.grid[row_offset..(row_offset + self.columns)]
    }
}

impl IndexMut<usize> for TreeGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row_offset = self.columns * index;
        &mut self.grid[row_offset..(row_offset + self.columns)]
    }
}

impl TreeGrid {
    fn calculate_score(&self, r: usize, c: usize) -> u32 {
        let tree_height = self[r][c];
        let mut up_distance = 0;
        let mut up_blocked = false;
        let mut down_distance = 0;
        let mut down_blocked = false;
        let mut left_distance = 0;
        let mut left_blocked = false;
        let mut right_distance = 0;
        let mut right_blocked = false;
        let limit = self.rows.max(self.columns);
        for d in 1..limit as i32 {
            if up_blocked && down_blocked && left_blocked && right_blocked {
                break;
            }

            if !up_blocked && r as i32 - d >= 0 {
                up_distance += 1;
                if self[r - d as usize][c] >= tree_height {
                    up_blocked = true;
                }
            }
            if !down_blocked && r + (d as usize) < self.rows {
                down_distance += 1;
                if self[r + d as usize][c] >= tree_height {
                    down_blocked = true;
                }
            }
            if !left_blocked && c as i32 - d >= 0 {
                left_distance += 1;
                if self[r][c - d as usize] >= tree_height {
                    left_blocked = true;
                }
            }
            if !right_blocked && c + (d as usize) < self.columns {
                right_distance += 1;
                if self[r][c + d as usize] >= tree_height {
                    right_blocked = true;
                }
            }
        }
        up_distance * down_distance * left_distance * right_distance
    }
}

impl Solution {
    fn create_grid(&self) -> Result<TreeGrid, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut v: Vec<u8> = Vec::new();
        let columns = input.lines().next().unwrap().len();
        for line in input.lines() {
            v.extend::<Vec<u8>>(line.bytes().map(|b| b - 48).collect());
        }
        Ok(TreeGrid::from_vec(v, columns))
    }
}

impl Solvable<8> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let grid = self.create_grid()?;
        let mut visible: HashSet<(usize, usize)> = HashSet::new();
        for r in 0..grid.rows {
            let mut max = -1;
            let mut rmax = -1;
            for c in 0..grid.columns {
                if grid[r][c] as i8 > max {
                    visible.insert((r, c));
                    max = grid[r][c] as i8;
                }

                let rc = grid.columns - 1 - c;
                if grid[r][rc] as i8 > rmax {
                    visible.insert((r, rc));
                    rmax = grid[r][rc] as i8;
                }
            }
        }

        for c in 1..(grid.columns - 1) {
            let mut max = -1;
            let mut rmax = -1;
            for r in 0..grid.rows {
                if grid[r][c] as i8 > max {
                    visible.insert((r, c));
                    max = grid[r][c] as i8;
                }

                let rr = grid.rows - 1 - r;
                if grid[rr][c] as i8 > rmax {
                    visible.insert((rr, c));
                    rmax = grid[rr][c] as i8;
                }
            }
        }
        Ok(visible.len().to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let grid = self.create_grid()?;
        let mut max_score = 0;
        for r in 1..(grid.rows - 1) {
            for c in 1..(grid.columns - 1) {
                let scenic_score = grid.calculate_score(r, c);
                if scenic_score > max_score {
                    max_score = scenic_score;
                }
            }
        }
        Ok(max_score.to_string())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "21");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "8");
        Ok(())
    }
}
