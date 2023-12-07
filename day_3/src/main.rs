use std::{collections::HashMap, fs, ops};

use regex::Regex;

// Positions aren't bound to grid to allow for easy negative index lookup
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position(i32, i32);

impl Position {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        Position(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

struct Schematic {
    symbols: HashMap<Position, char>,
    numbers: HashMap<Position, u32>,
}

fn num_length(mut n: u32) -> usize {
    let mut result = 0;
    loop {
        result += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    result
}

impl Schematic {
    fn find_all_matching<'a, T>(
        s: &'a str,
        re: &Regex,
        proc_function: fn(&'a str) -> T,
    ) -> HashMap<Position, T> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                re.find_iter(line)
                    .map(|mtch| {
                        let x = mtch.start();
                        let value = proc_function(mtch.as_str());
                        (Position(x as i32, y as i32), value)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn parse(s: &str) -> Self {
        let numbers_regex = Regex::new(r"\d+").unwrap();
        let symbols_regex = Regex::new(r"[^\d\.]").unwrap();

        let numbers = Self::find_all_matching(s, &numbers_regex, |n| {
            n.parse::<u32>()
                .expect("Regex should've only captured numbers")
        });

        let symbols = Self::find_all_matching(s, &symbols_regex, |m| {
            m.chars()
                .next()
                .expect("Regex should've caught single characters!")
        });

        Schematic { numbers, symbols }
    }
    fn is_symbol(&self, pos: Position) -> bool {
        self.symbols.contains_key(&pos)
    }

    fn is_next_to_symbol(&self, pos: Position) -> bool {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if self.is_symbol(pos + Position(dx, dy)) {
                    return true;
                }
            }
        }

        false
    }

    fn _is_range_next_to_symbol(&self, pos: Position, len: usize) -> bool {
        (0..len).any(|dx| self.is_next_to_symbol(pos + Position(dx as i32, 0)))
    }

    fn sum_numbers_next_to_symbols(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(&pos, &val)| self._is_range_next_to_symbol(pos, num_length(val)))
            .map(|(&_pos, &val)| val)
            .sum()
    }
}

fn main() {
    let schematic = Schematic::parse(&fs::read_to_string("input.txt").unwrap());
    println!("{}", schematic.sum_numbers_next_to_symbols());
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_schematic() {
        let schematic = Schematic::parse(&fs::read_to_string("example.txt").unwrap());
        assert_eq!(
            schematic.numbers,
            [
                (Position(0, 0), 467),
                (Position(5, 0), 114),
                (Position(2, 2), 35),
                (Position(6, 2), 633),
                (Position(0, 4), 617),
                (Position(7, 5), 58),
                (Position(2, 6), 592),
                (Position(6, 7), 755),
                (Position(1, 9), 664),
                (Position(5, 9), 598),
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            schematic.symbols,
            [
                (Position(3, 1), '*'),
                (Position(6, 3), '#'),
                (Position(3, 4), '*'),
                (Position(5, 5), '+'),
                (Position(3, 8), '$'),
                (Position(5, 8), '*')
            ]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn test_is_next_to_symbol() {
        let schematic = Schematic {
            numbers: HashMap::new(),
            symbols: [(Position(0, 0), '!')].into_iter().collect(),
        };
        assert!(!schematic.is_next_to_symbol(Position(0, 0)));
        assert!(schematic.is_next_to_symbol(Position(0, 1)));
        assert!(schematic.is_next_to_symbol(Position(1, 1)));
        assert!(schematic.is_next_to_symbol(Position(1, 0)));
        assert!(schematic.is_next_to_symbol(Position(-1, 0)));
        assert!(!schematic.is_next_to_symbol(Position(2, 0)));
    }

    #[test]
    fn test_num_length() {
        assert_eq!(num_length(0), 1);
        assert_eq!(num_length(7), 1);
        assert_eq!(num_length(72), 2);
        assert_eq!(num_length(72527), 5);
    }

    #[test]
    fn test_sum_nums_next_to_symbols() {
        let schematic = Schematic::parse(&fs::read_to_string("example.txt").unwrap());
        assert_eq!(schematic.sum_numbers_next_to_symbols(), 4361);
    }
}
