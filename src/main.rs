use days::{day_1::{Day1P1, Day1P2}, solution::Solution};
use days::{day_2::{Day2P1}};
mod days;

fn main() {
    Day1P1.solve("src/input/day_1.txt");
    Day1P2::new().solve("src/input/day_1.txt");

    Day2P1::new(12, 13, 14).solve("src/input/day_2.txt");
}