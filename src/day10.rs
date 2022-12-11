use crate::solvable::Solvable;
use std::fs;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

fn relevant_cycle(cycle: i32) -> bool {
    cycle == 20 || (cycle - 20) % 40 == 0
}

fn draw_pixel(screen_buffer: &mut [char], pixel: i32, sprite_pos: i32) {
    let p = pixel % 40;
    if sprite_pos - 1 <= p && p <= sprite_pos + 1 {
        screen_buffer[pixel as usize] = '#';
    }
}

impl Solvable<10> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut register = 1_i32;
        let mut cycle = 1_i32;
        let mut signal_strengths = vec![];
        for line in input.lines() {
            if relevant_cycle(cycle) {
                signal_strengths.push(cycle * register);
            }
            let instruction = &line[0..4];
            match instruction {
                "noop" => cycle += 1,
                "addx" => {
                    cycle += 1;
                    if relevant_cycle(cycle) {
                        signal_strengths.push(cycle * register);
                    }
                    cycle += 1;
                    let x = line[5..].parse::<i32>()?;
                    register += x;
                }
                _ => panic!("Unrecognized command {instruction}"),
            }
        }
        Ok(signal_strengths.iter().sum::<i32>().to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut screen_buffer = ['.'; 240];
        let mut sprite_pos = 1_i32;
        let mut cycle = 1;
        for line in input.lines() {
            let instruction = &line[0..4];
            draw_pixel(&mut screen_buffer, cycle - 1, sprite_pos);
            match instruction {
                "noop" => {
                    cycle += 1;
                }
                "addx" => {
                    cycle += 1;
                    draw_pixel(&mut screen_buffer, cycle - 1, sprite_pos);
                    cycle += 1;
                    let x = line[5..].parse::<i32>()?;
                    sprite_pos += x;
                }
                _ => panic!("Unrecognized command {instruction}"),
            }
        }

        // Write the final screen state to an output buffer
        let mut output_buffer = String::new();
        for (i, c) in screen_buffer.iter().enumerate() {
            if i > 0 && i % 40 == 0 {
                output_buffer.push('\n');
            }
            output_buffer.push(*c);
        }
        Ok(output_buffer)
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "13140");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(
            solution.answer2()?,
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######....."
        );
        Ok(())
    }
}
