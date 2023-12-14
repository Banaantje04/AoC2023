const INPUT: &str = include_str!("input_example.txt");

fn solve_part_1() -> usize {
    let mut sum = 0;

    'outer: for block in INPUT.split("\n\n") {
        let rows: Vec<_> = block.lines().collect();
        let chars: Vec<Vec<_>> = block.lines().map(|l| l.chars().filter(|c| *c != '\n').collect()).collect();
        let cols: Vec<String> = (0..rows[0].len()).map(|col| {
            (0..rows.len()).map(|row| chars[row][col].to_string()).reduce(|mut acc, c| {
                acc.push_str(&c);
                acc
            }).unwrap()
        }).collect();

        'rows: for (i, row) in rows.iter().enumerate() {
            if i == rows.len() - 1 || *row != rows[i+1] {
                continue;
            }

            let mut j = 0;
            while i >= j && i+1+j < rows.len() {
                if rows[i-j] != rows[i+1+j] {
                    continue 'rows;
                }
                j += 1;
            }

            sum += (i + 1) * 100;
            continue 'outer;
        }

        'cols: for (i, col) in cols.iter().enumerate() {
            if i == cols.len() - 1 || *col != cols[i+1] {
                continue;
            }

            let mut j = 0;
            while i >= j && i+1+j < cols.len() {
                if cols[i-j] != cols[i+1+j] {
                    continue 'cols;
                }
                j += 1;
            }

            sum += i + 1;
            break;
        }
    }

    sum
}

fn solve_part_2() -> usize {
    let mut sum = 0;

    'outer: for block in INPUT.split("\n\n") {
        let rows: Vec<_> = block.lines().collect();
        let chars: Vec<Vec<_>> = block.lines().map(|l| l.chars().filter(|c| *c != '\n').collect()).collect();
        let cols: Vec<String> = (0..rows[0].len()).map(|col| {
            (0..rows.len()).map(|row| chars[row][col].to_string()).reduce(|mut acc, c| {
                acc.push_str(&c);
                acc
            }).unwrap()
        }).collect();
        println!("new block");

        'rows: for (i, row) in rows.iter().enumerate() {
            if i == rows.len() - 1 {
                continue;
            }
            let mut diff = row.chars().zip(rows[i+1].chars()).filter(|c| c.0 != c.1).count();
            println!("amount of differences: {diff}");
            if diff > 1 {
                continue;
            }

            let mut j = 0;
            while i >= j && i+1+j < rows.len() {
                diff = rows[i-j].chars().zip(rows[i+1+j].chars()).filter(|c| c.0 != c.1).count();
                println!("amount of differences: {diff}");
                if diff > 1 {
                    continue 'rows;
                }
                j += 1;
            }

            sum += (i + 1) * 100;
            continue 'outer;
        }

        'cols: for (i, col) in cols.iter().enumerate() {
            if i == cols.len() - 1 {
                continue;
            }
            let mut diff = col.chars().zip(cols[i+1].chars()).filter(|c| c.0 != c.1).count();
            println!("amount of differences: {diff}");
            if diff > 1 {
                continue;
            }

            let mut j = 0;
            while i >= j && i+1+j < cols.len() {
                diff = cols[i-j].chars().zip(cols[i+1+j].chars()).filter(|c| c.0 != c.1).count();
                println!("amount of differences: {diff}");
                if diff > 1 {
                    continue 'cols;
                }
                j += 1;
            }

            sum += i + 1;
            break;
        }
    }

    sum
}

fn main() {
    println!("Day 13:");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}
