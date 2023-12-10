use std::collections::{HashMap,HashSet};

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let tiles: Vec<Vec<_>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    
    let mut start = (0, 0);
    for (i, l) in tiles.iter().enumerate() {
        match l.iter().position(|c| *c == 'S') {
            Some(p) => {
                start = (i, p);
                break;
            },
            _ => ()
        }
    }

    let directions = get_directions(&tiles, start);
    //println!("The position of the start is: {:?} with directions: {:?}", start, directions);

    let mut visited = HashMap::new();
    let mut prev = start;
    let mut current = directions.0;
    let mut distance = 1;
    while current != start {
        visited.insert(current, distance);
        let tmp = current;
        (current, _) = get_next(&tiles, prev, current);
        prev = tmp;
        distance += 1;
    }

    current = directions.1;
    distance = 1;
    prev = start;
    while distance != *visited.get(&current).unwrap_or(&0) {
        let tmp = current;
        (current, _) = get_next(&tiles, prev, current);
        prev = tmp;
        distance += 1;
    }

    distance
}

fn solve_part_2() -> usize {
    let mut tiles: Vec<Vec<_>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    
    let mut start = (0, 0);
    for (i, l) in tiles.iter().enumerate() {
        match l.iter().position(|c| *c == 'S') {
            Some(p) => {
                start = (i, p);
                break;
            },
            _ => ()
        }
    }

    let directions = get_directions(&tiles, start);
    //println!("The position of the start is: {:?} with directions: {:?}", start, directions);

    let mut visited = HashSet::new();
    visited.insert(start);
    let mut prev = start;
    let mut current = directions.0;
    while current != start {
        visited.insert(current);
        let tmp = current;
        (current, _) = get_next(&tiles, prev, current);
        prev = tmp;
    }

    prev = start;
    current = directions.0;
    while current != start {
        visited.insert(current);
        let tmp = current;
        let dir;
        (current, dir) = get_next(&tiles, prev, current);
        prev = tmp;

        let mut current_inside = get_next_left_of_dir(prev, dir);
        while !visited.contains(&current_inside) {
            //println!("{}", to_string(&tiles));
            //println!("");
            tiles[current_inside.0][current_inside.1] = 'I';
            current_inside = get_next_left_of_dir(current_inside, dir);
        }
        current_inside = get_next_left_of_dir(current, dir);
        while !visited.contains(&current_inside) {
            //println!("{}", to_string(&tiles));
            //println!("");
            tiles[current_inside.0][current_inside.1] = 'I';
            current_inside = get_next_left_of_dir(current_inside, dir);
        }
    }

    //println!("{}", to_string(&tiles));


    tiles.iter().flatten().filter(|c| **c == 'I').count()
}

fn to_string(tiles: &Vec<Vec<char>>) -> String {
    tiles.iter().map(|l| l.iter().map(|c| c.to_string()).reduce(|acc, c| {
        let mut str = acc.clone();
        str.push_str(&c);
        str
    })).map(|l| l.unwrap()).reduce(|acc, l| {
        let mut str = acc.clone();
        str.push('\n');
        str.push_str(&l);
        str
    }).unwrap()
}

fn get_directions(tiles: &Vec<Vec<char>>, position: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let north = tiles[position.0-1][position.1];
    let east = tiles[position.0][position.1+1];
    let south = tiles[position.0+1][position.1];
    let west = tiles[position.0][position.1-1];

    let dir_1;
    let mut dir_2 = position;
    if north == '7' || north == 'F' || north == '|' {
        dir_1 = (position.0-1, position.1);
    } else if east == '7' || east == 'J' || east == '-' {
        dir_1 = (position.0, position.1+1);
    } else {
        dir_1 = (position.0+1, position.1);
        dir_2 = (position.0, position.1-1);
    }

    if dir_2 == position && (west == 'F' || west == 'L' || west == '-') {
        dir_2 = (position.0, position.1-1);
    } else if dir_2 == position && (south == 'L' || south == 'J' || south == '|') {
        dir_2 = (position.0+1, position.1);
    } else if dir_2 == position {
        dir_2 = (position.0, position.1+1);
    }

    (dir_1, dir_2)
}

fn get_next(tiles: &Vec<Vec<char>>, prev: (usize, usize), curr: (usize, usize)) -> ((usize, usize), usize) {
    let curr_val = tiles[curr.0][curr.1];
    match curr_val {
        '-' => if prev.1 < curr.1 {
            ((curr.0, curr.1+1), 0)
        } else {
            ((curr.0, curr.1-1), 2)
        },
        '|' => if prev.0 < curr.0 {
            ((curr.0+1, curr.1), 1)
        } else {
            ((curr.0-1, curr.1), 3)
        },
        'J' => if prev.1 < curr.1 {
            //println!("left");
            ((curr.0-1, curr.1), 3)
        } else {
            //println!("right");
            ((curr.0, curr.1-1), 2)
        },
        'F' => if prev.1 > curr.1 {
            //println!("left");
            ((curr.0+1, curr.1), 1)
        } else {
            //println!("right");
            ((curr.0, curr.1+1), 0)
        },
        'L' => if prev.0 < curr.0 {
            //println!("left");
            ((curr.0, curr.1+1), 0)
        } else {
            //println!("right");
            ((curr.0-1, curr.1), 3)
        },
        '7' => if prev.1 < curr.1 {
            //println!("right");
            ((curr.0+1, curr.1), 1)
        } else {
            //println!("left");
            ((curr.0, curr.1-1), 2)
        },
        _ => panic!("We got off the loop")
    }
}

fn get_next_left_of_dir(current: (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
            1 => (current.0, current.1+1),
            2=> (current.0+1, current.1),
            3 => (current.0, current.1-1),
            0 => (current.0-1, current.1),
            _ => panic!("We went diagonal or something???")
        }
}

fn main() {
    println!("Day 10: Pipe Maze");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
