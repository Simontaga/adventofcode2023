pub trait Solution {
    fn solve(&self, input: &str);

    fn get_input (&self, input: &str) -> String {
        std::fs::read_to_string(input).unwrap()
    }
}