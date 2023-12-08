use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

struct Cursor<'a> {
    pub instructions: String,
    pub current_node: Option<&'a PathNode>,
}

impl<'a> Cursor<'a> {
    pub fn walk(mut self, start: &str, node_map: &'a HashMap<String, PathNode>) -> Option<u32> {
        self.current_node = node_map.get(&start.to_string());
        let mut steps = 0;
        while self.current_node.is_some() && self.current_node?.name != "ZZZ" {
            let instruction = self.instructions.remove(0);
            match instruction {
                'R' => {
                    println!("Right");
                    self.current_node = node_map.get(&self.current_node?.conns[1]);
                }
                'L' => {
                    println!("Left");
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
    let cursor = Cursor {
        instructions: input.lines().nth(0).unwrap().to_string(),
        current_node: None,
    };

    let mut node_map: HashMap<String, PathNode> = HashMap::new();

    input.lines().enumerate().for_each(|(idx, line)| {
        if idx >= 2 {
            let node = PathNode::new(line);
            node_map.insert(node.name.clone(), node.clone());
        }
    });

    cursor.walk("AAA", &node_map)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
