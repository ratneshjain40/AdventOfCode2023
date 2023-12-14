use std::{
    cmp::{max, min},
    fs::read_to_string,
    ops::Range,
};

#[derive(Debug, Clone)]
struct Entry {
    pub source: Range<usize>,
    pub destination: Range<usize>,
}

impl Entry {
    pub fn new(source: usize, destination: usize, range: usize) -> Self {
        Self {
            source: source..source + range - 1,
            destination: destination..destination + range - 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    pub entries: Vec<Entry>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn insert(&mut self, source: usize, destination: usize, range: usize) {
        self.entries.push(Entry::new(source, destination, range));
    }
}

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<Range<usize>>, Vec<Map>) {
    let mut maps: Vec<Map> = Vec::new();

    let raw_seeds = lines[0].split(" ").skip(1).collect::<Vec<&str>>();
    let seeds = raw_seeds
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut start_map = false;
    let mut map: Map = Map::new();
    for line_index in 1..lines.len() {
        let line = &lines[line_index];
        if start_map && line == "" {
            start_map = false;
            maps.push(map.clone());
            map.clear();
        }

        if start_map {
            let raw_data = line.split(" ").collect::<Vec<&str>>();
            let destination = raw_data[0].parse::<usize>().unwrap();
            let source = raw_data[1].parse::<usize>().unwrap();
            let range = raw_data[2].parse::<usize>().unwrap();

            map.insert(source, destination, range);
        }

        if line.ends_with("map:") {
            start_map = true;
        }
    }

    if start_map {
        maps.push(map.clone());
    }

    let mut seed_ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1] - 1);
    }

    return (seed_ranges, maps);
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_5/{}", filename).as_str());
    let (mut seed_ranges, blocks) = parse_input(lines);

    for block in blocks {
        let mut new = Vec::new();
        while seed_ranges.len() > 0 {
            let seed = seed_ranges.pop().unwrap();

            let mut flag = true;
            for entry in block.entries.iter() {
                let os = max(seed.start, entry.source.start);
                let oe = min(seed.end, entry.source.end);
                if os < oe {
                    new.push(
                        entry.destination.start + (os - entry.source.start)
                            ..entry.destination.start + (oe - entry.source.start),
                    );
                    if os > seed.start {
                        seed_ranges.push(seed.start..os);
                    }
                    if seed.end > oe {
                        seed_ranges.push(oe..seed.end);
                    }
                    flag = false;
                    break;
                }
            }
            if flag {
                new.push(seed);
            }
        }
        seed_ranges = new;
    }
    let min_value = seed_ranges.iter().map(|seed| seed.start).min().unwrap();
    return min_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 46);
    }

    #[test]
    fn refactor() {
        assert_eq!(run("input.txt"), 389056265);
    }
}
