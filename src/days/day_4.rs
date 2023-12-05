use crate::days::solution::Solution;
pub struct Day4P1 {}


#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Day4P1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day4P1 {
    fn solve(&self, input: &str) -> u32 {
        let parsed = parse_data(input);
        calculate_day_4(parsed)
    }

    fn get_solution_name(&self) -> &str {
        "Day 4 Part 1"
    }
}

fn parse_data(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let mut card = Card {
            winning_numbers: Vec::new(),
            numbers: Vec::new(),
        };

        let mut line_split = line.split(":");
        let id = line_split.next().unwrap().split(" ");

        let mut numbers = line_split.next().unwrap().split("|");

        let winning_numbers = numbers.next().unwrap().split(" ").collect::<Vec<&str>>();
        let numbers = numbers.next().unwrap().split(" ").collect::<Vec<&str>>();

        for number in winning_numbers {
            if number.trim().is_empty() { continue; }
            card.winning_numbers.push(number.trim().parse::<u8>().unwrap());
        }

        for number in numbers {
            if number.trim().is_empty() { continue; }
            card.numbers.push(number.trim().parse::<u8>().unwrap());
        }

        cards.push(card);
    }

    cards
}

fn calculate_day_4(cards: Vec<Card>) -> u32 {
    let mut sum = 0;

    for card in cards {
        let mut matches: u32 = 0;

        for number in card.winning_numbers {
            if card.numbers.contains(&number) {
                matches += 1;
            }

        }

        if matches == 0 { continue; }
        if matches == 1 {
            sum += 1;
            continue;
        }

        sum += 1 * 2u32.pow(matches-1);
    }

    sum
}

//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_part_1() {
        let day_4 = Day4P1 {};

        let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let parsed = parse_data(test_data);
        assert_eq!(calculate_day_4(parsed), 13);
    }
}