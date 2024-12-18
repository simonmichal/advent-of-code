use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn gen_indices( r : usize, c : usize ) -> Vec<(usize, usize)>
{
    let mut result = Vec::new();
    // upper row
    if r > 0 {
        if c > 0 { result.push((r - 1, c - 1)); }
        result.push((r - 1, c));
        result.push((r - 1, c + 1) );
    }
    // same row
    if c > 0 { result.push( (r, c - 1 ) ); }
    result.push( (r, c + 1 ) );
    // lower row
    if c > 0 { result.push( (r + 1, c - 1 ) ); }
    result.push( (r + 1, c ) );
    result.push( (r + 1, c + 1 ) );
    result
}

fn count_at( input : &[Vec<u8>], r : usize, c : usize, word : &[u8] ) -> usize
{
    if word.is_empty() || r >= input.len() || c >= input[0].len() {
        return 0;
    }
    if input[r][c] != word[0] {
        return 0;
    }
    if word.len() == 1 {
        return 1;
    }

    let mut result = 0;
    for (r, c) in gen_indices( r, c ) {
        result += count_at( input, r, c, &word[1..])
    }
    result
}

fn main() {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> usize {

        let mut input : Vec<Vec<u8>> = Vec::new();
        for line in reader.lines().filter_map(|line| line.ok()) {
            input.push( line.as_bytes().to_vec() );
        }
        let word = b"XMAS";
        let mut ans = 0;
        for r in 0 .. input.len() {
            for c in 0 .. input[r].len() {
                print!("{}", input[r][c] as char);
                ans += count_at( &input, r, c, word );
            }
            println!();
        }
        ans
    }

    fn part2<R: BufRead>(reader: R) -> i32 {
        let mut ans = 0;
        let bytes = reader.bytes().filter_map( |r| r.ok() ).collect::<Vec<u8>>();
        ans
    }

    if let Ok( file ) = File::open(INPUT_FILE) {
        let input_file = BufReader::new( file );
        //region Part 1
        println!("=== Part 1 ===");
        let answer = part1(input_file);
        println!("answer = {answer}");
        //region Part 2
        // println!("\n=== Part 2 ===");
        // let answer = part2(input_file);
        // println!("answer = {answer}");
    }
}
