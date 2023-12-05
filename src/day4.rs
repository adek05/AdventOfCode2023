use crate::utils::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;

fn part1(inputs: &[(HashSet<u64>, HashSet<u64>)]) {
    let res: u64 = inputs
        .iter()
        .map(|(a, b)| {
            let size = a.intersection(b).count();
            if size == 0 {
                0
            } else {
                1 << (size - 1)
            }
        })
        .sum();
    println!("Day 1.a: {}", res);
}

fn part2(inputs: &[(HashSet<u64>, HashSet<u64>)]) {
    let card_matches: HashMap<usize, u64> =
        HashMap::from_iter(inputs.iter().enumerate().map(|(idx, (a, b))| {
            let size = a.intersection(b).count();
            (idx, size as u64)
        }));
    let mut n_cards: HashMap<usize, u64> =
        HashMap::from_iter(inputs.iter().enumerate().map(|(idx, _)| (idx, 1)));
    let mut res = 0;
    for (idx, _) in inputs.iter().enumerate() {
        let matches = card_matches[&idx];
        res += n_cards[&idx];
        for c in (idx + 1)..(idx + 1 + matches as usize) {
            if n_cards.contains_key(&c) {
                *n_cards.get_mut(&c).unwrap() += n_cards[&idx];
            }
        }
    }
    println!("Day 1.b: {}", res);
}

pub fn run() {
    let lines = read_lines("in/day4.in").unwrap();

    let inputs: Vec<(HashSet<u64>, HashSet<u64>)> = lines.map(|l| {
        let_scan!(
                l.unwrap(); ("Card ", let _: i32, ":", [ let winning: u64 ]+: HashSet<u64>, "|", [ let cards: u64 ]+: HashSet<u64>)
        );
        (winning, cards)
    }).collect();
    part1(&inputs);
    part2(&inputs);
}
