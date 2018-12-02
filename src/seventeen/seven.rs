use std::str::FromStr;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

#[derive(Clone, Debug)]
struct Node {
    pub name: String,
    pub weight: u32,
    pub children: Vec<String>,
}

impl<'a> From<&'a str> for Node {
    fn from(input: &str) -> Self {
        let mut split = input.split(" -> ");
        let mut parent = split
            .next()
            .expect("Found an empty line")
            .split_whitespace();

        let name = parent.next().unwrap();
        let weight = parent.next().expect("Missing weight");
        let weight = u32::from_str(&weight[1..weight.len() - 1]).expect("Found invalid weight");

        let children = if let Some(children) = split.next() {
            children
                .split(", ")
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        Node {
            name: String::from(name),
            weight: weight,
            children: children,
        }
    }
}

fn get_root(nodes: &Vec<Node>) -> &Node {
    let mut parent = nodes.iter().next().expect("Supplied nodes are empty");
    while let Some(node) = nodes
              .iter()
              .find(|n| n.children.iter().find(|&c| c == &parent.name) != None) {
        parent = node;
    }

    parent
}

fn first(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut nodes: Vec<Node> = Vec::new();
    for line in input.split("\n") {
        nodes.push(line.into());
    }

    get_root(&nodes).name.clone()
}

fn find_child<'a>(nodes: &'a Vec<Node>, name: &str) -> &'a Node {
    nodes.iter().find(|n| &n.name == name).expect("Missing child")
}

fn find_error_dist(nodes: &Vec<Node>, root: Node, debug: bool) -> Result<u32, u32> {
    let mut dists = HashMap::new();

    let name = root.name;
    for child in root.children {
        let child = find_child(nodes, &child);

        match find_error_dist(nodes, child.clone(), debug) {
            Ok(dist) => return Ok(dist),
            Err(sum) => {
                dists.insert(child.name.clone(), sum);
            }
        }
    }

    if debug && !dists.is_empty() {
        println!("{} ->\tdists: {:?}", name, dists);
    }

    let correct = dists.iter().tuple_combinations().find(|&(a, b)| a.1 == b.1).map(|(a, _)| a.1);

    if let Some((a, b)) = dists.iter().tuple_combinations().find(|&(a, b)| a.1 != b.1) {
        if let Some(correct) = correct {
            let diff = *a.1 as i32 - *b.1 as i32;

            if correct == a.1 {
                let problem_child = find_child(nodes, b.0);
                return Ok((problem_child.weight as i32 + diff) as u32);

            } else {
                let problem_child = find_child(nodes, a.0);
                return Ok((problem_child.weight as i32 - diff) as u32);
            }

        } else {
            panic!("Solution not deterministic");
        }
    }


    Err(dists.iter().map(|d| d.1).sum::<u32>() + root.weight)
}

fn second(input: &str, debug: bool) -> String {

    let mut nodes: Vec<Node> = Vec::new();
    for line in input.split("\n") {
        nodes.push(line.into());
    }

    let root = get_root(&nodes).clone();

    if debug {
        println!("Root: {:?}", root);
    }

    find_error_dist(&nodes, root, debug)
        .expect("Everything is balanced")
        .to_string()
}
