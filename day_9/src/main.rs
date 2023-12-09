const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> isize {
    INPUT
        .lines()
        .map(|l| {
            let mut sequences: Vec<Vec<isize>> =
                vec![l.split_whitespace().map(|n| n.parse().unwrap()).collect()];
            while !sequences.last().unwrap().iter().all(|s| *s == 0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|n| n[1] - n[0])
                        .collect(),
                );
            }

            for i in (1..sequences.len()).rev() {
                let new = sequences[i - 1].last().unwrap() + sequences[i].last().unwrap();
                sequences[i - 1].push(new);
            }
            //println!("The sequences are: {:?}", sequences);
            *(sequences.first().unwrap().last().unwrap())
        })
        .sum()
}

fn solve_part_2() -> isize {
    INPUT
        .lines()
        .map(|l| {
            let mut sequences: Vec<Vec<isize>> =
                vec![l.split_whitespace().map(|n| n.parse().unwrap()).collect()];
            while !sequences.last().unwrap().iter().all(|s| *s == 0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|n| n[1] - n[0])
                        .collect(),
                );
            }

            for i in (1..sequences.len()).rev() {
                let new = sequences[i - 1].first().unwrap() - sequences[i].first().unwrap();
                sequences[i - 1].splice(0..0, vec![new]);
            }
            //println!("The sequences are: {:?}", sequences);
            *(sequences.first().unwrap().first().unwrap())
        })
        .sum()
}

fn main() {
    println!("Day 9: Mirage Maintenance");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
