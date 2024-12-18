use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

struct Solution {
    ordering : HashMap<i32, HashSet<i32>>,
    updates : Vec<Vec<i32>>
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut ordering = HashMap::new();
        let mut updates = Vec::new();

        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            let mut parse_ordering = true;
            for line in reader.lines().filter_map(|line| line.ok()) {
                if line.is_empty() {
                    parse_ordering = false;
                    continue;
                }

                if parse_ordering {
                    let ord = line.split('|').collect::<Vec<&str>>();
                    if ord.len() != 2 { continue; }
                    if let ( Ok(lhs), Ok(rhs) ) = ( ord[0].parse::<i32>(), ord[1].parse::<i32>() ) {
                        ordering.entry(lhs).or_insert(HashSet::new()).insert(rhs);
                    }
                } else {
                    let update = line.split(',').filter_map(|s|s.parse::<i32>().ok()).collect::<Vec<_>>();
                    updates.push(update);
                }
            }
        }
        Solution {
            ordering,
            updates
        }
    }
    
    fn is_correct(&self, update : &[i32] ) -> bool {
        for i in 1..update.len() {
            let key = update[i];
            if !self.ordering.contains_key( &key ) {
                continue;
            }
            for j in 0.. i {
                if let Some(ord) = self.ordering.get( &key ) {
                    if ord.contains( &update[j] ) {
                        return false;
                    }
                }
            }
        }
        true
    }
    
    fn order_correctly(&self, update : &[i32] ) -> Vec<i32> {
        let update_set : HashSet<i32> = HashSet::from_iter(update.to_vec());
        // narrow it down only to the numbers of interest
        let mut ord = HashMap::new();
        for nb in update.iter() {
            let mut dependencies = HashSet::new();
            if let Some( full_dependencies ) = self.ordering.get( &nb ) {
                for dep in full_dependencies {
                    if update_set.contains( dep ) {
                        dependencies.insert(*dep);
                    }
                }
            }
            ord.insert(*nb, dependencies);
        }
        let mut result = Vec::new();
        while !ord.is_empty() {
            // get those that have no dependencies
            let mut empty = Vec::new();
            for tpl in ord.iter() {
                if tpl.1.is_empty() {
                    empty.push(*tpl.0);
                }
            }
            // remove those that have no dependencies and add them to result
            for nb in empty.iter() {
                ord.remove( nb );
                result.push(*nb);
            }
            // remove those who have been added to result
            for tpl in ord.iter_mut() {
                for nb in empty.iter() {
                    tpl.1.remove( nb );
                }
            }
        }

        result
    }

    fn solve1(&self) -> i32 {
        let mut result = 0;
        for update in self.updates.iter() {
            if self.is_correct(update) {
                result += update[update.len() / 2];
            }
        }
        result
    }

    fn solve2(&self) -> i32 {
        let mut result = 0;
        for update in self.updates.iter() {
            if !self.is_correct(update) {
                let fixed = self.order_correctly(update);
                result += fixed[update.len() / 2];
            }
        }
        result
    }
}


fn main() {
    let solution = Solution::new(INPUT_FILE);
    let result1 = solution.solve1();
    println!("result1 = {}", result1);
    let result2 = solution.solve2();
    println!("result2 = {}", result2);
}
