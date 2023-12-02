use days::{day_1::{Day1P1, Day1P2}, solution::Solution};
use days::{day_2::{Day2P1, Day2P2}};
mod days;
fn main() {

    let day_one_input = include_str!("input/day_1.txt");

    Day1P1.solve_verbose(day_one_input);

    Day1P2::new().solve_verbose(day_one_input);

    let day_two_input = include_str!("input/day_2.txt");

    Day2P1::new(12, 13, 14).solve_verbose(day_two_input);

    Day2P2::new().solve_verbose(day_two_input);
}