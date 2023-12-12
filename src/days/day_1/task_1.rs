use std::fs::read_to_string;

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn run(filename: &str) -> usize {
    let blocks = read_input(format!("src/days/day_1/{}", filename).as_str());

    let mut pairs: Vec<(u32, u32)> = Vec::new();
    for line in blocks.iter() {
        let first_index = line.find(|c: char| c.is_digit(10)).unwrap();

        let first = line.chars().nth(first_index).unwrap().to_digit(10).unwrap();

        let last_index = line.rfind(|c: char| c.is_digit(10)).unwrap();
        let last = line.chars().nth(last_index).unwrap().to_digit(10).unwrap();

        pairs.push((first, last));
    }
    let digits: Vec<u32> = pairs
        .iter()
        .map(|(first, last)| {
            return 10 * first + last;
        })
        .collect();

    let sum: u32 = digits.iter().sum();
    sum as usize
}
