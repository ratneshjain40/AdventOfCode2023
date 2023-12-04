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

fn is_game_possible(game_info: &GameInfo, cubes_in_bag: &Cubes) -> bool {
    let keys = cubes_in_bag.keys().collect::<Vec<&String>>();
    for round in &game_info.result {
        for key in keys.iter() {
            let value = round.get(*key).unwrap_or(&0);
            if value > cubes_in_bag.get(*key).unwrap_or(&0) {
                return false;
            }
        }
    }
    return true;
}

pub fn run(filename: &str) -> u32 {
    let game_info = read_input(format!("src/days/day_2/{}", filename).as_str());
    let cube_in_bag = Cubes::from([
        ("red".to_owned(), 12),
        ("green".to_owned(), 13),
        ("blue".to_owned(), 14),
    ]);
    let mut possible_games_sum = 0;
    for game in game_info {
        if is_game_possible(&game, &cube_in_bag) {
            possible_games_sum += game.game_id;
        }
    }
    return possible_games_sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 8);
    }
}