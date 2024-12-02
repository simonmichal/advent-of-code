use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input



fn solve( txt: &str )  {

    print!("{}", txt);
}

fn main() {
    start_day(DAY);

    fn part1<R: BufRead>(reader: R) -> i32 {
        let mut c1 = BinaryHeap::new();
        let mut c2 = BinaryHeap::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                let columns = line.split("   " ).collect::<Vec<&str>>();
                assert!( columns.len() == 2 );
                if let ( Ok( v1), Ok( v2 ) ) = ( columns[0].parse::<i32>(), columns[1].parse::<i32>() ) {
                    c1.push( v1 );
                    c2.push( v2 );
                }
            }
        }
        let mut answer = 0;
        while !c1.is_empty() && !c2.is_empty() {
            if let ( Some( v1 ), Some( v2 ) ) = ( c1.pop(), c2.pop() ) {
                answer += ( v1 - v2 ).abs();
            }
        }

        answer
    }

    fn part2<R: BufRead>(reader: R) -> i32 {
        let mut c1 = Vec::new();
        let mut c2 = HashMap::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                let columns = line.split("   " ).collect::<Vec<&str>>();
                assert!( columns.len() == 2 );
                if let ( Ok( v1), Ok( v2 ) ) = ( columns[0].parse::<i32>(), columns[1].parse::<i32>() ) {
                    c1.push( v1 );
                    *c2.entry( v2 ).or_insert(0) += 1;
                }
            }
        }
        let mut answer = 0;
        for val in c1 {
            answer += val * *c2.entry( val ).or_insert(0);
        }

        answer
    }

    if let Ok( file ) = File::open(INPUT_FILE) {
        let input_file = BufReader::new( file );
        //region Part 1
        // println!("=== Part 1 ===");
        // let answer = part1(input_file);
        // assert_eq!( 1530215, answer );
        //region Part 2
        println!("\n=== Part 2 ===");
        let answer = part2(input_file);
        println!("answer = {answer}");
    }
    //endregion




    
    
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion
}
