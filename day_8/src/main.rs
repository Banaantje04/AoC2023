use num::integer::lcm;

const INPUT: &str = include_str!("input.txt");

fn solve_part_1() -> usize {
    let mut input_l = INPUT.lines();
    let moves = input_l.next().unwrap();

    let mut graph = Graph { nodes: Vec::new() };
    let mut node_list = Vec::new();
    for n in input_l.skip(1) {
        let mut line_split = n.split(" = ");
        let id = line_split.next().unwrap();
        let dirs = line_split.next().unwrap();
        let mut dirs = (&dirs[1..dirs.len() - 1]).split(", ");
        let left = dirs.next().unwrap();
        let right = dirs.next().unwrap();

        graph.nodes.push(Node {
            id,
            left: 0,
            right: 0,
        });
        node_list.push((id, (left, right)));
    }

    for n in node_list {
        let left = graph.nodes.iter().position(|l| l.id == n.1 .0).unwrap();
        let right = graph.nodes.iter().position(|r| r.id == n.1 .1).unwrap();

        let node: &mut Node = graph.nodes.iter_mut().find(|i| i.id == n.0).unwrap();
        node.left = left;
        node.right = right;
    }

    let mut curr = graph.nodes.iter().find(|n| n.id == "AAA").unwrap();
    let mut movechar = moves.chars();
    let mut num = 0;
    while curr.id != "ZZZ" {
        match movechar.next() {
            Some(dir) => {
                num += 1;
                match dir {
                    'R' => curr = &graph.nodes[curr.right],
                    'L' => curr = &graph.nodes[curr.left],
                    c => panic!("This is not a direction! {c}"),
                }
            }
            None => movechar = moves.chars(),
        }
    }

    num
}

fn solve_part_2() -> usize {
    let mut input_l = INPUT.lines();
    let moves = input_l.next().unwrap();

    let mut graph = Graph { nodes: Vec::new() };
    let mut node_list = Vec::new();
    for n in input_l.skip(1) {
        let mut line_split = n.split(" = ");
        let id = line_split.next().unwrap();
        let dirs = line_split.next().unwrap();
        let mut dirs = (&dirs[1..dirs.len() - 1]).split(", ");
        let left = dirs.next().unwrap();
        let right = dirs.next().unwrap();

        graph.nodes.push(Node {
            id,
            left: 0,
            right: 0,
        });
        node_list.push((id, (left, right)));
    }

    for n in node_list {
        let left = graph.nodes.iter().position(|l| l.id == n.1 .0).unwrap();
        let right = graph.nodes.iter().position(|r| r.id == n.1 .1).unwrap();

        let node: &mut Node = graph.nodes.iter_mut().find(|i| i.id == n.0).unwrap();
        node.left = left;
        node.right = right;
    }

    let curr = graph.nodes.iter().filter(|n| n.id.ends_with("A"));
    let runtimes = curr.map(|n| {
        let mut curr_n = n;
        let mut movechar = moves.chars();
        let mut num = 0;
        while !curr_n.id.ends_with("Z") {
            match movechar.next() {
                Some(dir) => {
                    num += 1;
                    match dir {
                        'R' => curr_n = &graph.nodes[curr_n.right],
                        'L' => curr_n = &graph.nodes[curr_n.left],
                        c => panic!("This is not a direction! {c}"),
                    }
                }
                None => movechar = moves.chars(),
            }
        }
        num
    });
    
    println!("{:?}", runtimes.clone().collect::<Vec<_>>());
    runtimes.reduce(|lcm, n| num::integer::lcm(lcm, n)).unwrap()
}

fn main() {
    println!("Day 8: Haunted Wasteland");
    println!("The solution to part one is: {}", solve_part_1());
    println!("The solution to part two is: {}", solve_part_2());
}

struct Graph<'a> {
    nodes: Vec<Node<'a>>,
}

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    left: usize,
    right: usize,
}
