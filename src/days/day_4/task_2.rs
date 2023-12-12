use std::fs::read_to_string;

struct Card {
    pub winning_numbers: Vec<usize>,
    pub numbers: Vec<usize>,
    pub copies: usize,
}

impl Card {
    pub fn new(winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            winning_numbers,
            numbers,
            copies: 1,
        }
    }

    pub fn get_no_of_copies(&self) -> usize {
        return self.copies;
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

    pub fn increase_copies(&mut self) {
        self.copies += 1;
    }
}

struct CardsSet {
    pub cards: Vec<Card>,
}

impl CardsSet {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn calc_copies(&mut self) -> usize {
        for i in 0..self.cards.len() {
            let card = &self.cards[i];
            let runs = card.get_no_of_copies();

            let wins = card.get_no_of_winning_numbers();
            let start_index = i + 1;
            let end_index = if i + wins <= self.cards.len() - 1 {
                i + wins
            } else {
                self.cards.len() - 1
            };
            for _ in 0..runs {
                for j in start_index..=end_index {
                    let _ = &mut self.cards[j].increase_copies();
                }
            }
        }

        let total_copies = self.cards.iter().fold(0, |acc, x| acc + x.copies);
        return total_copies;
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
    let mut cards_set = CardsSet::new(cards);
    return cards_set.calc_copies();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 30);
    }
}
