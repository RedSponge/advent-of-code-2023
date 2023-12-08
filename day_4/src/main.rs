use std::{
    collections::{HashSet, VecDeque},
    fs,
};

struct Card {
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

impl Card {
    ///
    /// Card 1: 10 20 30 40 | 50 60 70 80
    fn parse(line: &str) -> Self {
        let (_header, numbers) = line.split_once(": ").expect("Bad Format");
        let (winnings, yours) = numbers.split_once(" | ").expect("Bad Format");
        let your_numbers = yours
            .split_whitespace()
            .map(|s| s.parse().expect("Not a number!"))
            .collect();
        let winning_numbers = winnings
            .split_whitespace()
            .map(|s| s.parse().expect("Not a number!"))
            .collect();
        Self {
            your_numbers,
            winning_numbers,
        }
    }

    fn number_overlap_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.your_numbers)
            .count()
    }

    fn value(&self) -> usize {
        let overlap = self.number_overlap_count();
        if overlap == 0 {
            0
        } else {
            1 << (overlap - 1)
        }
    }
}

fn compute_card_count(cards: &[Card]) -> usize {
    let mut cards_to_process: VecDeque<usize> = (0..cards.len()).collect();
    let mut cards_processed = 0;
    loop {
        if let Some(card_index) = cards_to_process.pop_front() {
            let overlaps = cards[card_index].number_overlap_count();
            cards_to_process.extend((card_index + 1)..=(card_index + overlaps));
            cards_processed += 1;
        } else {
            break;
        }
    }

    cards_processed
}

fn compute_card_count_from_input(s: &str) -> usize {
    compute_card_count(&s.lines().map(Card::parse).collect::<Vec<_>>())
}

fn compute_winnings(s: &str) -> usize {
    s.lines().map(Card::parse).map(|c| c.value()).sum()
}

fn main() {
    println!(
        "{}",
        compute_winnings(&fs::read_to_string("input.txt").unwrap())
    );
    println!(
        "{}",
        compute_card_count_from_input(&fs::read_to_string("input.txt").unwrap())
    );
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_card() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            card.winning_numbers,
            [41, 48, 83, 86, 17].into_iter().collect()
        );
        assert_eq!(
            card.your_numbers,
            [83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect()
        );
    }

    #[test]
    fn test_overlap_count() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card.number_overlap_count(), 4);
    }

    #[test]
    fn test_card_value() {
        assert_eq!(
            Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").value(),
            8
        );
        assert_eq!(Card::parse("Card 2: 1 2 3 | 4 5 6").value(), 0);
    }

    #[test]
    fn test_compute_winnings() {
        assert_eq!(
            compute_winnings(&fs::read_to_string("example.txt").unwrap()),
            13
        )
    }

    #[test]
    fn test_compute_card_count() {
        assert_eq!(
            compute_card_count_from_input(&fs::read_to_string("example.txt").unwrap()),
            30
        )
    }
}
