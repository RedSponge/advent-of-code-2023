use std::{collections::HashMap, fs};

const HAND_SIZE: usize = 5;

#[derive(PartialEq, Eq, Debug)]
struct Hand([CardValue; HAND_SIZE]);

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum CardValue {
    Joker,
    Number(u32),
}

impl CardValue {
    fn from_char(ch: char) -> Self {
        match ch {
            'J' => Self::Joker,
            'A' => Self::Number(14),
            'K' => Self::Number(13),
            'Q' => Self::Number(12),
            'T' => Self::Number(10),
            other => Self::Number(other.to_digit(10).unwrap() as u32),
        }
    }
}

fn get_distinct_counts(vals: &[u32]) -> Vec<usize> {
    if vals.len() == 0 {
        return vec![0];
    }

    let mut collector: HashMap<u32, usize> = HashMap::default();
    vals.iter().for_each(|&v| {
        collector
            .entry(v)
            .and_modify(|n| {
                *n += 1;
            })
            .or_insert(1);
    });
    collector.into_values().collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandVariation {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

impl HandVariation {
    fn from_distinct_counts(counts: &[usize]) -> Self {
        let mut counts: Vec<_> = counts.into();
        counts.sort();

        if counts == vec![5] {
            Self::FiveOAK
        } else if counts == vec![1, 4] {
            Self::FourOAK
        } else if counts == vec![2, 3] {
            Self::FullHouse
        } else if counts == vec![1, 1, 3] {
            Self::ThreeOAK
        } else if counts == vec![1, 2, 2] {
            Self::TwoPairs
        } else if counts == vec![1, 1, 1, 2] {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl Hand {
    fn variation(&self) -> HandVariation {
        let mut numbers = vec![];
        let mut jokers = 0;

        for val in &self.0 {
            match val {
                CardValue::Number(n) => numbers.push(*n),
                CardValue::Joker => jokers += 1,
            }
        }

        let mut counts = get_distinct_counts(&numbers);
        counts.sort();
        *counts.last_mut().unwrap() += jokers;
        HandVariation::from_distinct_counts(&counts)
    }

    fn parse(s: &str) -> Self {
        Self(
            s.chars()
                .map(CardValue::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.variation(), &self.0).cmp(&(other.variation(), &other.0))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn find_total_winnings(s: &str) -> usize {
    let mut hands: Vec<(Hand, usize)> = s
        .lines()
        .map(|l| {
            let (hand_repr, bid) = l.split_once(" ").unwrap();
            (Hand::parse(hand_repr), bid.parse().unwrap())
        })
        .collect();

    // Sort will put worst hands in front, which is great for index * hand bid
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i + 1) * bid)
        .sum()
}

fn main() {
    println!(
        "{}",
        find_total_winnings(&fs::read_to_string("input.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    type CV = CardValue;
    type HV = HandVariation;

    fn assert_eq_sorted(a: &[usize], b: &[usize]) {
        let mut a = Vec::from_iter(a.iter());
        let mut b = Vec::from_iter(b.iter());
        a.sort();
        b.sort();
        assert_eq!(a, b)
    }

    #[test]
    fn test_get_distinct_counts() {
        assert_eq_sorted(&get_distinct_counts(&[1, 2, 3, 4, 5]), &vec![1, 1, 1, 1, 1]);
        assert_eq_sorted(&get_distinct_counts(&[1, 1, 3, 4, 5]), &vec![2, 1, 1, 1]);
        assert_eq_sorted(&get_distinct_counts(&[1, 1, 1, 1, 1]), &vec![5]);
    }

    #[test]
    fn test_hand_variation_from_counts() {
        assert_eq!(HV::from_distinct_counts(&[5]), HV::FiveOAK);
        assert_eq!(HV::from_distinct_counts(&[1, 4]), HV::FourOAK);
        assert_eq!(HV::from_distinct_counts(&[3, 2]), HV::FullHouse);
        assert_eq!(HV::from_distinct_counts(&[3, 1, 1]), HV::ThreeOAK);
        assert_eq!(HV::from_distinct_counts(&[2, 1, 2]), HV::TwoPairs);
        assert_eq!(HV::from_distinct_counts(&[1, 1, 1, 2]), HV::OnePair);
        assert_eq!(HV::from_distinct_counts(&[1, 1, 1, 1, 1]), HV::HighCard);
    }

    #[test]
    fn test_hand_comparison() {
        assert_eq!(Hand::parse("32T3K"), Hand::parse("32T3K"));
        // Test hand variation precedence
        assert!(Hand::parse("22223") > Hand::parse("33445"));

        // Test value precedence
        assert!(Hand::parse("32222") > Hand::parse("22223"));

        // Test Joker is weakest
        assert!(Hand::parse("22222") > Hand::parse("JJJJJ"));
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            Hand::parse("AK9TQ"),
            Hand([
                CV::Number(14),
                CV::Number(13),
                CV::Number(9),
                CV::Number(10),
                CV::Number(12)
            ])
        );
    }

    #[test]
    fn test_find_total_winnings() {
        assert_eq!(
            find_total_winnings(&fs::read_to_string("example.txt").unwrap()),
            5905
        );
    }

    #[test]
    fn test_hand_variation_with_jokers() {
        assert_eq!(Hand::parse("JJJJJ").variation(), HV::FiveOAK);
        assert_eq!(Hand::parse("JJQJJ").variation(), HV::FiveOAK);
        assert_eq!(Hand::parse("1234J").variation(), HV::OnePair);
        assert_eq!(Hand::parse("1334J").variation(), HV::ThreeOAK);
        assert_eq!(Hand::parse("4334J").variation(), HV::FullHouse);
    }
}
