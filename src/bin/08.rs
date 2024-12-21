use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");


struct Solution {
    dimensions: (usize, usize),
    frequencies : HashMap<char, Vec<(i64, i64)>>
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut dimensions = (0, 0);
        let mut frequencies = HashMap::new();
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|line| line.ok()) {
                for (i, c) in line.char_indices() {
                    if c.is_alphanumeric() {
                        frequencies.entry(c).or_insert(Vec::new()).push((dimensions.0 as i64, i as i64));
                    }
                }
                dimensions.0 += 1;
                if dimensions.1 == 0 { dimensions.1 += line.len(); }
            }
        }

        Solution {
            dimensions,
            frequencies
        }
    }

    fn is_on_map( pos : (i64, i64), dimensions : (i64, i64 ) ) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < dimensions.0 && pos.1 < dimensions.1
    }

    fn calculate_antifreq( ant1 : (i64, i64), ant2 : (i64, i64), dimensions: (i64, i64) ) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        let xdist = (ant1.0 - ant2.0).abs();
        let ydist = (ant1.1 - ant2.1).abs();
        let af1 = (
            if ant1.0 < ant2.0 { ant1.0 - xdist } else { ant1.0 + xdist },
            if ant1.1 < ant2.1 { ant1.1 - ydist } else { ant1.1 + ydist }
        );
        if Self::is_on_map(af1, dimensions) { result.push(af1); }
        let af2 = (
            if ant2.0 < ant1.0 { ant2.0 - xdist } else { ant2.0 + xdist },
            if ant2.1 < ant1.1 { ant2.1 - ydist } else { ant2.1 + ydist }
        );
        if Self::is_on_map(af2, dimensions) { result.push(af2); }
        result
    }

    fn calculate_antifreq_with_harmonic( ant1 : (i64, i64), ant2 : (i64, i64), dimensions: (i64, i64) ) -> Vec<(i64, i64)> {
        let mut result = Vec::new();
        let xdist = (ant1.0 - ant2.0).abs();
        let ydist = (ant1.1 - ant2.1).abs();

        let mut af1 = ant1;
        let xmod = if ant1.0 < ant2.0 { -xdist } else { xdist };
        let ymod = if ant1.1 < ant2.1 { -ydist } else { ydist };
        while Self::is_on_map( af1, dimensions ) {
            result.push(af1);
            af1 = (af1.0 + xmod, af1.1 + ymod);
        }

        let mut af2 = ant2;
        let xmod = -xmod;
        let ymod = -ymod;
        while Self::is_on_map( af2, dimensions ) {
            result.push(af2);
            af2 = (af2.0 + xmod, af2.1 + ymod);
        }
        result
    }

    fn solve1(&mut self) -> usize {
        let mut antifreq = HashSet::new();
        for antenas in self.frequencies.values_mut() {
            for i in 0 .. antenas.len() {
                for j in i + 1 .. antenas.len() {
                    let afs = Self::calculate_antifreq( antenas[i], antenas[j], (self.dimensions.0 as i64, self.dimensions.1 as i64) );
                    antifreq.extend(afs.into_iter());
                }
            }
        }
        antifreq.len()
    }

    fn solve2(&mut self) -> usize {
        let mut antifreq = HashSet::new();
        for antenas in self.frequencies.values_mut() {
            for i in 0 .. antenas.len() {
                for j in i + 1 .. antenas.len() {
                    let afs = Self::calculate_antifreq_with_harmonic( antenas[i], antenas[j], (self.dimensions.0 as i64, self.dimensions.1 as i64) );
                    antifreq.extend(afs.into_iter());
                }
            }
        }
        antifreq.len()
    }
}


fn main() {
    let mut solution = Solution::new(INPUT_FILE);
    let result1 = solution.solve1();
    println!("result1 = {}", result1);
    let result2 = solution.solve2();
    println!("result2 = {}", result2);
}
