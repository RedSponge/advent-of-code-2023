use std::{collections::HashMap, fs};

use gcd::Gcd;
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
        let re = Regex::new(r"(?<from>.{3}) = \((?<left>.{3}), (?<right>.{3})\)")
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
        self.count_simultanious_steps(&[from], &[to]).unwrap()
    }

    fn count_simultanious_steps(&self, froms: &[&str], tos: &[&str]) -> Option<usize> {
        if froms.len() != tos.len() || froms.len() == 0 {
            return None;
        };
        self.count_simultanious_steps_until(froms, |currents| currents == tos)
            .map(|(steps, _ends)| steps)
    }

    fn count_simultanious_steps_until<'a>(
        &'a self,
        froms: &[&'a str],
        mut check_fn: impl FnMut(&[&str]) -> bool,
    ) -> Option<(usize, Vec<&'a str>)> {
        if froms.len() == 0 {
            return None;
        };

        let mut currents: Vec<&str> = froms.into();
        let mut steps = 0;
        loop {
            if check_fn(&currents) {
                return Some((steps, currents));
            }
            currents.iter_mut().for_each(|val| {
                *val = self
                    .map
                    .step(val, self.directions[steps % self.directions.len()])
            });
            steps += 1;
            if steps % 1000000 == 0 {
                println!("{}", steps);
            }
        }
    }
}

fn get_puzzle_loops(puzzle: &Puzzle, starts: &[&str]) -> Result<Vec<usize>, String> {
    let mut res = vec![];
    for start in starts {
        let (steps, z_val) = puzzle
            .count_simultanious_steps_until(&[start], |s| s.iter().all(|l| l.ends_with('Z')))
            .unwrap();
        let z_val = z_val[0];
        let mut first = true;

        let (next_steps, next_z_val) = puzzle
            .count_simultanious_steps_until(&[z_val], |l| {
                if first {
                    first = false;
                    return false;
                }
                l.iter().any(|n| n.ends_with('Z'))
            })
            .unwrap();

        let next_z_val = next_z_val[0];

        if next_z_val != z_val {
            return Err(format!(
                "More than 1 Z on track! Found {} then {}",
                z_val, next_z_val
            ));
        }
        if next_steps != steps {
            return Err(format!(
                "Loop doesn't contain the same steps! First Z after {} and 2nd after {}",
                steps, next_steps
            ));
        }
        if steps % puzzle.directions.len() != 0 {
            return Err(format!("Loop doesn't conform with direction count and so isn't easily computable. Not supported!"));
        }
        res.push(steps)
    }

    Ok(res)
}

fn get_lowest_product(vals: &[usize]) -> u64 {
    let gcd = vals
        .iter()
        .copied()
        .reduce(|a, b| a.gcd(b))
        .map(|v| v as u64)
        .expect("No numbers supplied!");
    vals.iter().map(|&v| v as u64 / gcd).product::<u64>() * gcd
}

fn main() {
    let puzzle = Puzzle::from_str(&fs::read_to_string("input.txt").unwrap()).unwrap();

    let starts: Vec<_> = puzzle
        .map
        .directions
        .keys()
        .filter(|&d| d.ends_with('A'))
        .map(|v| v.as_str())
        .collect();

    let loops = get_puzzle_loops(&puzzle, &starts).unwrap();

    println!("{}", get_lowest_product(&loops));
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
            Puzzle::from_str(&fs::read_to_string("example.txt").unwrap()).expect("Bad Puzzle");
        assert_eq!(puzzle.count_steps("AAA", "ZZZ"), 6)
    }

    #[test]
    fn test_solve_simultanious() {
        let puzzle = Puzzle::from_str(&fs::read_to_string("simultanious_example.txt").unwrap())
            .expect("Bad Puzzle");
        assert_eq!(
            puzzle.count_simultanious_steps(&["11A", "22A"], &["11Z", "22Z"]),
            Some(6)
        )
    }

    #[test]
    fn test_step_until_zs() {
        let puzzle = Puzzle::from_str(&fs::read_to_string("simultanious_example.txt").unwrap())
            .expect("Bad Puzzle");
        assert_eq!(
            puzzle.count_simultanious_steps_until(&["11A", "22A"], |currents| currents
                .iter()
                .all(|l| l.ends_with('Z'))),
            Some((6, vec!["11Z", "22Z"]))
        )
    }
}
