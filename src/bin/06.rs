use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3
}
struct Solution {
    map : Vec<Vec<bool>>,
    pos : (usize, usize),
    direction : Direction,
}

impl Solution {
    fn new(filename: &str) -> Solution {
        let mut map = Vec::new();
        let mut pos = (0, 0);
        if let Ok( file ) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(|line| line.ok()) {
                let mut row = Vec::new();
                for ch in line.chars() {
                    if ch == '^' { pos = (map.len(), row.len()); }
                    row.push(ch == '#');
                }
                map.push(row);
            }
        }

        Solution {
            map,
            pos,
            direction: Direction::Up
        }
    }

    fn solve1(&self) -> usize {
        let mut visited = HashSet::new();
        let mut pos = self.pos;
        let mut direction = self.direction;
        visited.insert(pos);
        loop {
            match direction
            {
                Direction::Up =>
                    {
                        if pos.0 == 0 {
                            return visited.len();
                        }
                        let next = ( pos.0 - 1, pos.1 );
                        if self.map[next.0][next.1] {
                            direction = Direction::Right;
                        } else {
                            pos = next;
                            visited.insert(pos);
                        }
                    }
                Direction::Right =>
                    {
                        if pos.1 == self.map[pos.0].len() - 1 {
                            return visited.len();
                        }
                        let next = ( pos.0, pos.1 + 1 );
                        if self.map[next.0][next.1] {
                            direction = Direction::Down;
                        } else {
                            pos = next;
                            visited.insert(pos);
                        }
                    }
                Direction::Down => {
                    if pos.0 == self.map.len() - 1 {
                        return visited.len();
                    }
                    let next = ( pos.0 + 1, pos.1 );
                    if self.map[next.0][next.1] {
                        direction = Direction::Left;
                    } else {
                        pos = next;
                        visited.insert(pos);
                    }
                }
                Direction::Left => {
                    if pos.1 == 0 {
                        return visited.len();
                    }
                    let next = ( pos.0, pos.1 - 1 );
                    if self.map[next.0][next.1] {
                        direction = Direction::Up;
                    } else {
                        pos = next;
                        visited.insert(pos);
                    }
                }
            }
        }
    }

    fn has_cycle(&self) -> bool {

        let mut visited = HashSet::new();
        let mut pos = self.pos;
        let mut direction = self.direction;
        visited.insert((pos.0, pos.1, direction as usize) );
        loop {
            match direction
            {
                Direction::Up =>
                    {
                        if pos.0 == 0 {
                            return false;
                        }
                        let next = ( pos.0 - 1, pos.1 );
                        if self.map[next.0][next.1] {
                            direction = Direction::Right;
                        } else {
                            pos = next;
                            if !visited.insert( (pos.0, pos.1, direction as usize ) ) {
                                return true;
                            }
                        }
                    }
                Direction::Right =>
                    {
                        if pos.1 == self.map[pos.0].len() - 1 {
                            return false;
                        }
                        let next = ( pos.0, pos.1 + 1 );
                        if self.map[next.0][next.1] {
                            direction = Direction::Down;
                        } else {
                            pos = next;
                            if !visited.insert( (pos.0, pos.1, direction as usize ) ) {
                                return true;
                            }
                        }
                    }
                Direction::Down => {
                    if pos.0 == self.map.len() - 1 {
                        return false;
                    }
                    let next = ( pos.0 + 1, pos.1 );
                    if self.map[next.0][next.1] {
                        direction = Direction::Left;
                    } else {
                        pos = next;
                        if !visited.insert( (pos.0, pos.1, direction as usize ) ) {
                            return true;
                        }
                    }
                }
                Direction::Left => {
                    if pos.1 == 0 {
                        return false;
                    }
                    let next = ( pos.0, pos.1 - 1 );
                    if self.map[next.0][next.1] {
                        direction = Direction::Up;
                    } else {
                        pos = next;
                        if !visited.insert( (pos.0, pos.1, direction as usize ) ) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    fn solve2(&mut self) -> usize {

        let mut result = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] || (i, j) == self.pos { continue; } // it is already an obstacle
                self.map[i][j] = true;
                if self.has_cycle() { result += 1; }
                self.map[i][j] = false;
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
