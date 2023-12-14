use std::{collections::HashMap, fs::read_to_string};

struct Map(HashMap<String, (String, String)>);

impl Map {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: String) -> (String, String) {
        self.0.get(&key).unwrap().clone()
    }

    pub fn insert(&mut self, key: String, value: (String, String)) {
        self.0.insert(key, value);
    }

    pub fn traverse(&self, from: String, direction: char) -> String {
        let peers = self.get(from);
        if direction == 'L' {
            peers.0
        } else {
            peers.1
        }
    }
}

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(mut lines: Vec<String>) -> (String, Map) {
    let path = lines.remove(0);
    lines.remove(0);

    let mut map = Map::new();
    for line in lines {
        let (raw_node, raw_peers) = line.split_once("=").unwrap();
        let node = raw_node.trim();
        let (raw_left, raw_right) = raw_peers.trim().split_once(", ").unwrap();
        let left = raw_left.trim().strip_prefix("(").unwrap();
        let right = raw_right.trim().strip_suffix(")").unwrap();
        map.insert(node.to_string(), (left.to_string(), right.to_string()));
    }
    return (path, map);
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_8/{}", filename).as_str());
    let (path, map) = parse_input(lines);

    let mut curr = String::from("AAA");
    let directions = path.chars().collect::<Vec<char>>();
    let mut count = 0;

    while curr != "ZZZ" {
        for direction in directions.iter() {
            curr = map.traverse(curr, direction.clone());
            count += 1;
            if curr == "ZZZ" {
                break;
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 2);
    }

    #[test]
    fn test_run_2() {
        assert_eq!(run("input_test_2.txt"), 6);
    }
}
