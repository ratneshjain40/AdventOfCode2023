use std::{fs::read_to_string, usize};

struct Data {
    symbols: Vec<(char, Coord)>,
    numbers: Vec<(String, Vec<Coord>)>,
}

type Coord = (usize, usize);

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn chebyshev_distance(coord_1: Coord, coord_2: Coord) -> usize {
    let x_diff = (coord_1.0 as i32 - coord_2.0 as i32).abs();
    let y_diff = (coord_1.1 as i32 - coord_2.1 as i32).abs();
    return x_diff.max(y_diff) as usize;
}

fn parse_input(lines: Vec<String>) -> Data {
    let mut data: Data = Data {
        symbols: Vec::new(),
        numbers: Vec::new(),
    };

    let mut num: (String, Vec<Coord>) = (String::new(), Vec::new());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    num.0.push(c);
                    num.1.push((x, y));
                }
                '.' => {
                    if num.0.len() > 0 {
                        data.numbers.push(num.clone());
                    }
                    num.0.clear();
                    num.1.clear();
                }
                _ => {
                    if num.0.len() > 0 {
                        data.numbers.push(num.clone());
                    }
                    data.symbols.push((c, (x, y)));
                    num.0.clear();
                    num.1.clear();
                }
            }
        }
    }
    return data;
}

fn find_adj(data: Data) -> Vec<usize> {
    let mut num_adj_to_symbol: Vec<usize> = Vec::new();
    for num in &data.numbers {
        let symbols = &data.symbols;
        for symbol in symbols {
            let min_distance = num
                .1
                .iter()
                .map(|coord| chebyshev_distance(*coord, symbol.1))
                .min()
                .unwrap();
            if min_distance == 1 {
                num_adj_to_symbol.push(num.0.parse::<usize>().unwrap());
            };
        }
    }
    num_adj_to_symbol
}

pub fn run(filename: &str) -> usize {
    let input = read_input(format!("src/days/day_3/{}", filename).as_str());
    let grid = parse_input(input);

    let adj_nums = find_adj(grid);
    return adj_nums.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 4361);
    }
}
