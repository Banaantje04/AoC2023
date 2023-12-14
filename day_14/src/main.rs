use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let mut tiles: Vec<Vec<_>> = INPUT.lines().map(|l| l.chars().collect()).collect();

    for i in 0..tiles.len() {
        for j in 0..tiles[0].len() {
            if i > 0 && tiles[i][j] == 'O' {
                let mut k = i as isize - 1;
                while k >= 0 && tiles[k as usize][j] == '.' {
                    k -= 1;
                }
                if i as isize - k > 1 {
                    tiles[(k + 1) as usize][j] = 'O';
                    tiles[i][j] = '.';
                }
            }
        }
    }

    let mut sum = 0;
    for (i, l) in (1..=tiles.len()).rev().zip(tiles.iter()) {
        sum += i * l.iter().filter(|&&c| c == 'O').count();
    }

    sum
}

fn solve_part_2() -> usize {
    let mut tiles: Vec<Vec<_>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let mut permutations = HashMap::new();
    let mut loop_num = (0,0);

    for n in 0..1000000000 {
        for i in 0..tiles.len() {
            for j in 0..tiles[0].len() {
                if i > 0 && tiles[i][j] == 'O' {
                    let mut k = i as isize - 1;
                    while k >= 0 && tiles[k as usize][j] == '.' {
                        k -= 1;
                    }
                    if i as isize - k > 1 {
                        tiles[(k + 1) as usize][j] = 'O';
                        tiles[i][j] = '.';
                    }
                }
            }
        }

        for i in 0..tiles.len() {
            for j in 0..tiles[0].len() {
                if j > 0 && tiles[i][j] == 'O' {
                    let mut k = j as isize - 1;
                    while k >= 0 && tiles[i][k as usize] == '.' {
                        k -= 1;
                    }
                    if j as isize - k > 1 {
                        tiles[i][(k + 1) as usize] = 'O';
                        tiles[i][j] = '.';
                    }
                }
            }
        }

        for i in (0..tiles.len()).rev() {
            for j in (0..tiles[0].len()).rev() {
                if i < tiles.len() - 1 && tiles[i][j] == 'O' {
                    let mut k = i + 1;
                    while k < tiles.len() && tiles[k][j] == '.' {
                        k += 1;
                    }
                    if k - i > 1 {
                        tiles[k - 1][j] = 'O';
                        tiles[i][j] = '.';
                    }
                }
            }
        }

        for i in (0..tiles.len()).rev() {
            for j in (0..tiles[0].len()).rev() {
                if j < tiles[0].len() - 1 && tiles[i][j] == 'O' {
                    let mut k = j + 1;
                    while k < tiles[0].len() && tiles[i][k] == '.' {
                        k += 1;
                    }
                    if k - j > 1 {
                        tiles[i][k - 1] = 'O';
                        tiles[i][j] = '.';
                    }
                }
            }
        }

        if permutations.contains_key(&tiles) {
            loop_num = (n, *permutations.get(&tiles).unwrap());
            break;
        } else {
            permutations.insert(tiles.clone(), n);
        }
        
    }

    let index_same_as_biiig = loop_num.1 + (999999999 - loop_num.1) % (loop_num.0 - loop_num.1);
    tiles = permutations.iter().find_map(|(k, &v)| if v == index_same_as_biiig { Some(k.clone()) } else { None }).unwrap();

    let mut sum = 0;
    for (i, l) in (1..=tiles.len()).rev().zip(tiles.iter()) {
        sum += i * l.iter().filter(|c| **c == 'O').count();
    }

    sum
}

fn main() {
    println!("Day 14: Parabolic Reflector Dish");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
