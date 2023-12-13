use std::{fs::read_to_string, ops::Range};

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

    fn sort_by_source(&mut self) {
        self.entries
            .sort_by(|a, b| a.source.start.cmp(&b.source.start));
    }

    pub fn fill_range_gaps(&mut self) {
        self.sort_by_source();
        let mut gaps: Vec<Entry> = Vec::new();
        for i in 0..self.entries.len() - 2 {
            let current = &self.entries[i];
            let next = &self.entries[i + 1];
            let gap = next.source.start - current.source.end;
            if gap > 1 {
                gaps.push(Entry::new(
                    current.source.end + 1,
                    current.source.end + 1,
                    gap - 1,
                ));
                println!("{:?}", gaps);
            }
        }
        self.entries.append(&mut gaps);
        self.sort_by_source();
    }

    pub fn get_desination_ranges(&self, ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut result: Vec<Range<usize>> = Vec::new();
        for mut range in ranges {
            let mut destination_ranges: Vec<Range<usize>> = Vec::new();

            if range.start < self.entries[0].source.start
                && range.end < self.entries[0].source.start
                || range.start > self.entries[0].source.end
                    && range.end > self.entries[0].source.end
            {
                destination_ranges.push(range.clone());
                break;
            }

            if range.start < self.entries[0].source.start {
                destination_ranges.push(range.start..self.entries[0].source.start - 1);
                range.start = self.entries[0].source.start;
            }

            if range.end > self.entries[self.entries.len() - 1].source.end {
                destination_ranges
                    .push(self.entries[self.entries.len() - 1].source.end + 1..range.end);
                range.end = self.entries[self.entries.len() - 1].source.end;
            }

            for (_, entry) in self.entries.iter().enumerate() {
                if entry.source.contains(&range.start) && entry.source.contains(&range.end) {
                    destination_ranges.push(
                        entry.destination.start + (range.start - entry.source.start)
                            ..entry.destination.start + (range.end - entry.source.start),
                    );
                    break;
                }
                if entry.source.contains(&range.start) {
                    destination_ranges.push(
                        entry.destination.start + (range.start - entry.source.start)
                            ..entry.destination.end,
                    );
                    continue;
                }
                if entry.source.start > range.start && entry.source.end < range.end {
                    destination_ranges.push(entry.destination.clone());
                    continue;
                }
                if entry.source.contains(&range.end) {
                    destination_ranges.push(
                        entry.destination.start
                            ..entry.destination.start + (range.end - entry.source.start),
                    );
                    break;
                }
            }

            println!("{:?}", destination_ranges);
            result.append(&mut destination_ranges);
        }
        return result;
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
            map.fill_range_gaps();
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
        seed_ranges.push(seeds[i]..seeds[i] + seeds[i + 1]);
    }

    return (seed_ranges, maps);
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_5/{}", filename).as_str());
    let (seed_ranges, maps) = parse_input(lines);
    let locations = seed_ranges
        .iter()
        .map(|seed_ranges| {
            let mut value = vec![seed_ranges.clone()];
            for map in &maps {
                value = map.get_desination_ranges(value);
            }

            value.iter().map(|x| x.start).min().unwrap()
        })
        .collect::<Vec<usize>>();

    return locations.iter().min().unwrap().clone();
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
