use days::{day_1::{Day1P1, Day1P2}, solution::Solution};
use days::{day_2::{Day2P1, Day2P2}};
use days::{day_3::{Day3P1, Day3P2}};
use days::{day_4::{Day4P1}};
mod days;
fn main() {

    let day_one_input = include_str!("input/day_1.txt");

    Day1P1.solve_verbose(day_one_input);

    Day1P2::new().solve_verbose(day_one_input);

    let day_two_input = include_str!("input/day_2.txt");

    Day2P1::new(12, 13, 14).solve_verbose(day_two_input);

    Day2P2::new().solve_verbose(day_two_input);


    let day_three_input = include_str!("input/day_3.txt");

    Day3P1::new().solve_verbose(day_three_input);

    Day3P2::new().solve_verbose(day_three_input);

    let day_four_input = include_str!("input/day_4.txt");

    Day4P1::new().solve_verbose(day_four_input);
}