use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let limits: HashMap<&str, usize> = HashMap::from([
                    ("red", 12),
                    ("green", 13),
                    ("blue", 14)]);
    let mut sum = 0;
    for (i, l) in INPUT.lines().enumerate() {
        if l.split(": ").collect::<Vec<&str>>()[1].split("; ").all(|set| {
            set.split(", ").all(|colour| {
                let sp: Vec<&str> = colour.split_whitespace().collect();
                let amount: usize = sp[0].parse().unwrap();
                if amount <= *limits.get(sp[1]).unwrap() {
                    true
                } else {
                    false
                }
            })
        }) {
            sum += i+1;
        }
    }

    sum
}

fn solve_part_2() -> usize {
    INPUT.lines().map(|l| {
        let game = l.split(": ").collect::<Vec<&str>>()[1];
        let mut limits: HashMap<&str, usize> = HashMap::from([
                            ("red", 0),
                            ("green", 0),
                            ("blue", 0)]);
        for set in game.split("; ") {
            for colour in set.split(", ") {
                let sp: Vec<&str> = colour.split_whitespace().collect();
                let amount: usize = sp[0].parse().unwrap();
                if amount > *limits.get(sp[1]).unwrap() {
                    limits.insert(sp[1], amount);
                }
            }
        }
        limits.values().product::<usize>()
    }).sum()
}


fn main() {
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
