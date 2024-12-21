use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


struct Solution {
    diskmap : Vec<(usize, Vec<u32>)>,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut diskmap = Vec::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|line| line.ok()) {
                // let line = "2333133121414131402".to_string();
                let mut empty_space = false;
                let mut fileid = 0;
                for digit in line.chars().filter_map(|c| c.to_digit(10)) {
                    if empty_space {
                        diskmap.push((digit as usize, Vec::new()));
                        empty_space = false;
                    } else {
                        diskmap.push((0, vec![ fileid ; digit as usize ]));
                        fileid += 1;
                        empty_space = true;
                    }
                }
            }
        }

        Solution {
            diskmap,
        }
    }

    fn compactize1(&mut self) {
        let mut left = 1;
        let mut right = self.diskmap.len() - 1;
        if self.diskmap[right].0 > 0 {
            right -= 1;
        }

        while left < right {
            if self.diskmap[left].0 == 0 {
                left += 1;
                continue;
            }
            if let Some( value ) = self.diskmap[right].1.pop() {
                self.diskmap[left].1.push(value);
                self.diskmap[left].0 -= 1;
            }
            else {
                right -= 1;
            }
        }
    }

    fn compactize2(&mut self) {
        let mut right = self.diskmap.len() - 1;
        if self.diskmap[right].0 > 0 { // if last block is empty, shift by one
            right -= 1;
        }
        while right > 0 {
            if self.diskmap[right].1.is_empty() {
                right -= 2;
                continue;
            }
            let len = self.diskmap[right].1.len();
            let mut left = 1;
            while left < right {
                if self.diskmap[left].0 >= len {
                    while !self.diskmap[right].1.is_empty() {
                        if let Some( value ) = self.diskmap[right].1.pop() {
                            self.diskmap[left].1.push(value);
                        }
                    }
                    self.diskmap[right].0 += len;
                    self.diskmap[left].0 -= len;
                    break;
                }
                left += 2;
            }
            right -= 2;
        }
        // println!("{:?}", self.diskmap);
        // println!("00992111777.44.333....5555.6666.....8888..");
    }

    fn calculate_checksum(&self) -> u64 {
        let mut result = 0;
        let mut idx = 0;
        for (capacity, buff) in self.diskmap.iter() {
            for val in buff.iter() {
                result += (idx as u64) * ( *val as u64 ) ;
                idx += 1;
            }
            if *capacity > 0 {
                idx += capacity;
            }
        }

        result
    }

    fn solve1(&mut self) -> u64 {
        self.compactize1();
        self.calculate_checksum()
    }

    fn solve2(&mut self) -> u64 {
        self.compactize2();
        self.calculate_checksum()
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
