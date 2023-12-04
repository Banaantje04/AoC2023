use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            let card: Vec<_> = l.split(":").nth(1).unwrap().split('|').collect();
            let winning: Vec<_> = card[0]
                .split_whitespace()
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect();
            let wins = card[1]
                .split_whitespace()
                .map(|n| n.trim().parse::<usize>().unwrap())
                .filter(|n| winning.contains(&n))
                .count();

            if wins == 1 {
                1
            } else if wins > 1 {
                usize::pow(2, (wins - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum()
}

fn solve_part_2() -> usize {
    let mut hm = HashMap::new();

    for (i, l) in INPUT.lines().enumerate() {
        let card: Vec<_> = l.split(":").nth(1).unwrap().split('|').collect();
        let winning: Vec<_> = card[0]
            .split_whitespace()
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect();
        let wins = card[1]
            .split_whitespace()
            .map(|n| n.trim().parse::<usize>().unwrap())
            .filter(|n| winning.contains(&n))
            .count();

        hm.insert(i, wins);
    }

    //println!("Map of wins: {:#?}", hm);

    let mut cards: HashMap<usize, usize> = HashMap::new();
    let mut amount = 0;
    for i in 0..INPUT.lines().count() {
        cards.insert(i, 1);
    }

    //println!("Starting queue: {:?}", queue);

    for i in 0..INPUT.lines().count() {
        let card_count = *(cards.get(&i).unwrap());
        amount += card_count;

        for j in i + 1..i + hm.get(&i).unwrap() + 1 {
            cards.insert(j, cards.get(&j).unwrap() + card_count);
        }
    }

    amount
}

fn main() {
    println!("Day 4: Scratchcards");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
