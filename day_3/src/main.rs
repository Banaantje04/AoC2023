const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> u32 {
    let mut sum = 0;
    let chars: Vec<_> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    for (i, l) in chars.iter().enumerate() {
        let mut num = 0;
        let mut engine_part = false;
        for (j, c) in l.iter().enumerate() {
            if !c.is_digit(10) {
                if engine_part {
                    sum += num;
                }
                num = 0;
                engine_part = false;
            } else {
                num = 10 * num + c.to_digit(10).unwrap();
                engine_part |= check_if_engine_part(&chars, i, j);
                if engine_part {
                    //println!("Engine part! line {} {:?}, character {}, number is now {}", i, l, j, num);
                }
            }
        }
        if engine_part {
            sum += num;
        }
    }

    sum
}

fn check_if_engine_part(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    (j < chars[i].len() - 1 && is_symbol(chars[i][j + 1]))
        || (j > 0 && is_symbol(chars[i][j - 1]))
        || (i < chars.len() - 1 && j < chars[i + 1].len() - 1 && is_symbol(chars[i + 1][j + 1]))
        || (i < chars.len() - 1 && is_symbol(chars[i + 1][j]))
        || (i < chars.len() - 1 && j > 0 && is_symbol(chars[i + 1][j - 1]))
        || (i > 0 && j < chars[i - 1].len() - 1 && is_symbol(chars[i - 1][j + 1]))
        || (i > 0 && is_symbol(chars[i - 1][j]))
        || (i > 0 && j > 0 && is_symbol(chars[i - 1][j - 1]))
}

fn is_symbol(c: char) -> bool {
    //println!("{c}");
    (!c.is_digit(10)) && c != '.'
}

fn solve_part_2() -> u32 {
    let mut sum = 0;
    let chars: Vec<_> = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    for (i, l) in chars.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == '*' {
                sum += get_ratio(&chars, i, j);
                //println!("sum is now: {sum}");
            }
        }
    }

    sum
}

fn get_ratio(chars: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut amount_num = 0;
    let mut ratio = 1;
    if j < chars[i].len() - 1 && chars[i][j + 1].is_digit(10) {
        amount_num += 1;
        ratio *= get_number(&chars[i], j + 1);
    }
    if j > 0 && chars[i][j - 1].is_digit(10) {
        amount_num += 1;
        ratio *= get_number(&chars[i], j - 1);
    }
    if i > 0 && chars[i - 1][j].is_digit(10) {
        amount_num += 1;
        ratio *= get_number(&chars[i - 1], j);
    } else {
        //println!("looking up");
        if j > 0 && chars[i - 1][j - 1].is_digit(10) {
            amount_num += 1;
            ratio *= get_number(&chars[i - 1], j - 1);
            //println!("found number: {}", ratio);
        }
        if j < chars[i - 1].len() - 1 && chars[i - 1][j + 1].is_digit(10) {
            amount_num += 1;
            ratio *= get_number(&chars[i - 1], j + 1);
        }
    }
    if i < chars.len() - 1 && chars[i + 1][j].is_digit(10) {
        amount_num += 1;
        ratio *= get_number(&chars[i + 1], j);
        //println!("found num {}", ratio);
    } else {
        if j > 0 && chars[i + 1][j - 1].is_digit(10) {
            amount_num += 1;
            ratio *= get_number(&chars[i + 1], j - 1);
        }
        if j < chars[i - 1].len() - 1 && chars[i + 1][j + 1].is_digit(10) {
            amount_num += 1;
            ratio *= get_number(&chars[i + 1], j + 1);
        }
    }

    if amount_num == 2 {
        ratio
    } else {
        0
    }
}

fn get_number(chars: &Vec<char>, i: usize) -> u32 {
    let mut num = 0;
    {
        let mut i = i;
        while chars[i].is_digit(10) {
            if i > 0 {
                i -= 1;
            } else {
                num = chars[i].to_digit(10).unwrap();
                break;
            }
        }
        i += 1;
        while chars[i].is_digit(10) {
            num = 10 * num + chars[i].to_digit(10).unwrap();
            if i < chars.len() - 1 {
                i += 1;
            } else {
                break;
            }
        }
    }
    //println!("got number {} on line {:?} with char {}", num, chars, i);
    num
}

fn main() {
    println!("Day 3: Gear Ratios");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
