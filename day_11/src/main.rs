const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    calculate_distance(1)
}

fn solve_part_2() -> usize {
    calculate_distance(999999)
}

fn calculate_distance(expansion: usize) -> usize {
    let picture: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let mut row_blanks: Vec<usize> = Vec::new();
    let mut column_blanks: Vec<usize> = Vec::new();

    let mut blanks = 0;
    for l in picture.iter() {
        if !l.contains(&'#') {
            blanks += 1;
        }
        row_blanks.push(blanks);
    }
    
    blanks = 0;
    for r in 0..picture[0].len() {
        if picture.iter().map(|l| l[r]).filter(|c| *c == '#').count() == 0 {
            blanks += 1;
        }
        column_blanks.push(blanks);
    }

    let mut sum = 0;
    for i in 0..picture.len() {
        let row_inc = row_blanks[i];
        for j in 0..picture[0].len() {
            if picture[i][j] != '#' {
                continue;
            }
            let col_inc = column_blanks[j];
            for m in 0..picture.len() {
                let row_inc_o = row_blanks[m];
                for n in 0..picture[0].len() {
                    if (m == i && n == j) || picture[m][n] != '#' {
                        continue;
                    }
                    let col_inc_o = column_blanks[n];

                    sum += m.abs_diff(i) + row_inc.abs_diff(row_inc_o)*expansion + n.abs_diff(j) + col_inc.abs_diff(col_inc_o)*expansion;
                }
            }
        }
    }

    sum / 2
}

fn main() {
    println!("Day 11: Cosmic Expansion");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
