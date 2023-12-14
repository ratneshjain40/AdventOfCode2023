use std::fs::read_to_string;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (usize, usize) {
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

    let time: usize = raw_time.join("").parse().unwrap();
    let distance: usize = raw_distance.join("").parse().unwrap();

    return (time, distance);
}

pub fn run(filename: &str) -> usize {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let lines = read_input(format!("src/days/day_6/{}", filename).as_str());
    let (time, distance) = parse_input(lines);

    let race_time = time;
    let max_distance = distance;

    let progress_bar = indicatif::ProgressBar::new(race_time as u64);

    let victory_counter: Vec<usize> = (0..=race_time)
        .into_par_iter()
        .map(|speed| {
            progress_bar.inc(1);
            let rm_time = race_time - speed;
            let distance = speed * rm_time;
            if distance > max_distance {
                1
            } else {
                0
            }
        }).collect();

    let victory_counter: usize = victory_counter.into_iter().sum();

    return victory_counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 71503);
    }
}
