const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let times = INPUT
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse().unwrap())
        .collect::<Vec<usize>>();
    let distances = INPUT
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut sum = 1;
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        let mut num = 0;
        for j in 1..*time {
            if (*time - j) * j > distance {
                num += 1;
            }
        }
        sum *= num;
    }
    sum
}

fn solve_part_2() -> usize {
    let mut time = INPUT.lines().next().unwrap().to_owned();
    time.retain(|c| c.is_digit(10));
    let time = time.parse::<usize>().unwrap();
    let mut distance = INPUT.lines().skip(1).next().unwrap().to_owned();
    distance.retain(|c| c.is_digit(10));
    let distance = distance.parse::<usize>().unwrap();

    let mut start = 0;
    for j in 1..time {
        if (time - j) * j > distance {
            start = j;
            break;
        }
    }
    let mut end = 0;
    for j in (1..time).rev() {
        if (time - j) * j > distance {
            end = j + 1;
            break;
        }
    }

    end - start
}

fn main() {
    println!("Day 6: Wait For It");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
