use regex::Regex;
use std::collections::HashSet;

use crate::utils::read_lines;

type Loc = (i32, i32);

struct Schematic {
    matrix: Vec<String>,
}

#[derive(Clone)]
struct Part {
    pub number: u32,
    pub locations: HashSet<Loc>,
}

fn is_symbol(c: &char) -> bool {
    c != &'.' && !c.is_ascii_digit()
}

fn is_gear(c: &char) -> bool {
    c == &'*'
}

impl Schematic {
    fn get_char(&self, loc: &Loc) -> char {
        if loc.0 < 0
            || loc.0 >= self.matrix.len() as i32
            || loc.1 < 0
            || loc.1 >= self.matrix[0].len() as i32
        {
            '.'
        } else {
            self.matrix[loc.0 as usize]
                .chars()
                .nth(loc.1 as usize)
                .unwrap()
        }
    }

    pub fn find_digits(&self) -> Vec<Part> {
        let mut parts = vec![];
        let re = Regex::new(r"[0-9]+").unwrap();
        // Re::
        for (x, l) in self.matrix.iter().enumerate() {
            re.find_iter(l).for_each(|m| {
                parts.push(Part {
                    number: m.as_str().parse::<u32>().unwrap(),
                    locations: (m.start()..m.end()).map(|y| (x as i32, y as i32)).collect(),
                })
            })
        }
        parts
    }

    pub fn find_gears(&self) -> Vec<Loc> {
        let mut gears = vec![];
        for (x, l) in self.matrix.iter().enumerate() {
            for (y, c) in l.chars().enumerate() {
                if is_gear(&c) {
                    gears.push((x as i32, y as i32));
                }
            }
        }
        gears
    }
}

fn neighbors(loc: &Loc) -> Vec<Loc> {
    vec![
        (loc.0 - 1, loc.1 - 1),
        (loc.0 - 1, loc.1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
        (loc.0 + 1, loc.1 - 1),
        (loc.0 + 1, loc.1),
        (loc.0 + 1, loc.1 + 1),
    ]
}

fn part1(engine: &Schematic) {
    let candidate_parts = engine.find_digits();
    let valid_parts = candidate_parts.iter().filter(|part| {
        part.locations
            .iter()
            .flat_map(neighbors)
            .any(|loc| is_symbol(&engine.get_char(&loc)))
    });
    println!("Day 1.a: {}", valid_parts.map(|x| x.number).sum::<u32>());
}

fn part2(engine: &Schematic) {
    let candidate_parts = engine.find_digits();
    let valid_parts: Vec<Part> = candidate_parts
        .into_iter()
        .filter(|part| {
            part.locations
                .iter()
                .flat_map(neighbors)
                .any(|loc| is_symbol(&engine.get_char(&loc)))
        })
        .collect();

    let gears = engine.find_gears();
    let res: u32 = gears
        .iter()
        .map(|gear| {
            let n: HashSet<Loc> = HashSet::from_iter(neighbors(gear).iter().cloned());
            let parts: Vec<Part> = valid_parts
                .iter()
                .cloned()
                .filter(|part| n.intersection(&part.locations).next().is_some())
                .collect();
            if parts.len() == 2 {
                parts[0].number * parts[1].number
            } else {
                0
            }
        })
        .sum();
    println!("Day 1.b: {}", res);
}

pub fn run() {
    let lines = read_lines("in/day3.in").unwrap();
    let matrix: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let engine = Schematic { matrix };
    part1(&engine);
    part2(&engine);
}
