use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


struct Solution {
    topographic_map  : Vec<Vec<u32>>,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut topographic_map = Vec::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            // let lines = [
            //     "89010123".to_string(),
            //     "78121874".to_string(),
            //     "87430965".to_string(),
            //     "96549874".to_string(),
            //     "45678903".to_string(),
            //     "32019012".to_string(),
            //     "01329801".to_string(),
            //     "10456732".to_string(),
            // ];
            for line in reader.lines().filter_map(|line| line.ok()) {
                topographic_map.push( line.chars().filter_map(|c|c.to_digit(10)).collect::<Vec<_>>() );
            }
        }

        Solution {
            topographic_map,
        }
    }

    fn score_trailhead(&self, pos : (usize, usize), expected : u32, score: &mut HashSet<(usize, usize)> ) {
        if self.topographic_map[pos.0][pos.1] != expected {
            return;
        }
        if self.topographic_map[pos.0][pos.1] == 9 {
            score.insert((pos.0, pos.1));
            return;
        }
        // up
        if pos.0 > 0 {
            self.score_trailhead( (pos.0 - 1, pos.1), expected + 1, score );
        }
        // down
        if pos.0 < self.topographic_map.len() - 1 {
            self.score_trailhead( (pos.0 + 1, pos.1), expected + 1, score );
        }
        // left
        if pos.1 > 0 {
            self.score_trailhead( (pos.0, pos.1 - 1), expected + 1, score );
        }
        // right
        if pos.1 < self.topographic_map[pos.0].len() - 1 {
            self.score_trailhead( (pos.0, pos.1 + 1), expected + 1, score );
        }
    }

    fn rate_trailhead(&self, pos : (usize, usize), expected : u32 ) -> usize {
        if self.topographic_map[pos.0][pos.1] != expected {
            return 0;
        }
        if self.topographic_map[pos.0][pos.1] == 9 {
            return 1;
        }
        let mut result = 0;
        // up
        if pos.0 > 0 {
            result += self.rate_trailhead( (pos.0 - 1, pos.1), expected + 1 );
        }
        // down
        if pos.0 < self.topographic_map.len() - 1 {
            result += self.rate_trailhead( (pos.0 + 1, pos.1), expected + 1 );
        }
        // left
        if pos.1 > 0 {
            result += self.rate_trailhead( (pos.0, pos.1 - 1), expected + 1 );
        }
        // right
        if pos.1 < self.topographic_map[pos.0].len() - 1 {
            result += self.rate_trailhead( (pos.0, pos.1 + 1), expected + 1 );
        }
        result
    }

    fn solve1(&mut self) -> usize {
        let mut result = 0;
        for i in 0..self.topographic_map.len() {
            for j in 0..self.topographic_map[i].len() {
                if self.topographic_map[i][j] == 0 {
                    let mut score = HashSet::new();
                    self.score_trailhead( (i, j), 0, &mut score) ;
                    result += score.len();
                }
            }
        }
        result
    }

    fn solve2(&mut self) -> usize {
        let mut result = 0;
        for i in 0..self.topographic_map.len() {
            for j in 0..self.topographic_map[i].len() {
                if self.topographic_map[i][j] == 0 {
                    result += self.rate_trailhead( (i, j), 0) ;
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
    let result2 = solution.solve2();
    println!("result2 = {}", result2);
}
