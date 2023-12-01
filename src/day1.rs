use crate::utils::read_lines;

const DIGITS_WORD: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS_CHAR: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn numbera(value: &str) -> u32 {
    value
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap()
        * 10
        + value
            .chars()
            .rfind(|c| c.is_ascii_digit())
            .unwrap_or('0')
            .to_digit(10)
            .unwrap()
}

fn numberb(s: &str) -> u32 {
    let mut digit_pos: Vec<(usize, usize)> = DIGITS_WORD
        .iter()
        .enumerate()
        .clone()
        .flat_map(|(idx, word)| s.match_indices(word).map(move |(pos, _)| (pos, idx + 1)))
        .collect();
    digit_pos.extend(
        DIGITS_CHAR
            .iter()
            .enumerate()
            .clone()
            .flat_map(|(idx, word)| s.match_indices(word).map(move |(pos, _)| (pos, idx + 1))),
    );
    let first_digit = digit_pos.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().1;
    let last_digit = digit_pos.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1;

    (first_digit * 10 + last_digit) as u32
}

pub fn run() {
    let lines = read_lines("in/day1.in").unwrap();

    let parsed_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let calibration_values_a: Vec<u32> = parsed_lines.iter().map(|s| numbera(s)).collect();
    let calibration_values_b: Vec<u32> = parsed_lines.iter().map(|s| numberb(s)).collect();

    println!("Day 1.a: {}", calibration_values_a.iter().sum::<u32>());
    println!("Day 1.b: {}", calibration_values_b.iter().sum::<u32>());
}
