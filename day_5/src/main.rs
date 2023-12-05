const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> isize {
    let mut sections = INPUT.split("\n\n");
    let seeds = sections.next();
    let sections = sections.map(|s| {
        let ranges = s.lines().skip(1).map(|m| {
            let nums: Vec<_> = m.split_whitespace().map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        }).collect();

        SectionMap {
            ranges,
        }
    }).collect();

    let almanac = Almanac {
        sections,
    };

    let mut lowest_location = isize::MAX;
    for seed in seeds.unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap()) {
        lowest_location = isize::min(lowest_location, almanac.get_location(seed));
    }
    lowest_location
}

fn solve_part_2() -> isize {
    0
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}

struct Almanac {
    sections: Vec<SectionMap>
}

impl Almanac {
    fn get_location(&self, seed: isize) -> isize {
        //println!("seed: {seed}");
        let mut id = self.sections[0].get(seed);
        //println!("soil: {id}");
        for section in &self.sections[1..self.sections.len()] {
            id = section.get(id);
            //println!("thing: {id}");
        }
        id
    }
}

#[derive(Debug)]
struct SectionMap {
    ranges: Vec<(isize, isize, isize)>
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
        //panic!("Id {source} was not found! {:?}", self);      //this should not happen but otherwise rust complains lol
    }
}
