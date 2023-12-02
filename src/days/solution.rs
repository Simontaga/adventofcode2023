
pub trait Solution {
    fn solve(&self, input: &str) -> u32;

    // Returns the result and the time in milliseconds it took to solve
    fn benchmark_solve(&self, input: &str) -> (u32, u128) {
        let now = std::time::Instant::now();
        let result = self.solve(input);
        let elapsed = now.elapsed();
        (result, elapsed.as_millis())
    }

    fn get_solution_name(&self) -> &str;

    fn solve_verbose(&self, input: &str) -> (u32, u128) {
        let name = self.get_solution_name();
        let (result, elapsed) = self.benchmark_solve(input);
        println!("{} : {}, took {}ms", name, result, elapsed);
        (result, elapsed)
    }
}