use std::{collections::HashMap, fs::read_to_string, usize};

#[derive(Debug)]
struct Data {
    max_row: usize,
    symbols: HashMap<RowNumber, Vec<(char, Coord)>>,
    numbers: HashMap<RowNumber, Vec<(String, Vec<Coord>)>>,
}

type RowNumber = usize;
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
        max_row: lines.len() - 1,
        symbols: HashMap::new(),
        numbers: HashMap::new(),
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
                        data.numbers.entry(y).or_insert(vec![]).push(num.clone());
                    }
                    num.0.clear();
                    num.1.clear();
                }
                _ => {
                    if num.0.len() > 0 {
                        data.numbers.entry(y).or_insert(vec![]).push(num.clone());
                    }
                    data.symbols.entry(y).or_insert(vec![]).push((c, (x, y)));
                    num.0.clear();
                    num.1.clear();
                }
            }
        }
    }
    return data;
}

fn get_nearby_symbols(data: &Data, row_number: RowNumber) -> Vec<(char, Coord)> {
    let mut nearby_symbols = Vec::new();

    if row_number > 0 {
        if let Some(symbols) = data.symbols.get(&(row_number - 1)) {
            for symbol in symbols {
                nearby_symbols.push(symbol.clone());
            }
        }
    }

    if row_number < data.max_row {
        if let Some(symbols) = data.symbols.get(&(row_number + 1)) {
            for symbol in symbols {
                nearby_symbols.push(symbol.clone());
            }
        }
    }

    if row_number > 0 || row_number < data.max_row {
        if let Some(symbols) = data.symbols.get(&row_number) {
            for symbol in symbols {
                nearby_symbols.push(symbol.clone());
            }
        }
    }

    nearby_symbols
}

fn find_adj(data: Data) -> Vec<usize> {
    let mut adj_nums = Vec::new();

    for (row_number, numbers) in &data.numbers {
        let symbols = get_nearby_symbols(&data, *row_number);
        for num in numbers {
            for symbol in &symbols {
                let min_distance = num
                    .1
                    .iter()
                    .map(|coord| chebyshev_distance(*coord, symbol.1))
                    .min()
                    .unwrap();
                if min_distance == 1 {
                    adj_nums.push(num.0.parse::<usize>().unwrap());
                    break;
                };
            }
        }
    }

    adj_nums
}

pub fn run(filename: &str) -> usize {
    let input = read_input(format!("src/days/day_3/{}", filename).as_str());
    let grid = parse_input(input);

    let adj_nums = find_adj(grid);
    let mut sorted_adj_nums = adj_nums.clone();
    sorted_adj_nums.sort();
    println!("{:?}", sorted_adj_nums);
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
