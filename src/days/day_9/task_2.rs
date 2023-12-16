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

    let predictions = nums
        .into_iter()
        .map(|list| {
            println!("\nList {:?}", list);
            let mut accumulator = vec![list[0]];
            let mut diffs: Vec<_> = list.windows(2).map(|pair| pair[1] - pair[0]).collect();
            while !diffs.iter().all(|diff| diff == &0) {
                println!("Diffs {:?}", diffs);
                accumulator.push(diffs[0]);
                println!("Sub {:?} Accumulator {:?} ", diffs[0], accumulator);
                diffs = diffs.windows(2).map(|pair| pair[1] - pair[0]).collect();
            }

            accumulator.reverse();
            let mut value = accumulator[0];
            for i in 1..accumulator.len() {
                value = accumulator[i] - value;
            }
            println!("Value {:?}", value);
            return value;
        })
        .collect::<Vec<_>>();
    let sum = predictions.iter().sum::<i128>();
    return sum as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 2);
    }
}
