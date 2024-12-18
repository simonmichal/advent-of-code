use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input


fn is_increasing( numbers: &[i32] ) -> bool {
    if numbers.len() < 2 { return true; }
    for i in 1..numbers.len() {
        if numbers[i] <= numbers[i-1] {
            return false;
        }
    }
    true
}

fn is_decreasing( numbers: &[i32] ) -> bool {
    if numbers.len() < 2 { return true; }
    for i in 1..numbers.len() {
        if numbers[i] >= numbers[i-1] {
            return false;
        }
    }
    true
}

fn differs_by_3( numbers: &[i32] ) -> bool {
    if numbers.len() < 2 { return true; }
    for i in 1..numbers.len() {
        if (numbers[i] - numbers[i-1]).abs() > 3 {
            return false;
        }
    }
    true
}

fn is_save( numbers: &[i32] ) -> bool {
    if numbers.len() < 2 { return true; }
    if !is_increasing( numbers ) && !is_decreasing( numbers ) {
        return false;
    }
    differs_by_3( numbers)
}

fn is_save_except_1( numbers: &[i32] ) -> bool {
    if is_save( numbers ) { return true; }

    for i in 0..numbers.len() {
        let mut numbers_except_1 = numbers.to_vec();
        numbers_except_1.remove( i );
        if is_save( &numbers_except_1 ) { return true; }
    }

    false
}

fn main() {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> i32 {
        let mut ans = 0;
        for line in reader.lines().filter_map(|line| line.ok()) {
            let lvls = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<_>>();
            if is_save(&lvls) {
                ans += 1;
            }
        }
        ans
    }

    fn part2<R: BufRead>(reader: R) -> i32 {
        let mut ans = 0;
        for line in reader.lines().filter_map(|line| line.ok()) {
            let lvls = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<_>>();
            if is_save_except_1(&lvls) {
                ans += 1;
            }
        }
        ans
    }

    if let Ok( file ) = File::open(INPUT_FILE) {
        let input_file = BufReader::new( file );
        //region Part 1
        // println!("=== Part 1 ===");
        // let answer = part1(input_file);
        // println!("answer = {answer}");
        //region Part 2
        println!("\n=== Part 2 ===");
        let answer = part2(input_file);
        println!("answer = {answer}");
    }
}
