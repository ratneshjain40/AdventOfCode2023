use std::fs::read_to_string;

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> Vec<String> {
    todo!()
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_X/{}", filename).as_str());
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 35);
    }
}