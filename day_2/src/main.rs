use regex::Regex;
use std::fs;

#[derive(PartialEq, Eq, Debug)]
struct CubeStats {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeStats {
    fn parse_stats_line(line: &str) -> Self {
        // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let re = Regex::new(r"(?<amount>\d+) (?<color>red|green|blue)([,;] )?").unwrap();
        re.captures_iter(line).for_each(|cap| {
            let amount: usize = cap.name("amount").unwrap().as_str().parse().unwrap();
            let color = cap.name("color").unwrap().as_str();
            match color {
                "red" => red = red.max(amount),
                "green" => green = green.max(amount),
                "blue" => blue = blue.max(amount),
                _ => panic!("Bad color text {color}"),
            }
        });

        CubeStats { red, green, blue }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: usize,
    cube_stats: CubeStats,
}

impl Game {
    fn new(id: usize, cube_stats: CubeStats) -> Self {
        Self { id, cube_stats }
    }
    fn parse_game_line(line: &str) -> Self {
        let (header, data) = line.split_once(": ").unwrap();
        let id = header.split_once(' ').unwrap().1.parse().unwrap();
        let cube_stats = CubeStats::parse_stats_line(data);
        Self::new(id, cube_stats)
    }
}

fn is_game_valid(game: &Game, stats: &CubeStats) -> bool {
    game.cube_stats.red <= stats.red
        && game.cube_stats.green <= stats.green
        && game.cube_stats.blue <= stats.blue
}

fn sum_valid_ids(text: &str, valid_stats: &CubeStats) -> usize {
    text.lines()
        .map(Game::parse_game_line)
        .filter(|g| is_game_valid(g, valid_stats))
        .map(|g| g.id)
        .sum()
}

fn main() {
    println!(
        "{}",
        sum_valid_ids(
            &fs::read_to_string("input.txt").unwrap(),
            &CubeStats {
                red: 12,
                green: 13,
                blue: 14
            }
        )
    );
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_cube_stats() {
        assert_eq!(
            CubeStats::parse_stats_line("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            CubeStats {
                red: 4,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game::parse_game_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            Game::new(
                3,
                CubeStats {
                    red: 20,
                    green: 13,
                    blue: 6
                }
            )
        );
    }

    #[test]
    fn test_game_valid() {
        assert!(is_game_valid(
            &Game::parse_game_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            &CubeStats {
                red: 12,
                green: 13,
                blue: 14
            }
        ));
        assert!(!is_game_valid(
            &Game::parse_game_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            &CubeStats {
                red: 12,
                green: 13,
                blue: 14
            }
        ));
    }

    #[test]
    fn test_sum_valid_ids() {
        assert_eq!(
            sum_valid_ids(
                &fs::read_to_string("example.txt").unwrap(),
                &CubeStats {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            ),
            8
        )
    }
}
