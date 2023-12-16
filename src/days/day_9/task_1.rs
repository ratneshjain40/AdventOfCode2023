use std::fs::read_to_string;

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<i128>> {
    lines
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i128>().unwrap())
                .collect()
        })
        .collect()
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_9/{}", filename).as_str());
    let nums = parse_input(lines);

    let mut predictions = Vec::new();
    for list in nums {
        let mut accumulator = list[list.len() - 1];
        let mut diffs: Vec<_> = list.windows(2).map(|pair| pair[1] - pair[0]).collect();
        while !diffs.iter().all(|diff| diff == &0) {
            accumulator += diffs[diffs.len() - 1];
            diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect();
        }
        predictions.push(accumulator);
    }
    let sum = predictions.iter().sum::<i128>();
    return sum as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 114);
    }
}
