use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "03"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input


fn is_do( data : &[u8] ) -> bool {
    if data.len() < 4 { return false; }
    let expected = b"do()";
    for i in 0..expected.len() {
        if data[i] != expected[i] { return false; }
    }
    true
}

fn is_dont( data : &[u8] ) -> bool {
    if data.len() < 4 { return false; }
    let expected = b"don't()";
    for i in 0..expected.len() {
        if data[i] != expected[i] { return false; }
    }
    true
}

fn valid_multiplication( data : &[u8] ) -> Option<(i32, usize)> {
    // parse 'mul('
    let mut idx = 0;
    if idx >= data.len()  || data[idx] != b'm'{
        return None;
    }
    idx += 1;
    if idx >= data.len()  || data[idx] != b'u'{
        return None;
    }
    idx += 1;
    if idx >= data.len()  || data[idx] != b'l'{
        return None;
    }
    idx += 1;
    if idx >= data.len()  || data[idx] != b'('{
        return None;
    }
    // parse first number
    idx += 1;
    if idx >= data.len()  || data[idx]  < b'0' || data[idx] > b'9' {
        return None;
    }
    let mut lhs = ( data[idx] - b'0' ) as i32;
    loop {
        idx += 1;
        if idx >= data.len() || ( (data[idx]  < b'0' || data[idx] > b'9') && data[idx] != b',' ) {
            return None;
        }
        if data[idx] == b',' {
            break;
        }
        lhs *= 10;
        lhs += ( data[idx] - b'0' ) as i32;
    }
    // parse second number
    idx += 1;
    if idx >= data.len()  || data[idx]  < b'0' || data[idx] > b'9' {
        return None;
    }
    let mut rhs = ( data[idx] - b'0' ) as i32;
    loop {
        idx += 1;
        if idx >= data.len() || ( (data[idx]  < b'0' || data[idx] > b'9') && data[idx] != b')' ) {
            return None;
        }
        if data[idx] == b')' {
            break;
        }
        rhs *= 10;
        rhs += ( data[idx] - b'0' ) as i32;
    }
    // return the result
    Some( ( lhs * rhs, idx ) )
}

fn main() {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> i32 {
        let mut ans = 0;
        let bytes = reader.bytes().filter_map( |r| r.ok() ).collect::<Vec<u8>>();
        for mut i in 0..bytes.len() {
            if let Some( ( result, idx ) ) = valid_multiplication( &bytes[i..]) {
                ans += result;
                i += idx;
            }
        }
        ans
    }

    fn part2<R: BufRead>(reader: R) -> i32 {
        let mut ans = 0;
        let bytes = reader.bytes().filter_map( |r| r.ok() ).collect::<Vec<u8>>();
        let mut ignore = false;
        for mut i in 0..bytes.len() {
            if ignore {
                if is_do( &bytes[i..]) {
                    ignore = false;
                    i += 3;
                }
            }
            else {
                if is_dont( &bytes[i..]) {
                    ignore = true;
                    i += 6;
                }
                else if let Some( ( result, idx ) ) = valid_multiplication( &bytes[i..]) {
                    ans += result;
                    i += idx;
                }
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
