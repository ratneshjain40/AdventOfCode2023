use std::fs::read_to_string;

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut time: Vec<usize> = Vec::new();
    let mut distance: Vec<usize> = Vec::new();

    let raw_time = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();
    let raw_distance = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .filter(|x| x != &"")
        .collect::<Vec<&str>>();

    for i in 0..raw_time.len() {
        time.push(raw_time[i].parse::<usize>().unwrap());
        distance.push(raw_distance[i].parse::<usize>().unwrap());
    }

    return (time, distance);
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_6/{}", filename).as_str());
    let (time, distance) = parse_input(lines);

    let total_races = time.len();
    let possible_victories: Vec<usize> = (0..total_races)
        .filter_map(|race_index| {
            let race_time = time[race_index];
            let max_distance = distance[race_index];

            let victory_counter = (0..=race_time)
                .filter(|&speed| speed * (race_time - speed) > max_distance)
                .count();

            if victory_counter > 0 {
                Some(victory_counter)
            } else {
                None
            }
        })
        .collect();

    return possible_victories.into_iter().product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 288);
    }

    #[test]
    fn test_run_2() {
        assert_eq!(run("input.txt"), 4568778);
    }
}
