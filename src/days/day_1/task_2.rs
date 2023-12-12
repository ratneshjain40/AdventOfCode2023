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

    let numbers_text_pairs = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut pairs: Vec<(u32, u32)> = Vec::new();
    for line in blocks.iter() {
        let mut first: Option<u32> = None;

        for (i, c) in line.chars().enumerate() {
            if let Some(number) = c.to_digit(10) {
                first = Some(number);
                break;
            } else {
                for (text, number) in numbers_text_pairs.iter() {
                    if line[i..].starts_with(text) {
                        first = Some(number.parse().unwrap());
                        break;
                    }
                }
                if first.is_some() {
                    break;
                }
            }
        }

        let mut last: Option<u32> = None;
        for i in (0..line.len()).rev() {
            let c = line.chars().nth(i).unwrap();
            if c.is_digit(10) {
                last = Some(c.to_digit(10).unwrap());
                break;
            } else {
                for (text, number) in numbers_text_pairs.iter() {
                    if line[i..].starts_with(text) {
                        last = Some(number.parse::<u32>().unwrap());
                        break;
                    }
                }
                if last.is_some() {
                    break;
                }
            }
        }

        pairs.push((first.unwrap(), last.unwrap()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 281);
    }
}
