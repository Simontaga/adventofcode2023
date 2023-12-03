use std::collections::HashMap;
use crate::days::solution::Solution;

// This is just magic, don't worry about it.
// Or psychotic code, you decide.

pub struct Day3P1 {}
pub struct Day3P2 {}

impl Day3P1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day3P2 {
    pub fn new() -> Self {
        Self {}
    }
}

// A line contains either a number or a symbol
// (object, (x,y))
// x being the index of the line
// y being the line
#[derive(Debug)]
struct Line {
    objects: HashMap<(i32 , i32),(LineObject)>,
}

#[derive(Debug)]
#[derive(Clone)]
enum LineObject {
    Number(u32, u8),
    Symbol(char),
}

impl Solution for Day3P1 {
    fn solve(&self, input: &str) -> u32 {
        let parsed = parse(input);
        calculate_day_1(parsed)
    }

    fn get_solution_name(&self) -> &str {
        "Day 3 Part 1"
    }
}

impl Solution for Day3P2 {
    fn solve(&self, input: &str) -> u32 {
        let parsed = parse(input);
        calculate_day_2(parsed)
    }

    fn get_solution_name(&self) -> &str {
        "Day 3 Part 2"
    }
}

fn parse(input: &str) -> HashMap<(i32 , i32),(LineObject)> {
    let mut input_lines = input.lines();
    let mut lines: HashMap<(i32 , i32),(LineObject)> = HashMap::new();

    for (y, line) in input_lines.enumerate() {
        let mut number_stack: Vec<(char, i32)> = Vec::new();

        for (x, char) in line.chars().enumerate() {
            // Symbol.
            if char == '.' {
                handle_num_stack(&mut number_stack, y as i32, &mut lines);
                continue;
            }


            if !char.is_digit(10) {
                lines.insert( (x as i32, y as i32), LineObject::Symbol(char));
                handle_num_stack(&mut number_stack, y as i32, &mut lines);
                continue;
            }

            // Number.
           number_stack.push((char, x as i32));
        }
    }

    lines
}

fn handle_num_stack(number_stack: &mut Vec<(char, i32)>, y: i32, lines: &mut HashMap<(i32 , i32),(LineObject)>) {
    if number_stack.is_empty() {
        return;
    }

    let chars = &number_stack.iter().map(|(c, _)| c.clone()).collect::<Vec<char>>();
    let highest_x = &number_stack.iter().map(|(_, x)| x).max().unwrap();
    let val = calculate_number_stack(&chars);
    lines.insert( (**highest_x, y), LineObject::Number(val, chars.len() as u8));
    number_stack.clear();

}

fn calculate_number_stack(number_stack: &Vec<char>) -> u32 {
    let string = number_stack.iter().collect::<String>();
    let number = string.parse::<u32>().expect("Not a number");
    number
}

fn calculate_day_1(objects: HashMap<(i32 , i32),(LineObject)>) -> u32 {
    let mut sum = 0;

    for ((x,y), object) in &objects {

        if let LineObject::Number(n, width) = object {
            for (x, y) in get_3x3_grid(*x, *y, *width) {
                let mut match_found = false;
                match objects.get(&(x, y)) {
                    Some(LineObject::Symbol(c)) => {
                        sum += n;
                        match_found = true;
                    },
                    _ => {
                    }
                }
                if match_found {
                    break;
                }
            }
        }


    }


    sum
}

fn calculate_day_2(objects: HashMap<(i32 , i32),(LineObject)>) -> u32 {
    struct NumberCollision {
        number: u32,
        asterisk: (u32, u32),
    }

    let mut numbers_collided_with_asterisk: Vec<NumberCollision> = Vec::new();

    for ((x,y), object) in &objects {
        if let LineObject::Number(n, width) = object {
            for (x, y) in get_3x3_grid(*x, *y, *width) {
                match objects.get(&(x, y)) {
                    Some(LineObject::Symbol('*')) => {
                        numbers_collided_with_asterisk.push(NumberCollision {
                            number: *n,
                            asterisk: (x as u32, y as u32),
                        });
                    },
                    _ => {}
                }
            }
        }
    }

    let mut sum = 0;

    for number in &numbers_collided_with_asterisk {
        let shared_collisions: Vec<&NumberCollision> = numbers_collided_with_asterisk.iter().filter(|n| n.asterisk == number.asterisk).collect();
        if shared_collisions.len() != 2 { continue; }

        let multiplied = shared_collisions.iter().map(|n| n.number).product::<u32>();
        sum += multiplied;
    }

    // Magic ;)
    sum/2
}

// The offset is the width of the number's text representation.
fn get_3x3_grid(x: i32, y: i32, x_offset: u8) -> Vec<(i32, i32)> {
    let mut grid: Vec<(i32, i32)> = Vec::new();

    for y in (y-1)..(y+2) {
        for x in (x-(x_offset as i32))..(x+2) {
            grid.push((x, y));
        }
    }
    grid
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(calculate_day_1(parse(input)), 4361);
    }

    #[test]
    fn test_day_3_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(calculate_day_2(parse(input)), 467835);
    }
}