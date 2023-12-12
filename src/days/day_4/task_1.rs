use std::fs::read_to_string;

struct Card {
    pub winning_numbers: Vec<usize>,
    pub numbers: Vec<usize>,
}

impl Card {
    pub fn new(winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            winning_numbers,
            numbers,
        }
    }

    pub fn get_no_of_winning_numbers(&self) -> usize {
        let mut count = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }
        return count;
    }

    pub fn calc_points(&self) -> usize {
        let no_of_winning_numbers = self.get_no_of_winning_numbers() as u32;
        if no_of_winning_numbers == 0 {
            return 0;
        } else {
            return 2u32.pow(no_of_winning_numbers - 1) as usize;
        }
    }
}

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        let (_, numbers) = line.split_once(":").unwrap();
        let (raw_winning_numbers, raw_numbers) = numbers.split_once("|").unwrap();
        let raw_winning_numbers: Vec<&str> = raw_winning_numbers
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let raw_winning_numbers: Vec<usize> = raw_winning_numbers
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let raw_numbers: Vec<&str> = raw_numbers
            .trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let raw_numbers: Vec<usize> = raw_numbers
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        cards.push(Card::new(raw_winning_numbers, raw_numbers));
    }
    return cards;
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_4/{}", filename).as_str());
    let cards = parse_input(lines);
    let mut points = 0;
    for card in cards {
        points += card.calc_points();
    }
    return points;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 13);
    }
}
