use lazy_regex::{regex, Captures, Lazy, Regex};

use crate::solvable::Solvable;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
    row_to_inspect: i32,
    limit: i32,
}

static LINE_REGEX: &Lazy<Regex> = regex!(r"Sensor at (.+): closest beacon is at (.+)");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut location_parts = s.split(", ");
        let location_x = location_parts.next().unwrap();
        let location_y = location_parts.next().unwrap();

        Point {
            x: location_x[2..].parse::<i32>().expect("Invalid x"),
            y: location_y[2..].parse::<i32>().expect("Invalid y"),
        }
    }
}

impl Point {
    fn distance_to(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon_location: Point,
    beacon_distance: u32,
}

impl Sensor {
    fn x_bounds(&self) -> (i32, i32) {
        (
            self.location.x - self.beacon_distance as i32,
            self.location.x + self.beacon_distance as i32,
        )
    }

    fn in_range(&self, p: &Point) -> bool {
        self.location.distance_to(p) <= self.beacon_distance
    }

    fn points_on_range_radius(&self, limit: i32) -> Vec<Point> {
        let radius_distance = self.beacon_distance as i32 + 1;
        let mut points: HashSet<Point> = HashSet::new();
        for x in (self.location.x - radius_distance).max(0)
            ..=(self.location.x + radius_distance).min(limit)
        {
            let d = radius_distance - self.location.x.abs_diff(x) as i32;
            if self.location.y + d <= limit {
                points.insert(Point {
                    x,
                    y: self.location.y + d,
                });
            }
            if self.location.y - d >= 0 {
                points.insert(Point {
                    x,
                    y: self.location.y - d,
                });
            }
        }
        points.into_iter().collect::<Vec<Point>>()
    }
}

impl From<Captures<'_>> for Sensor {
    fn from(c: Captures) -> Self {
        let location = Point::from(c.get(1).unwrap().as_str());
        let beacon_location = Point::from(c.get(2).unwrap().as_str());
        let beacon_distance = location.distance_to(&beacon_location);

        Self {
            location,
            beacon_location,
            beacon_distance,
        }
    }
}

impl Solution {
    pub fn set_row_to_inspect(&mut self, row: i32) {
        self.row_to_inspect = row;
    }

    pub fn set_limit(&mut self, limit: i32) {
        self.limit = limit;
    }
}

impl Solvable<15> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
            row_to_inspect: 10,
            limit: 20,
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut max_left: i32 = 0;
        let mut max_right: i32 = 0;
        let mut beacon_locations: HashSet<Point> = HashSet::new();
        for line in input.lines() {
            let captures = LINE_REGEX.captures(line).unwrap();
            let sensor = Sensor::from(captures);
            beacon_locations.insert(sensor.beacon_location.clone());
            let (left_bound, right_bound) = sensor.x_bounds();
            max_left = max_left.min(left_bound);
            max_right = max_right.max(right_bound);

            sensors.push(sensor);
        }
        let mut num_invalid = 0;
        for x in max_left..=max_right {
            let p = Point {
                x,
                y: self.row_to_inspect,
            };
            if beacon_locations.contains(&p) {
                continue;
            }
            for sensor in &sensors {
                if sensor.in_range(&p) {
                    num_invalid += 1;
                    break;
                }
            }
        }
        Ok(num_invalid.to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut possible_locations: HashSet<Point> = HashSet::new();
        let mut sensors = Vec::new();

        println!("generating possible locations...");
        for line in input.lines() {
            let captures = LINE_REGEX.captures(line).unwrap();
            let sensor = Sensor::from(captures);
            for p in sensor.points_on_range_radius(self.limit) {
                possible_locations.insert(p);
            }
            sensors.push(sensor);
        }
        println!("locations to consider: {}", possible_locations.len());
        // Eliminate points that are out of bounds
        let locations_in_bounds: Vec<Point> = possible_locations
            .into_iter()
            .filter(|pl| pl.x >= 0 && pl.x <= self.limit && pl.y >= 0 && pl.y <= self.limit)
            .collect();
        println!("locations in bounds: {}", locations_in_bounds.len());
        let mut locations = Vec::new();
        'locations: for location in locations_in_bounds {
            for s in &sensors {
                if s.in_range(&location) {
                    continue 'locations;
                }
            }
            locations.push(location);
        }

        assert_eq!(locations.len(), 1);
        let final_location = locations.iter().next().unwrap();
        let answer: i128 = final_location.x as i128 * 4_000_000 + final_location.y as i128;
        Ok(answer.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_correct_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "26");
        Ok(())
    }

    #[test]
    fn it_returns_the_correct_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "56000011");
        Ok(())
    }
}
