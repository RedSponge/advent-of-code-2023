use std::{collections::HashMap, fs};

use regex::Regex;

#[derive(Default, PartialEq, Debug)]
struct Map {
    directions: HashMap<String, (String, String)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

impl Map {
    fn add_direction(
        &mut self,
        from: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) {
        self.directions
            .insert(from.into(), (left.into(), right.into()));
    }

    fn step(&self, from: &str, direction: Direction) -> &str {
        let options = &self.directions[from];
        match direction {
            Direction::Left => &options.0,
            Direction::Right => &options.1,
        }
    }

    /// line format is:
    /// AAA = (BBB, CCC)
    /// START = (LEFT, RIGHT)
    fn add_direction_line(&mut self, line: &str) -> Result<(), ()> {
        if let Some((from, left, right)) = Self::parse_direction_line(line) {
            self.add_direction(from, left, right);
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_direction_line(line: &str) -> Option<(&str, &str, &str)> {
        let re = Regex::new(r"(?<from>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)")
            .expect("Invalid regex!");
        let caps = re.captures(line)?;
        Some((
            caps.name("from").expect("No from").into(),
            caps.name("left").expect("No left").into(),
            caps.name("right").expect("No right").into(),
        ))
    }
}

struct Puzzle {
    map: Map,
    directions: Vec<Direction>,
}

impl Puzzle {
    fn from_str(s: &str) -> Option<Puzzle> {
        let mut lines = s.lines();
        let directions = lines
            .next()?
            .chars()
            .map(Direction::from_char)
            .collect::<Option<Vec<_>>>()?;
        let mut map = Map::default();

        lines.next(); // Discard empty line
        lines
            .map(|l| map.add_direction_line(l).ok())
            .collect::<Option<_>>()?;

        Some(Self { map, directions })
    }

    fn count_steps(&self, from: &str, to: &str) -> usize {
        let mut steps = 0;
        let mut current = from;
        loop {
            if current == to {
                return steps;
            }
            current = self
                .map
                .step(current, self.directions[steps % self.directions.len()]);
            steps += 1;
        }
    }
}

fn main() {
    let puzzle = Puzzle::from_str(&fs::read_to_string("input.txt").unwrap()).unwrap();
    println!("{}", puzzle.count_steps("AAA", "ZZZ"));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_direction_from_char() {
        assert_eq!(Direction::from_char('R'), Some(Direction::Right));
        assert_eq!(Direction::from_char('L'), Some(Direction::Left));
        assert_eq!(Direction::from_char('n'), None);
    }

    #[test]
    fn test_step() {
        let mut map = Map::default();
        map.add_direction("AAA", "BBB", "CCC");
        assert_eq!(map.step("AAA", Direction::Right), "CCC");
        assert_eq!(map.step("AAA", Direction::Left), "BBB");
    }

    #[test]
    fn test_parse_direction_line() {
        assert_eq!(
            Map::parse_direction_line("AAA = (BBB, CCC)"),
            Some(("AAA", "BBB", "CCC"))
        );
        assert_eq!(Map::parse_direction_line("AAA = BBB, CCC)"), None);
    }

    #[test]
    fn test_add_direction() {
        let mut map = Map::default();
        // Add invalid direction:
        assert_eq!(map.add_direction_line("XXX = YYY, ZZZ"), Err(()));

        // Add valid direction
        assert_eq!(map.add_direction_line("AAA = (BBB, CCC)"), Ok(()));
        // Make sure was added:
        assert_eq!(map.step("AAA", Direction::Right), "CCC");
    }

    #[test]
    fn test_parse_puzzle() {
        let puzzle = Puzzle::from_str(&fs::read_to_string("example.txt").unwrap()).unwrap();
        assert_eq!(
            puzzle.directions,
            vec![Direction::Left, Direction::Left, Direction::Right]
        );
        assert_eq!(
            puzzle.map,
            [
                ("AAA", "BBB", "BBB"),
                ("BBB", "AAA", "ZZZ"),
                ("ZZZ", "ZZZ", "ZZZ")
            ]
            .into_iter()
            .fold(Map::default(), |mut map, (from, left, right)| {
                map.add_direction(from, left, right);
                map
            })
        );
    }

    #[test]
    fn test_solve_puzzle() {
        let puzzle =
            Puzzle::from_str(&fs::read_to_string("example.txt").expect("Bad file example.txt"))
                .expect("Bad Puzzle");
        assert_eq!(puzzle.count_steps("AAA", "ZZZ"), 6)
    }
}
