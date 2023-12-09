use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let mut hands = INPUT
        .lines()
        .map(|l| Hand::create_part_1(l.split_whitespace().collect()))
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.cmp(b));

    //println!("The sorted hands are: {:?}", hands);

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }
    sum
}

fn solve_part_2() -> usize {
    let mut hands = INPUT
        .lines()
        .map(|l| Hand::create_part_2(l.split_whitespace().collect()))
        .collect::<Vec<_>>();
    hands.sort_by(|a, b| a.cmp(b));

    //println!("The sorted hands are: {:?}", hands);

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }
    sum
}

fn main() {
    println!("Day 7: Camel Cards");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    hand: Vec<usize>,
    bid: usize,
    type_id: usize,
}

impl Ord for Hand {
    fn cmp(&self, o: &Self) -> Ordering {
        if self.type_id == o.type_id {
            Hand::compare_hand_string(&self.hand, &o.hand)
        } else if self.type_id > o.type_id {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Hand {
    fn compare_hand_string(hand: &Vec<usize>, ohand: &Vec<usize>) -> Ordering {
        let mut cmps = hand
            .iter()
            .zip(ohand)
            .map(|c| c.0.cmp(&c.1))
            .filter(|o| *o != Ordering::Equal);
        let cmp = cmps.next().unwrap();
        //println!("Hand {:?} was {:?} than {:?}", hand, cmp, ohand);
        cmp
    }

    fn create_part_1(hand: Vec<&str>) -> Hand {
        let conv: HashMap<char, usize> = HashMap::from([
            ('2', 0),
            ('3', 1),
            ('4', 2),
            ('5', 3),
            ('6', 4),
            ('7', 5),
            ('8', 6),
            ('9', 7),
            ('T', 8),
            ('J', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]);

        let handnums = hand[0].chars().map(|c| *(conv.get(&c).unwrap())).collect();
        let bid = hand[1].parse().unwrap();
        let type_id = Hand::get_type_part_1(&handnums);

        Hand {
            hand: handnums,
            bid,
            type_id,
        }
    }

    fn get_type_part_1(hand: &Vec<usize>) -> usize {
        let mut m: HashMap<usize, usize> = HashMap::new();
        for i in hand {
            *m.entry(*i).or_default() += 1;
        }

        let maxs = m.iter().map(|(_, v)| *v);
        let max = maxs.clone().max().unwrap();

        if max == 5 {
            0
        } else if max == 4 {
            1
        } else if max == 3 {
            if maxs.filter(|n| *n == 2).count() > 0 {
                2
            } else {
                3
            }
        } else if max == 2 {
            if maxs.filter(|n| *n == 2).count() == 2 {
                4
            } else {
                5
            }
        } else {
            6
        }
    }

    fn create_part_2(hand: Vec<&str>) -> Hand {
        let conv: HashMap<char, usize> = HashMap::from([
            ('J', 0),
            ('2', 1),
            ('3', 2),
            ('4', 3),
            ('5', 4),
            ('6', 5),
            ('7', 6),
            ('8', 7),
            ('9', 8),
            ('T', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]);

        let handnums = hand[0].chars().map(|c| *(conv.get(&c).unwrap())).collect();
        let bid = hand[1].parse().unwrap();
        let type_id = Hand::get_type_part_2(&handnums);

        Hand {
            hand: handnums,
            bid,
            type_id,
        }
    }

    fn get_type_part_2(hand: &Vec<usize>) -> usize {
        let mut m: HashMap<usize, isize> = HashMap::new();
        for i in hand {
            *m.entry(*i).or_default() += 1;
        }

        let maxs = m.iter().filter(|(k, _)| **k != 0);
        let max = maxs.clone().max_by_key(|(_, v)| *v).unwrap_or((&0, &0));

        let type_id = if *max.1 == 5 || max.1 + m.get(&0).unwrap_or(&0) == 5 {
            0
        } else if *max.1 == 4 || *max.1 + m.get(&0).unwrap_or(&0) == 4 {
            1
        } else if *max.1 == 3 || *max.1 + m.get(&0).unwrap_or(&0) == 3 {
            if maxs.filter(|n| *n.1 == 2 && *n.0 != *max.0).count() > 0 {
                2
            } else {
                3
            }
        } else if *max.1 == 2 || *max.1 + m.get(&0).unwrap_or(&0) == 2 {
            if maxs.filter(|n| *n.1 == 2 && *n.0 != *max.0).count() > 0 {
                4
            } else {
                5
            }
        } else {
            6
        };
        if *m.get(&0).unwrap_or(&0) > 0 {
            assert!(type_id != 6);
        }
        //println!("Hand {:?} was assigned type {type_id}", hand);
        type_id
    }
}
