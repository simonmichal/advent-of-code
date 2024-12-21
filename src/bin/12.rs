use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "12"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

struct Solution {
    garden  : Vec<Vec<(char, bool)>>,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut garden = Vec::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            let lines = [
                "AAAAAA",
                "AAABBA",
                "AAABBA",
                "ABBAAA",
                "ABBAAA",
                "AAAAAA"
            ];
            for line in reader.lines().filter_map(|line| line.ok()) {
                garden.push( line.chars().map(|c|(c, false)).collect() );
            }
        }

        Solution {
            garden,
        }
    }

    fn visit_region1(&mut self, plant : char, r : usize, c : usize, area : &mut u64, perimeter : &mut u64 ) {
        if plant != self.garden[r][c].0 { // we are out of our region
            *perimeter += 1;
            return;
        }
        if self.garden[r][c].1 { // we visited this place already
            return;
        }
        *area += 1;
        self.garden[r][c].1 = true; // mark as visited
        // up
        if r > 0 {
            self.visit_region1(plant, r - 1, c, area, perimeter);
        } else { *perimeter += 1; } // we are out of our region
        // down
        if r < self.garden.len() - 1 {
            self.visit_region1(plant, r + 1, c, area, perimeter);
        } else { *perimeter += 1; } // we are out of our region
        // left
        if c > 0 {
            self.visit_region1(plant, r, c - 1, area, perimeter);
        } else { *perimeter += 1; } // we are out of our region
        // right
        if c < self.garden[r].len() - 1 {
            self.visit_region1(plant, r, c + 1, area, perimeter);
        } else { *perimeter += 1; } // we are out of our region
    }

    fn count_vertical_sides(vertical_side : &mut Vec<(i64,i64)>) -> u64 {
        if vertical_side.is_empty() { return 0; }
        vertical_side.sort();
        let mut result = 1;
        let mut pos = vertical_side[0];
        for i in 1..vertical_side.len() {
            let next = vertical_side[i];
            if pos.0 != next.0 || pos.1 + 1 != next.1 {
                result += 1;
            }
            pos = next;
        }
        result
    }

    fn count_horizontal_sides(horizontal_side : &mut Vec<(i64,i64)>) -> u64 {
        horizontal_side.iter_mut().for_each(|pos| *pos = (pos.1, pos.0) );
        return Self::count_vertical_sides( horizontal_side );
    }

    fn visit_region2(&mut self, plant : char, r : i64, c : i64, area : &mut u64, direction: Direction,
                     up_side : &mut Vec<(i64,i64)>,
                     down_side : &mut Vec<(i64,i64)>,
                     left_side : &mut Vec<(i64,i64)>,
                     right_side : &mut Vec<(i64,i64)>) {
        if  r < 0 || r >= self.garden.len() as i64 ||
            c < 0 || c >= self.garden[r as usize].len() as i64 ||
            plant != self.garden[r as usize][c as usize].0 { // we are out of our region
            match direction {
                Direction::Left => left_side.push((r, c) ),
                Direction::Right => right_side.push((r, c) ),
                Direction::Up => up_side.push((r, c) ),
                Direction::Down => down_side.push((r, c) ),
                _ => {}
            }
            return;
        }
        if self.garden[r as usize][c as usize].1 { // we visited this place already
            return;
        }
        *area += 1;
        self.garden[r as usize][c as usize].1 = true; // mark as visited
        // up
        self.visit_region2(plant, r - 1, c, area, Direction::Up, up_side, down_side, left_side, right_side);
        // down
        self.visit_region2(plant, r + 1, c, area, Direction::Down, up_side, down_side, left_side, right_side);
        // left
        self.visit_region2(plant, r, c - 1, area, Direction::Left, up_side, down_side, left_side, right_side);
        // right
        self.visit_region2(plant, r, c + 1, area, Direction::Right, up_side, down_side, left_side, right_side);
    }

    fn solve1(&mut self) -> u64 {
        let mut result = 0;
        for r in 0..self.garden.len() {
            for c in 0..self.garden[r].len() {
                let mut area = 0;
                let mut perimeter = 0;
                let plant = self.garden[r][c].0;
                self.visit_region1(plant, r, c, &mut area, &mut perimeter);
                result += area * perimeter ;
            }
        }
        result
    }

    fn solve2(&mut self) -> u64 {
        let mut result = 0;
        for r in 0..self.garden.len() {
            for c in 0..self.garden[r].len() {
                let mut area = 0;
                let mut up_side = Vec::new();
                let mut down_side = Vec::new();
                let mut left_side = Vec::new();
                let mut right_side = Vec::new();
                let plant = self.garden[r][c].0;
                self.visit_region2(plant, r as i64, c as i64, &mut area, Direction::None, &mut up_side, &mut down_side, &mut left_side, &mut right_side);
                if area > 0 {
                    let up = Self::count_vertical_sides(&mut up_side);
                    let down = Self::count_vertical_sides(&mut down_side);
                    let left = Self::count_horizontal_sides(&mut left_side);
                    let right = Self::count_horizontal_sides(&mut right_side);

                    result += area * (up + down + left + right);
                }
            }
        }
        result
    }
}


fn main() {
    let mut solution = Solution::new(INPUT_FILE);
    let result1 = solution.solve1();
    println!("result1 = {}", result1);
    let mut solution = Solution::new(INPUT_FILE);
    let result2 = solution.solve2();
    println!("result2 = {}", result2);
}
