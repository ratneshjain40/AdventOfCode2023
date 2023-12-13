use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Entry {
    pub source: usize,
    pub destination: usize,
    pub range: usize,
}

impl Entry {
    pub fn new(source: usize, destination: usize, range: usize) -> Self {
        Self {
            source,
            destination,
            range,
        }
    }

    pub fn get_destination(&self, source: usize) -> Option<usize> {
        if source >= self.source && source <= self.source + self.range {
            return Some(self.destination + (source - self.source));
        }
        return None;
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

    pub fn get_destination(&self, source: usize) -> usize {
        for entry in &self.entries {
            if let Some(destination) = entry.get_destination(source) {
                return destination;
            }
        }
        return source;
    }
}

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<usize>, Vec<Map>) {
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

    return (seeds, maps);
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_5/{}", filename).as_str());
    let (seeds, maps) = parse_input(lines);
    let min_value = seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for map in &maps {
                value = map.get_destination(value);
            }
            return value;
        })
        .min()
        .unwrap();
    return min_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 35);
    }

    #[test]
    fn refactor() {
        assert_eq!(run("input.txt"), 389056265);
    }
}
