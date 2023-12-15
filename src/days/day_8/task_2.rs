use std::{collections::HashMap, fs::read_to_string};

struct Data {
    pub map: HashMap<String, (String, String)>,
    pub loops: HashMap<String, Vec<String>>,
}

impl Data {
    fn find_all_loops(&mut self, start: &str, current: &str, directions: &mut String) {
        if directions.len() > 0 && current == start {
            self.loops
                .entry(start.to_string())
                .or_insert(Vec::new())
                .push(directions.clone());
            return;
        }
        if current == "ZZZ" {
            directions.pop();
            return;
        }

        let left = self.traverse(current.to_string(), 'L');

        if left != current && left != "ZZZ" {
            directions.push('L');
            self.find_all_loops(start, &left, directions);
            directions.pop();
        }

        let right = self.traverse(current.to_string(), 'R');
        if right != current && right != "ZZZ" {
            directions.push('R');
            self.find_all_loops(start, &right, directions);
            directions.pop();
        }
    }

    pub fn find_path_to_end(&self, current: &str, directions: &mut String) {
        if current == "ZZZ" {
            return;
        }

        let left = self.traverse(current.to_string(), 'L');
        if left != current && left != "ZZZ" {
            directions.push('L');
            self.find_path_to_end(&left, directions);
        }

        let right = self.traverse(current.to_string(), 'R');
        if right != current && right != "ZZZ" {
            directions.push('R');
            self.find_path_to_end(&right, directions);
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            loops: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> (String, String) {
        self.map.get(&key).unwrap().clone()
    }

    pub fn insert(&mut self, key: String, value: (String, String)) {
        self.map.insert(key, value);
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

fn parse_input(mut lines: Vec<String>) -> (String, Data) {
    let path = lines.remove(0);
    lines.remove(0);

    let mut map = Data::new();
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
    let (path, mut map) = parse_input(lines);

    let keys = map.map.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        println!("For {}", key);
        map.find_all_loops(&key, &key, &mut String::new());
    }
    println!("{:?}", map.loops);


    println!("Final Directions");
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test_2.txt"), 2);
    }
}
