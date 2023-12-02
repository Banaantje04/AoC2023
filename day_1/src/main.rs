const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let mut sum = 0;
    for l in INPUT.lines() {
        let mut first = true;
        let mut number = String::new();
        let mut second_char: char = '0';
        for c in l.chars() {
            if c.is_digit(10) && first {
                number.push(c);
                second_char = c;
                first = false;
            } else if c.is_digit(10) {
                second_char = c;
            }
        }

        number.push(second_char);
        sum += number.parse::<usize>().expect("HELP");
    }

    sum
}

fn solve_part_2() -> u32 {
    let mut sum = 0;
    for l in INPUT.lines() {
        let mut curr = l;   
        let mut first = true;
        let mut num = 0;
        let mut second_digit = 0;
        while curr.len() > 0 {
            match curr {
                _ if curr.chars().next().unwrap().is_digit(10) => {
                    let number = curr.chars().next().unwrap().to_digit(10).unwrap();
                    if first {
                        num = 10 * number;
                        first = false;
                    }
                    second_digit = number;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("one") => {
                    if first {
                        num = 10;
                        first = false;
                    }
                    second_digit = 1;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("two") => {
                    if first {
                        num = 20;
                        first = false;
                    }
                    second_digit = 2;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("three") => {
                    if first {
                        num = 30;
                        first = false;
                    }
                    second_digit = 3;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("four") => {
                    if first {
                        num = 40;
                        first = false;
                    }
                    second_digit = 4;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("five") => {
                    if first {
                        num = 50;
                        first = false;
                    }
                    second_digit = 5;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("six") => {
                    if first {
                        num = 60;
                        first = false;
                    }
                    second_digit = 6;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("seven") => {
                    if first {
                        num = 70;
                        first = false;
                    }
                    second_digit = 7;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("eight") => {
                    if first {
                        num = 80;
                        first = false;
                    }
                    second_digit = 8;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("nine") => {
                    if first {
                        num = 90;
                        first = false;
                    }
                    second_digit = 9;
                    curr = &curr[1..];
                }
                _ if curr.starts_with("zero") => {
                    if first {
                        num = 0;
                        first = false;
                    }
                    second_digit = 0;
                    curr = &curr[1..];
                }
                _ => {
                    curr = &curr[1..];
                }
            }
        }

        num += second_digit;
        sum += num;
    }

    sum
}

fn main() {
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
