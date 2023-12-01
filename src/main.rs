use days::{day_1::{Day1P1, Day1P2}, solution::Solution};

mod days;

fn main() {
    Day1P1.solve("src/input/day_1.txt");
    Day1P2::new().solve("src/input/day_1.txt");
}
