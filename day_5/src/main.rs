use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Debug)]
struct RangeTransformation {
    src: usize,
    dst: usize,
    length: usize,
}

impl RangeTransformation {
    fn new(src: usize, dst: usize, length: usize) -> Self {
        Self { src, dst, length }
    }
    fn contains(&self, value: usize) -> bool {
        self.src <= value && value < self.src + self.length
    }
    fn transform(&self, value: usize) -> Option<usize> {
        if self.contains(value) {
            Some(self.dst + (value - self.src))
        } else {
            None
        }
    }
}

struct Almanac {
    mappings: HashMap<String, Vec<RangeTransformation>>,
}

impl Almanac {
    fn new() -> Self {
        Self {
            mappings: HashMap::default(),
        }
    }

    fn add_entry(
        &mut self,
        category: &str,
        dst_range_start: usize,
        src_range_start: usize,
        range_length: usize,
    ) {
        if !self.mappings.contains_key(category) {
            self.mappings.insert(category.to_string(), vec![]);
        }

        let vec = self
            .mappings
            .get_mut(category)
            .expect("Should've been inserted if doesn't exist!");
        vec.push(RangeTransformation::new(
            src_range_start,
            dst_range_start,
            range_length,
        ));
    }

    fn add_entry_line(&mut self, category: &str, line: &str) {
        let mut parts = line.split_whitespace();
        let dst_range_start = parts.next().unwrap().parse().unwrap();
        let src_range_start = parts.next().unwrap().parse().unwrap();
        let range_length = parts.next().unwrap().parse().unwrap();
        self.add_entry(category, dst_range_start, src_range_start, range_length);
    }

    fn apply_transformation(&self, value: usize, transformation: &str) -> usize {
        self.mappings[transformation]
            .iter()
            .filter_map(|t| t.transform(value))
            .next()
            .unwrap_or(value)
    }

    fn compute_value(&self, start_value: usize, transformations: &[&str]) -> usize {
        let mut result = start_value;
        for &t in transformations {
            result = self.apply_transformation(result, t);
        }
        result
    }
}

fn parse_almanac(s: &str) -> (Almanac, Vec<usize>) {
    let mut almanac = Almanac::new();
    let mut seeds = vec![];
    // \r\n\r\n is an ugly hack and I should probably just split iterate over lines.
    for section in s.split("\r\n\r\n") {
        if section.starts_with("seeds: ") {
            seeds = section["seeds: ".len()..]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        } else {
            let mut lines = section.lines();
            let header = lines.next().unwrap();
            if header.ends_with("map:") {
                let category = header.split_once(" ").unwrap().0;
                lines.for_each(|line| almanac.add_entry_line(category, line))
            } else {
                panic!("Unknown Section {}", header)
            }
        }
    }

    (almanac, seeds)
}

fn compute_seed_location(almanac: &Almanac, seed: usize) -> usize {
    almanac.compute_value(
        seed,
        &[
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ],
    )
}

fn find_lowest_seed_from_input(s: &str) -> usize {
    let (almanac, seeds) = parse_almanac(s);
    seeds
        .iter()
        .map(|&seed| compute_seed_location(&almanac, seed))
        .min()
        .expect("No seeds :(")
}

fn main() {
    println!(
        "{}",
        find_lowest_seed_from_input(&fs::read_to_string("input.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_add_entry() {
        let mut almanac = Almanac::new();
        almanac.add_entry("cool", 3, 5, 2);
        assert_eq!(
            almanac.mappings["cool"],
            vec![RangeTransformation::new(5, 3, 2)]
        )
    }

    #[test]
    fn test_add_entry_line() {
        let mut almanac = Almanac::new();
        almanac.add_entry_line("cool", "3 5 2");
        assert_eq!(
            almanac.mappings["cool"],
            vec![RangeTransformation::new(5, 3, 2)]
        )
    }

    #[test]
    fn test_parse_almanac() {
        let (almanac, seeds) = parse_almanac(&fs::read_to_string("example.txt").unwrap());
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            almanac.mappings["humidity-to-location"],
            vec![
                RangeTransformation::new(56, 60, 37),
                RangeTransformation::new(93, 56, 4)
            ]
        )
    }

    #[test]
    fn test_compute_value() {
        let mut almanac = Almanac::new();
        almanac.add_entry("my-category", 0, 2, 2);
        almanac.add_entry("my-second-category", 4, 0, 2);

        // Simple
        assert_eq!(almanac.compute_value(2, &["my-category"]), 0);

        // Chain mappings
        assert_eq!(
            almanac.compute_value(2, &["my-category", "my-second-category"]),
            4
        );

        // No mapping - return same value
        assert_eq!(almanac.compute_value(4, &["my-category"]), 4);
    }

    #[test]
    fn test_compute_seed_location() {
        let (almanac, _seeds) = parse_almanac(&fs::read_to_string("example.txt").unwrap());
        assert_eq!(compute_seed_location(&almanac, 79), 82);
    }

    #[test]
    fn test_find_lowest_seed_from_input() {
        assert_eq!(
            find_lowest_seed_from_input(&fs::read_to_string("example.txt").unwrap()),
            35
        );
    }

    #[test]
    fn test_range_transformation() {
        let r = RangeTransformation::new(1, 5, 2);
        assert!(r.contains(1));
        assert!(r.contains(2));
        assert!(!r.contains(3));
        assert_eq!(r.transform(1), Some(5));
        assert_eq!(r.transform(2), Some(6));
        assert_eq!(r.transform(3), None);
    }
}
