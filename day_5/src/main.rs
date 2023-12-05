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
    for seed_range in seeds.unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<_>>().chunks(2) {
        //for seed in seed_range[0]..seed_range[0] + seed_range[1] {
        //    lowest_location = isize::min(lowest_location, almanac.get_location(seed));
        //}
        let mut start = seed_range[0];
        let end = seed_range[1] + start;
        //println!("Start: {start}, range: {}, end: {end}", seed_range[1]);
        while start <= end {
            if start < seed_range[0] {
                panic!("We went below...");
            }
            let ub = isize::min(end, almanac.get_upper_bound_of_same(start));
            //println!("(outside) start: {start}, actual start: {}, end: {end}, ub: {ub}", seed_range[0]);
            lowest_location = isize::min(lowest_location, almanac.get_location(ub));
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

    fn get_upper_bound_of_same(&self, seed: isize) -> isize {
        let ub = self.sections[0].get_upper_bound_of_same(seed);
        //let mut id_ub = self.sections[0].get(ub);
        let mut id = self.sections[0].get(seed);
        let mut diff = ub - seed;
        for section in &self.sections[1..self.sections.len()] {
            //if !section.get_upper_bound_of_same(id) == section.get_upper_bound_of_same(id_ub) {
            //    ub = self.get_seed(id, i + 1);
            //}
            
            //if !section.get_upper_bound_of_same(id) == section.get_upper_bound_of_same(id_ub) {
            //    ub = isize::min(ub, self.get_seed(section.get_upper_bound_of_same(id), i));
            //}

            //println!("Diff is {diff}");
            diff = isize::min(diff, section.get_upper_bound_of_same(id) - id);
            if diff == 0 {
                return seed;
            }
            id = section.get(id);
        }
        seed + diff
    }

    fn get_seed(&self, id: isize, level: usize) -> isize {
        let mut id = id;
        for i in (0..level).rev() {
            id = self.sections[i].get_rev(id);
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
    }

    fn get_rev(&self, dest: isize) -> isize {
        for range in &self.ranges {
            let diff = dest - range.0;
            if diff < range.2 && diff >= 0 {
                return range.1 + diff;
            }
        }
        dest
    }

    fn get_upper_bound_of_same(&self, source: isize) -> isize {
        for range in &self.ranges {
            let diff = source - range.1;
            if diff < range.2 && diff >= 0 {
                return range.1 + range.2 - 1;
            }
        }
        source
    }
}
