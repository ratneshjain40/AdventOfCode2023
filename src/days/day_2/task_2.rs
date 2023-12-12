use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct GameInfo {
    pub game_id: u32,
    pub result: Vec<Cubes>,
}

type Cubes = HashMap<String, u32>;

fn read_input(filepath: &str) -> Vec<GameInfo> {
    let data: Vec<String> = fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut game_info: Vec<GameInfo> = Vec::new();
    for entry in data {
        let game_data = entry.split_once(":").unwrap();
        let game_id = game_data.0.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        let sub_data: Vec<&str> = game_data.1.split(";").collect();
        let rounds: Vec<Cubes> = sub_data
            .iter()
            .map(|round| {
                let sub_round_data: Vec<&str> = round.trim().split(",").collect();
                let sub_round: Cubes = sub_round_data
                    .iter()
                    .map(|cube| {
                        let cube_data = cube.trim().split_once(" ").unwrap();
                        let cube_color = cube_data.1;
                        let cube_value = cube_data.0.parse::<u32>().unwrap();
                        (cube_color.to_string(), cube_value)
                    })
                    .collect();
                sub_round
            })
            .collect();

        game_info.push(GameInfo {
            game_id,
            result: rounds,
        });
    }
    return game_info;
}

fn min_max_cubes_needed(game_info: &GameInfo) -> Cubes {
    let mut min_max_cubes: Cubes = HashMap::new();
    for round in &game_info.result {
        for (key, value) in round {
            let min_max_value = min_max_cubes.entry(key.to_string()).or_insert(0);
            if value > min_max_value {
                *min_max_value = *value;
            }
        }
    }
    return min_max_cubes;
}

pub fn run(filename: &str) -> usize {
    let game_info = read_input(format!("src/days/day_2/{}", filename).as_str());
    let mut sum = 0;
    for game in &game_info {
        let min_max_cubes = min_max_cubes_needed(game);
        sum += min_max_cubes.values().product::<u32>();
    }
    return sum as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 2286);
    }
}