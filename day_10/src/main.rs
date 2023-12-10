use std::collections::HashMap;

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
    println!("The position of the start is: {:?} with directions: {:?}", start, directions);

    let mut visited = HashMap::new();
    let mut prev = start;
    let mut current = directions.0;
    let mut distance = 1;
    while current != start {
        visited.insert(current, distance);
        let tmp = current;
        current = get_next(&tiles, prev, current);
        prev = tmp;
        distance += 1;
    }

    current = directions.1;
    distance = 1;
    prev = start;
    while distance != *visited.get(&current).unwrap_or(&0) {
        let tmp = current;
        current = get_next(&tiles, prev, current);
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
    println!("The position of the start is: {:?} with directions: {:?}", start, directions);

    let mut prev = start;
    let mut current = directions.0;
    while current != start {
        let tmp = current;
        current = get_next(&tiles, prev, current);
        prev = tmp;
        tiles[tmp.0][tmp.1] = 'V';
    }

    let string = tiles.iter().map(|l| l.iter().map(|c| c.to_string()).reduce(|acc, c| {
        let mut str = acc.clone();
        str.push_str(&c);
        str
    })).map(|l| l.unwrap()).reduce(|acc, l| {
        let mut str = acc.clone();
        str.push('\n');
        str.push_str(&l);
        str
    }).unwrap();
    println!("{}", string);
    
    0
}

fn get_directions(tiles: &Vec<Vec<char>>, position: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let position_val = tiles[position.0][position.1];
    let north = if position.0 > 0 {tiles[position.0-1][position.1]} else {position_val};
    let east = if position.1 < tiles[0].len() - 1 {tiles[position.0][position.1+1]} else {position_val};
    let south = if position.0 < tiles.len() - 1 {tiles[position.0+1][position.1]} else {position_val};
    let west = if position.1 > 0 {tiles[position.0][position.1-1]} else {position_val};

    let dir_1;
    let mut dir_2 = position;
    if north != position_val && (north == '7' || north == 'F' || north == '|') {
        dir_1 = (position.0-1, position.1);
    } else if east != position_val && (east == '7' || east == 'J' || east == '-') {
        dir_1 = (position.0, position.1+1);
    } else {
        dir_1 = (position.0+1, position.1);
        dir_2 = (position.0, position.1-1);
    }

    if dir_2 == position && west != position_val && (west == 'F' || west == 'L' || west == '-') {
        dir_2 = (position.0, position.1-1);
    } else if dir_2 == position && south != position_val && (south == 'L' || south == 'J' || south == '|') {
        dir_2 = (position.0+1, position.1);
    } else if dir_2 == position {
        dir_2 = (position.0, position.1+1);
    }

    (dir_1, dir_2)
}

fn get_next(tiles: &Vec<Vec<char>>, prev: (usize, usize), curr: (usize, usize)) -> (usize, usize) {
    let curr_val = tiles[curr.0][curr.1];
    match curr_val {
        '-' => if prev.1 < curr.1 {
            (curr.0, curr.1+1)
        } else {
            (curr.0, curr.1-1)
        },
        '|' => if prev.0 < curr.0 {
            (curr.0+1, curr.1)
        } else {
            (curr.0-1, curr.1)
        },
        'J' => if prev.1 < curr.1 {
            println!("left");
            (curr.0-1, curr.1)
        } else {
            println!("right");
            (curr.0, curr.1-1)
        },
        'F' => if prev.1 > curr.1 {
            println!("left");
            (curr.0+1, curr.1)
        } else {
            println!("right");
            (curr.0, curr.1+1)
        },
        'L' => if prev.0 < curr.0 {
            println!("left");
            (curr.0, curr.1+1)
        } else {
            println!("right");
            (curr.0-1, curr.1)
        },
        '7' => if prev.1 < curr.1 {
            println!("right");
            (curr.0+1, curr.1)
        } else {
            println!("left");
            (curr.0, curr.1-1)
        },
        _ => panic!("We got off the loop")
    }
}

fn main() {
    println!("Day 10: Pipe Maze");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
