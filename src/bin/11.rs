use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


struct Solution {
    stones  : Vec<u64>,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut stones = Vec::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|line| line.ok()) {
                stones = line.split(' ').filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<_>>()
            }
        }

        Solution {
            stones,
        }
    }

    fn get_number( digits : &mut Vec<u32>, len : usize ) -> u64 {
        let mut result = 0;
        for i in 0..len {
            if let Some(value ) = digits.pop() {
                result *= 10;
                result += value as u64;
            }
        }
        result
    }

    fn evolve( value : u64 ) -> Vec<u64> {
        if value == 0 {
            return vec![1];
        }
        let mut digits = value.to_string().chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
        if digits.len() % 2 == 0 {
            digits.reverse();
            let len = digits.len() / 2;
            let fstval = Self::get_number( &mut digits, len );
            let sndval = Self::get_number( &mut digits, len );
            return vec![fstval, sndval];
        }
        vec![value * 2024]
    }

    fn blink(stone : u64, count : usize, memo : &mut HashMap<(u64, usize),usize> ) -> usize {
        if let Some(&result) = memo.get(&(stone, count)) {
            return result;
        }
        if count == 0 {
            return 1;
        }
        let stones = Self::evolve(stone);
        let mut result = 0;
        for stone in stones {
            result += Self::blink(stone, count - 1, memo);
        }
        memo.insert((stone, count), result);
        result
    }

    fn solve1(&self) -> usize {
        let mut result = 0;
        let mut memo = HashMap::new();
        for stone in self.stones.iter() {
            result += Self::blink( *stone, 25, &mut memo );
        }
        result
    }

    fn solve2(&self) -> usize {
        let mut result = 0;
        let mut memo = HashMap::new();
        for stone in self.stones.iter() {
            result += Self::blink( *stone, 75, &mut memo );
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
