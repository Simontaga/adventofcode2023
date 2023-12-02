use crate::days::solution::Solution;

pub struct Day2P1 {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

impl Game {
    fn new(i: u32, _rounds: Vec<Round>) -> Self {
        Self {
            id: i,
            rounds: _rounds,
        }
    }
}

impl Solution for Day2P1 {
    fn solve(&self, input: &str) {
        let input = self.get_input(input);
        println!("Total: {}", self.calculate_possible_games(&input));
    }
}

impl Day2P1 {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self {
            max_red: r,
            max_green: g,
            max_blue: b,
        }
    }

    fn calculate_possible_games(&self, input: &str) -> u32 {
        let games = self.parse_games(input);

        let mut total = 0;

        for game in games {

            let mut possible = true;
            for round in game.rounds {
                if round.red_count > self.max_red ||
                    round.green_count > self.max_green ||
                    round.blue_count > self.max_blue {
                    possible = false;
                }
            }

            if possible {
                total += game.id;
            }
        }

        total
    }

    fn parse_games(&self, input: &str) -> Vec<Game> {
        /*
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
         */

        let mut games: Vec<Game> = Vec::new();

        for line in input.lines() {
            let colon_index = line.find(":").unwrap();
            let game_index = line.find("Game").unwrap();
            let game_id = &line[game_index+4..colon_index].trim();
            let game_id = game_id.parse::<u32>().expect("Game id is not a number");


            let game_str = line.get(colon_index+1..).unwrap();



            let mut rounds_result: Vec<Round> = Vec::new();

            let rounds = game_str.split(";");

            for round in rounds {
                let round = round.trim();
                let split = round.split(",");


                let mut current_round = Round {
                    red_count: 0,
                    green_count: 0,
                    blue_count: 0,
                };

                for s in split {
                    let s = s.trim();
                    let split = s.split(" ");
                    let split: Vec<&str> = split.collect();

                    let count = split[0].parse::<u32>().expect("Count is not a number");
                    let color = split.last().expect("Count is not a number");
                    match color {
                        &"red" => { current_round.red_count += count; },
                        &"green" => { current_round.green_count += count; },
                        &"blue" => { current_round.blue_count += count; },
                        _ => { panic!("Unknown color: {}", color);
                        }
                    }
                }

                rounds_result.push(current_round);
            }


            let game = Game::new(game_id, rounds_result);
            games.push(game);
        }

        games
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        let day2 = Day2P1::new(12, 13, 14);

        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(day2.calculate_possible_games(&input), 8);
    }
}