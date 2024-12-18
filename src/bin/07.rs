use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


struct Solution {
    input : Vec<(u64, Vec<u64>)>
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut input = Vec::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|line| line.ok()) {
                let tokens = line.split(":").collect::<Vec<&str>>();
                if tokens.len() != 2 { continue; }
                if let Ok( total ) = tokens[0].parse::<u64>() {
                    let numbers = tokens[1].split(" ").filter_map(|s|s.parse::<u64>().ok()).collect::<Vec<_>>();
                    input.push((total, numbers));
                }
            }
        }

        Solution {
            input
        }
    }

    fn match_total( total : u64, values : &mut Vec<u64> ) -> bool {
        values.reverse();
        let ret = Self::match_total_impl(total, values);
        values.reverse();
        ret
    }

    fn match_total_impl( total :  u64, values : &mut Vec<u64> ) -> bool {
        if values.len() == 1 { return total == values[0]; }
        if let (Some(v1), Some(v2)) = ( values.pop(), values.pop() ) {
            // try addition
            if let Some(value) = v1.checked_add(v2) {
                values.push(value );
                let ret = Self::match_total_impl(total, values);
                values.pop();
                if ret {
                    values.push(v2);
                    values.push(v1);
                    return true;
                }
            }
            // try multiplication
            if let Some(value) = v1.checked_mul(v2) {
                values.push(value);
                let ret = Self::match_total_impl(total, values);
                values.pop();
                values.push(v2);
                values.push(v1);
                return ret;
            }
            // nothing worked, put it back
            values.push(v2);
            values.push(v1);
            return false;
        }
        false
    }

    fn match_total_cancat( total : u64, values : &mut Vec<u64> ) -> bool {
        values.reverse();
        let ret = Self::match_total_concat_impl(total, values);
        values.reverse();
        ret
    }

    fn concat(lhs: u64, rhs: u64) -> Option<u64> {
        format!("{}{}", lhs, rhs).parse().ok()
    }

    fn match_total_concat_impl( total :  u64, values : &mut Vec<u64> ) -> bool {
        if values.len() == 1 { return total == values[0]; }
        if let (Some(v1), Some(v2)) = ( values.pop(), values.pop() ) {
            // try concatenation
            if let Some(value) = Self::concat(v1, v2) {
                values.push(value );
                let ret = Self::match_total_concat_impl(total, values);
                values.pop();
                if ret {
                    values.push(v2);
                    values.push(v1);
                    return true;
                }
            }
            // try addition
            if let Some(value) = v1.checked_add(v2) {
                values.push(value );
                let ret = Self::match_total_concat_impl(total, values);
                values.pop();
                if ret {
                    values.push(v2);
                    values.push(v1);
                    return true;
                }
            }
            // try multiplication
            if let Some(value) = v1.checked_mul(v2) {
                values.push(value);
                let ret = Self::match_total_concat_impl(total, values);
                values.pop();
                values.push(v2);
                values.push(v1);
                return ret;
            }
            // nothing worked, put it back
            values.push(v2);
            values.push(v1);
            return false;
        }
        false
    }

    fn solve1(&mut self) -> u64 {
        let mut result = 0;
        for (total, numbers) in self.input.iter_mut() {
            if Self::match_total( *total, numbers) { result += *total; }
        }
        result
    }

    fn solve2(&mut self) -> u64 {
        let mut result = 0;
        for (total, numbers) in self.input.iter_mut() {
            if Self::match_total_cancat( *total, numbers) { result += *total; }
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
