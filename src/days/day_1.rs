use std::collections::HashMap;
use crate::days::solution::Solution;
pub struct Day1P1;
pub struct Day1P2 {
    hashmap: HashMap<String, u8>,
}

impl Solution for Day1P1 {
    fn solve(&self, input: &str) -> u32 {
        self.calculate_first_last(&input)
    }
}

impl Solution for Day1P2 {
    fn solve(&self, input: &str) -> u32 {
        self.calculate_first_last(&input)
    }
}


impl Day1P1 {
    fn calculate_first_last(&self, lines: &str) -> u32 {
        let mut total: u32 = 0;
        for line in lines.lines() {
            let combine_digit = self.get_combined_digit(line);
            let combine_digit = combine_digit.parse::<u32>().unwrap();
            total += combine_digit;
        }

        total
    }

    fn get_combined_digit(&self, line: &str) -> String {
        let first_digit = Self::get_first_digit(line);
        let last_digit = Self::get_first_digit(&line.chars().rev().collect::<String>());

        let mut combine_digit = String::new();
        combine_digit.push(first_digit);
        combine_digit.push(last_digit);

        combine_digit
    }


    fn get_first_digit(line: &str) -> char {
        for (_, c) in line.chars().enumerate() {
            if !c.is_numeric() { continue; }
            return c;
        }

        panic!("No digit found in line: {}", line);
    }
}

impl Day1P2 {
    pub fn new() -> Self {
        let hashmap = HashMap::from([
            ("one".to_owned(), 1),
            ("two".to_owned(), 2),
            ("three".to_owned(), 3),
            ("four".to_owned(), 4),
            ("five".to_owned(), 5),
            ("six".to_owned(), 6),
            ("seven".to_owned(), 7),
            ("eight".to_owned(), 8),
            ("nine".to_owned(), 9),
        ]);

        Self {
            hashmap,
        }
    }


    fn calculate_first_last(&self, lines: &str) -> u32 {
        let mut total: u32 = 0;
        for line in lines.lines() {
            let first_digit = self.get_first_digit(line);
            let last_digit = self.get_last_digit(line);
            let combine_digit = format!("{}{}", first_digit, last_digit);
            let combine_digit = combine_digit.parse::<u32>().unwrap();
            total += combine_digit;
        }

        total
    }

    fn get_first_digit(&self, line: &str) -> u8 {

        // Will work 100% of the time 98.9% of the time.
        let mut least_word_index = u8::MAX as usize;
        let mut found_word: u8 = 0;
        for word in &self.hashmap  {
            let index = line.find(word.0);
            if index.is_none() { continue; }
            if index.unwrap() < least_word_index { 
                least_word_index = index.unwrap();
                found_word = *word.1;
             } 
        }

        let mut least_number_index = u8::MAX as usize;
        let mut least_number: u8 = 0;
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() { continue; }
            least_number_index = i;
            least_number = c.to_digit(10).unwrap() as u8;
            break;
        }

        if least_word_index < least_number_index {
            return found_word;
        }

        least_number
    }
    
    fn get_last_digit(&self, line: &str) -> u8 {
        // Oh god.
        let mut highest_word_index = 0;
        let mut found_word_num: u8 = 0;

        for word in &self.hashmap  {
            let matches = line.match_indices(word.0);
            let last_word = matches.last();

            if last_word.is_none() { continue; }

            let (word_index,_) = last_word.unwrap();
            if word_index > highest_word_index {
                highest_word_index = word_index;
                found_word_num = *word.1;
             } 
        }
        
        let mut last_number_index = 0;
        let mut last_number: u8 = 0;
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() { continue; }
            last_number_index = i;
            last_number = c.to_digit(10).unwrap() as u8;
        }

        if highest_word_index > last_number_index {
            return found_word_num;
        }

        last_number
    }
}

// Tests are for the cool kids.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_p1() {
        // Thoroughly testing indeed.
        let day_1_p1 = Day1P1;
        assert_eq!(day_1_p1.calculate_first_last(
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);

    }

    #[test]
    fn test_day_1_part_two_get_first_digit() {
        let day_1_p2 = Day1P2::new();

        assert_eq!(day_1_p2.get_first_digit("threefourfive"), 3);
        assert_eq!(day_1_p2.get_first_digit("sevenfivefournine"), 7);
        assert_eq!(day_1_p2.get_first_digit("ninenineonewhatsyouremergency"), 9);
        assert_eq!(day_1_p2.get_first_digit("1ninenineonewhatsyouremergency"), 1);
        assert_eq!(day_1_p2.get_first_digit("9seveneightnine"), 9);
    }

    #[test]
    fn test_day_1_part_two_get_last_digit() {
        let day_1_p2 = Day1P2::new();

        assert_eq!(day_1_p2.get_last_digit("threefourfive"), 5);
        assert_eq!(day_1_p2.get_last_digit("threefourfive876"), 6);
        assert_eq!(day_1_p2.get_last_digit("seven8ninefoursupper42"), 2);
        assert_eq!(day_1_p2.get_last_digit("seven8ninefoursuppertennineate"), 9);
    }

    #[test]
    fn test_day_1_part_two_test_data() {
        let day_1_p2 = Day1P2::new();

        assert_eq!(day_1_p2.calculate_first_last(
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281);
    }
}