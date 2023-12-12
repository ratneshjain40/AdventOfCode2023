use std::{collections::HashMap, fs::read_to_string, usize};

#[derive(Debug)]
struct Data {
    max_row: usize,
    symbols: HashMap<RowNumber, Vec<(char, Coord)>>,
    numbers: HashMap<RowNumber, Vec<(String, Vec<Coord>)>>,
}

impl Data {
    fn get_nearby_numbers(&self, row_number: RowNumber) -> Vec<(String, Vec<Coord>)> {
        let mut nearby_numbers = Vec::new();

        if row_number > 0 {
            if let Some(numbers) = self.numbers.get(&(row_number - 1)) {
                for number in numbers {
                    nearby_numbers.push(number.clone());
                }
            }
        }

        if row_number < self.max_row {
            if let Some(numbers) = self.numbers.get(&(row_number + 1)) {
                for number in numbers {
                    nearby_numbers.push(number.clone());
                }
            }
        }

        if row_number > 0 || row_number < self.max_row {
            if let Some(numbers) = self.numbers.get(&row_number) {
                for number in numbers {
                    nearby_numbers.push(number.clone());
                }
            }
        }

        nearby_numbers
    }

    fn get_adj_num_to_symbol(&self, row_number: RowNumber, index: usize) -> Vec<usize> {
        let nums = self.get_nearby_numbers(row_number);
        let symbol = self.symbols.get(&row_number).unwrap()[index];
        let adj_nums: Vec<usize> = nums
            .iter()
            .map(|(num, coords)| {
                let min_distance = coords
                    .iter()
                    .map(|coord| chebyshev_distance(*coord, symbol.1))
                    .min()
                    .unwrap();
                if min_distance == 1 {
                    Some(num.parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .filter_map(|x| x)
            .collect();
        return adj_nums;
    }
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

fn get_gear_ratios(data: Data) -> Vec<usize> {
    let mut gear_ratios = Vec::new();

    for (row_number, symbols) in &data.symbols {
        for (index, (symbol, _)) in symbols.iter().enumerate() {
            if symbol == &'*' {
                let adj_nums = data.get_adj_num_to_symbol(*row_number, index);
                if adj_nums.len() == 2 {
                    gear_ratios.push(adj_nums.iter().product());
                }
            }
        }
    }

    gear_ratios
}

pub fn run(filename: &str) -> usize {
    let input = read_input(format!("src/days/day_3/{}", filename).as_str());
    let grid = parse_input(input);

    let adj_nums = get_gear_ratios(grid);
    return adj_nums.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 467835);
    }
}
