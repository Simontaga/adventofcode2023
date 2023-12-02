use days::{day_1::{Day1P1, Day1P2}, solution::Solution};
use days::{day_2::{Day2P1, Day2P2}};
mod days;
fn main() {

    let day_one_input = include_str!("input/day_1.txt");

    println!("Day 1 Part 1: {}", Day1P1.solve(day_one_input));

    println!("Day 1 Part 2: {}", Day1P2::new().solve(day_one_input));

    let day_two_input = include_str!("input/day_2.txt");

    println!("Day 2 Part 1: {}", Day2P1::new(12, 13, 14).solve(day_two_input));

    println!("Day 2 Part 2: {}", Day2P2::new().solve(day_two_input));
}