use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

struct Cursor<'a> {
    pub instructions: String,
    pub current_node: Option<&'a PathNode>,
    pub start: String,
}

impl<'a> Cursor<'a> {
    pub fn walk(&mut self, node_map: &'a HashMap<String, PathNode>) -> Option<u32> {
        self.current_node = node_map.get(&self.start);
        let mut steps = 0;

        while self.current_node.is_some() && !self.current_node?.name.ends_with("Z") {
            let instruction = self.instructions.remove(0);
            match instruction {
                'R' => {
                    self.current_node = node_map.get(&self.current_node?.conns[1]);
                }
                'L' => {
                    self.current_node = node_map.get(&self.current_node?.conns[0]);
                }
                _ => {
                    println!("UNKNOWN STEP");
                    return None;
                }
            }
            self.instructions.push(instruction);
            steps += 1;
        }

        Some(steps)
    }
}

#[derive(Clone)]
struct PathNode {
    pub name: String,
    pub conns: Vec<String>,
}

impl PathNode {
    pub fn new(line: &str) -> PathNode {
        let (name, conns) = line.split_once('=').unwrap();
        PathNode {
            name: name.trim().to_string(),
            conns: conns
                .replace("(", "")
                .replace(")", "")
                .split(",")
                .map(|word| word.trim())
                .map(|word| word.to_string())
                .collect_vec(),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut cursor = Cursor {
        instructions: input.lines().nth(0).unwrap().to_string(),
        current_node: None,
        start: "AAA".to_string(),
    };

    let mut node_map: HashMap<String, PathNode> = HashMap::new();

    input.lines().enumerate().for_each(|(idx, line)| {
        if idx >= 2 {
            let node = PathNode::new(line);
            node_map.insert(node.name.clone(), node.clone());
        }
    });

    cursor.walk(&node_map)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_of_list(numbers: Vec<u32>) -> u64 {
    numbers.iter().fold(1, |acc, n| lcm(acc, *n as u64))
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = input.lines().nth(0).unwrap().to_string();

    let mut node_map: HashMap<String, PathNode> = HashMap::new();
    let mut cursors: Vec<Cursor> = vec![];
    input.lines().enumerate().for_each(|(idx, line)| {
        if idx >= 2 {
            let node = PathNode::new(line);
            if node.name.ends_with("A") {
                cursors.push(Cursor {
                    instructions: instructions.clone(),
                    current_node: None,
                    start: node.name.clone(),
                });
            }
            node_map.insert(node.name.clone(), node.clone());
        }
    });

    println!("Found {} cursors.", cursors.len());

    let step_counts = cursors
        .iter_mut()
        .map(|cursor| cursor.walk(&node_map).unwrap())
        .collect_vec();

    Some(lcm_of_list(step_counts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
