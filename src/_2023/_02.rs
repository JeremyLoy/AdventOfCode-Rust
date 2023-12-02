use crate::_2023::_02::Color::{Blue, Green, Red};
use std::cmp;

pub struct Set {
    blue_count: i32,
    green_count: i32,
    red_count: i32,
}
pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

pub enum Color {
    Blue(i32),
    Green(i32),
    Red(i32),
}

pub fn parse_game(s: &str) -> Option<Game> {
    let (id, sets) = s.split_once(":")?;
    let id = id.split_whitespace();
    let id = id.last()?;
    let id = id.parse::<i32>().ok()?;

    let sets = sets
        .split(";")
        .map(|set| {
            set.split(",")
                .filter_map(|cube| {
                    let cube = cube.trim();
                    let (count, color) = cube.split_once(" ")?;
                    let count = count.parse::<i32>().ok()?;
                    match color {
                        "blue" => Some(Blue(count)),
                        "green" => Some(Green(count)),
                        "red" => Some(Red(count)),
                        _ => None,
                    }
                })
                .fold(
                    Set {
                        blue_count: 0,
                        green_count: 0,
                        red_count: 0,
                    },
                    |mut set, color| {
                        match color {
                            Blue(count) => set.blue_count += count,
                            Green(count) => set.green_count += count,
                            Red(count) => set.red_count += count,
                        }
                        set
                    },
                )
        })
        .collect();

    Some(Game { id, sets })
}

pub fn parse_batch_games(input: impl Iterator<Item = String>) -> Option<Vec<Game>> {
    input.map(|s| parse_game(&s)).collect()
}

pub fn filter_where_impossible(games: Vec<Game>, max: Set) -> Vec<Game> {
    games
        .into_iter()
        .filter(|g| {
            g.sets.iter().all(|s| {
                s.blue_count <= max.blue_count
                    && s.green_count <= max.green_count
                    && s.red_count <= max.red_count
            })
        })
        .collect()
}

pub fn calculate_power(game: &Game) -> i32 {
    let min = Set {
        blue_count: i32::MIN,
        green_count: i32::MIN,
        red_count: i32::MIN,
    };

    let min = game.sets.iter().fold(min, |mut min, set| {
        min.blue_count = cmp::max(min.blue_count, set.blue_count);
        min.green_count = cmp::max(min.green_count, set.green_count);
        min.red_count = cmp::max(min.red_count, set.red_count);
        min
    });

    return min.blue_count * min.green_count * min.red_count;
}

pub fn sum_of_powers(games: Vec<Game>) -> i32 {
    games.iter().map(calculate_power).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::to_lines;
    use crate::input_parsing::Input::{Path, Raw};

    #[test]
    fn test_calculate_power() {
        assert_eq!(
            48,
            calculate_power(
                &parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap()
            )
        );
        assert_eq!(
            12,
            calculate_power(
                &parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                    .unwrap()
            )
        );
        assert_eq!(
            1560,
            calculate_power(
                &parse_game(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                )
                .unwrap()
            )
        );
        assert_eq!(
            630,
            calculate_power(
                &parse_game(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                )
                .unwrap()
            )
        );
        assert_eq!(
            36,
            calculate_power(
                &parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap()
            )
        );
    }

    #[test]
    fn test_1_sample() {
        let input = to_lines(Raw("\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
"));
        let max = Set {
            blue_count: 14,
            green_count: 13,
            red_count: 12,
        };

        let games = parse_batch_games(input).unwrap();
        let games = filter_where_impossible(games, max);

        assert_eq!(games.iter().map(|g| g.id).sum::<i32>(), 8);
    }

    #[test]
    fn test_1() {
        let input = to_lines(Path("input/2023/02.txt"));
        let max = Set {
            blue_count: 14,
            green_count: 13,
            red_count: 12,
        };

        let games = parse_batch_games(input).unwrap();
        let games = filter_where_impossible(games, max);

        assert_eq!(games.iter().map(|g| g.id).sum::<i32>(), 2_239);
    }

    #[test]
    fn test_2_sample() {
        let input = to_lines(Raw("\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
"));

        let games = parse_batch_games(input).unwrap();
        let power_sum = sum_of_powers(games);

        assert_eq!(power_sum, 2_286);
    }

    #[test]
    fn test_2() {
        let input = to_lines(Path("input/2023/02.txt"));

        let games = parse_batch_games(input).unwrap();
        let power_sum = sum_of_powers(games);

        assert_eq!(power_sum, 83_435);
    }
}
