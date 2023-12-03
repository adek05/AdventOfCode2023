use crate::utils::read_lines;

#[derive(Debug)]
struct Hand {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

type Game = Vec<Hand>;

fn parse_hand(s: &str) -> Hand {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    s.split(',').for_each(
        |x| {
            let mut it = x.trim().split(' ');
            let number_of_cubes = it.next().unwrap().parse::<i64>().unwrap();
            let color = it.next().unwrap();
            match color {
                "red" => {r = number_of_cubes}
                "green" => {g = number_of_cubes}
                "blue" => {b = number_of_cubes}
                _ => panic!("Parse error: unknonw color {}", color)
            }
        }
    );

    Hand {
        red: r,
        green: g,
        blue: b,
    }

}

fn parse(s: &str) -> Vec<Hand> {
    let mut it = s.split(':');
    let _game = it.next().unwrap();
    let hand_strings = it.next().unwrap();
    let hands: Vec<Hand> = hand_strings.split(';').map(parse_hand).collect();
    hands
}

fn part1(games: &[Game]) {
    const MAX_RED: i64 = 12;
    const MAX_GREEN: i64 = 13;
    const MAX_BLUE: i64 = 14;
    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    let res = games.iter().enumerate().filter(
        |(_game_id, hands)| {
            hands.iter().all(|hand| {
                hand.red <= MAX_RED &&
                hand.green <= MAX_GREEN &&
                hand.blue <= MAX_BLUE
            })
        }
    ).map(|(game_id, _)| game_id+1)
    .sum::<usize>();

    println!("Day 1.a: {}", res);
}

fn part2(games: &[Game]) {
    let res: i64 = games.iter().map(
        |hands| {
            let mut max_r: i64 = 0;
            let mut max_g: i64 = 0;
            let mut max_b: i64 = 0;
            hands.iter().for_each(|hand| {
                max_r = max_r.max(hand.red);
                max_g = max_g.max(hand.green);
                max_b = max_b.max(hand.blue);
            });
            max_r * max_g * max_b
        }
    ).sum();
    println!("Day 1.b: {}", res);
}

pub fn run() {
    let lines = read_lines("in/day2.in").unwrap();
    let l: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let games: Vec<Game> = l.iter().map(|line| parse(line)).collect();

    part1(&games);
    part2(&games);
}
