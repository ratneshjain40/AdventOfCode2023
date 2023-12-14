use std::{cmp::Ordering, fs::read_to_string};

#[derive(Debug, Clone)]
struct Hand {
    pub hand_type: u16,
    pub hand_value: usize,
    pub bid: usize,
}

impl Hand {
    pub fn compare(&self, other: &Hand) -> Ordering {
        let cp = self.hand_type.cmp(&other.hand_type);
        if cp == Ordering::Equal {
            return self.hand_value.cmp(&other.hand_value);
        } else {
            return cp;
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

fn calculate_hand_type(hand_string: Vec<char>) -> u16 {
    let hand_vec: Vec<char> = hand_string.to_vec();

    let mut char_counts = std::collections::HashMap::new();
    for &ch in &hand_vec {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    let mut values = char_counts.values().collect::<Vec<&usize>>();
    values.sort();
    let values = values.into_iter().rev().collect::<Vec<&usize>>();

    if values[0] == &5 {
        return 7;
    } else if values[0] == &4 {
        return 6;
    } else if values[0] == &3 {
        if values[1] == &2 {
            return 5;
        } else {
            return 4;
        }
    } else if values[0] == &2 {
        if values[1] == &2 {
            return 3;
        } else {
            return 2;
        }
    } else {
        return 1;
    }
}
fn parse_input(lines: Vec<String>) -> Vec<Hand> {
    let symbol_mapping: std::collections::HashMap<char, char> = [
        ('2', '1'),
        ('3', '2'),
        ('4', '3'),
        ('5', '4'),
        ('6', '5'),
        ('7', '6'),
        ('8', '7'),
        ('9', '8'),
        ('T', '9'),
        ('J', 'A'),
        ('Q', 'B'),
        ('K', 'C'),
        ('A', 'D'),
    ]
    .iter()
    .cloned()
    .collect();

    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let crad_transformed = cards
            .chars()
            .map(|x| symbol_mapping.get(&x).unwrap())
            .collect::<String>();
        let hex = usize::from_str_radix(&crad_transformed, 16).unwrap();
        let hand_type = calculate_hand_type(cards.chars().collect::<Vec<char>>());
        let hand_value = hex as usize;
        hands.push(Hand {
            hand_type,
            hand_value,
            bid,
        });
    }
    return hands;
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_7/{}", filename).as_str());
    let mut hands = parse_input(lines);
    hands.sort_by(|a, b| a.compare(b));

    let winnings: Vec<usize> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            return hand.bid * (i + 1);
        }).collect();
    
    return winnings.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 6440);
    }
}
