use days::{day_1::{Day1_P1, Day1_P2}, solution::Solution};

mod days;

fn main() {
    Day1_P1.solve("src/input/day_1.txt");
    Day1_P2::new().solve("src/input/day_1.txt");
}
