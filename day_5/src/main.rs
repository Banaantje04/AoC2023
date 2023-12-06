const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> isize {
    let mut sections = INPUT.split("\n\n");
    let seeds = sections.next();
    let sections = sections
        .map(|s| {
            let ranges = s
                .lines()
                .skip(1)
                .map(|m| {
                    let nums: Vec<_> = m.split_whitespace().map(|n| n.parse().unwrap()).collect();
                    (nums[0], nums[1], nums[2])
                })
                .collect();

            SectionMap { ranges }
        })
        .collect();

    let almanac = Almanac { sections };

    let mut lowest_location = isize::MAX;
    for seed in seeds
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
    {
        lowest_location = isize::min(lowest_location, almanac.get_location(seed));
    }
    lowest_location
}

fn solve_part_2() -> isize {
    let mut sections = INPUT.split("\n\n");
    let seeds = sections.next();
    let sections = sections
        .map(|s| {
            let ranges = s
                .lines()
                .skip(1)
                .map(|m| {
                    let nums: Vec<_> = m.split_whitespace().map(|n| n.parse().unwrap()).collect();
                    (nums[0], nums[1], nums[2])
                })
                .collect();

            SectionMap { ranges }
        })
        .collect();

    let almanac = Almanac { sections };

    let mut lowest_location = isize::MAX;
    for seed_range in seeds
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
    {
        let mut start = seed_range[0];
        let end = seed_range[1] + start;

        while start < end {
            if start < seed_range[0] {
                panic!("We went below...");
            }
            lowest_location = isize::min(lowest_location, almanac.get_location(start));

            let ub = almanac.get_upper_bound_of_same(start);

            start = ub + 1;
        }
    }
    lowest_location
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}

struct Almanac {
    sections: Vec<SectionMap>,
}

impl Almanac {
    fn get_location(&self, seed: isize) -> isize {
        let mut id = self.sections[0].get(seed);
        for section in &self.sections[1..self.sections.len()] {
            id = section.get(id);
        }
        id
    }

    fn get_upper_bound_of_same(&self, seed: isize) -> isize {
        let ub = self.sections[0].get_upper_bound_of_same(seed);
        let mut id = self.sections[0].get(seed);
        let mut diff = ub - seed;
        for section in &self.sections[1..self.sections.len()] {
            diff = isize::min(diff, section.get_upper_bound_of_same(id) - id);

            if diff == 0 {
                return seed;
            }

            id = section.get(id);
        }
        seed + diff
    }
}

#[derive(Debug)]
struct SectionMap {
    ranges: Vec<(isize, isize, isize)>,
}

impl SectionMap {
    fn get(&self, source: isize) -> isize {
        for range in &self.ranges {
            let diff = source - range.1;
            if diff < range.2 && diff >= 0 {
                return range.0 + diff;
            }
        }
        source
    }

    fn get_upper_bound_of_same(&self, source: isize) -> isize {
        for range in &self.ranges {
            let diff = source - range.1;
            if diff < range.2 && diff >= 0 {
                return range.1 + range.2 - 1;
            }
        }
        isize::MAX
    }
}
