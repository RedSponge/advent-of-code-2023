use std::{collections::HashMap, fs};

const HAND_SIZE: usize = 5;

#[derive(PartialEq, Eq, Debug)]
struct Hand([u32; HAND_SIZE]);

fn get_distinct_counts(vals: &[u32]) -> Vec<usize> {
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
        HandVariation::from_distinct_counts(&get_distinct_counts(&self.0))
    }

    fn parse(s: &str) -> Self {
        Self(
            s.chars()
                .map(|ch| match ch {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    other => other.to_digit(10).unwrap(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.variation(), self.0).cmp(&(other.variation(), other.0))
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
        assert_eq!(
            HandVariation::from_distinct_counts(&[5]),
            HandVariation::FiveOAK
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[1, 4]),
            HandVariation::FourOAK
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[3, 2]),
            HandVariation::FullHouse
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[3, 1, 1]),
            HandVariation::ThreeOAK
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[2, 1, 2]),
            HandVariation::TwoPairs
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[1, 1, 1, 2]),
            HandVariation::OnePair
        );
        assert_eq!(
            HandVariation::from_distinct_counts(&[1, 1, 1, 1, 1]),
            HandVariation::HighCard
        );
    }

    #[test]
    fn test_hand_comparison() {
        assert_eq!(Hand([3, 2, 10, 3, 13]), Hand([3, 2, 10, 3, 13]));
        // Test hand variation precedence
        assert!(Hand([2, 2, 2, 2, 3]) > Hand([3, 3, 4, 4, 5]));

        // Test value precedence
        assert!(Hand([3, 2, 2, 2, 2]) > Hand([2, 2, 2, 2, 3]));
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(Hand::parse("AK9TQ"), Hand([14, 13, 9, 10, 12]));
    }

    #[test]
    fn test_find_total_winnings() {
        assert_eq!(
            find_total_winnings(&fs::read_to_string("example.txt").unwrap()),
            6440
        );
    }
}
