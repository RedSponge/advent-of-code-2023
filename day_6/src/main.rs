struct Game(f64, f64);

impl Game {
    fn total_seconds(&self) -> f64 {
        self.0
    }
    fn distance_to_beat(&self) -> f64 {
        self.1
    }
}

/// Finds the possible wins by representing the question as a function of seconds pressed:
/// distance_traveled(seconds) = seconds * (total_seconds - seconds)
/// Then solves this equation for distance_traveled(seconds) > distance_to_beat:
/// distance_traveled(seconds) > distance_to_beat
/// distance_traveled(seconds) - distance_to_beat > 0
/// -seconds^2 + total_seconds*seconds - distance_to_beat > 0
///
/// Using the quadratic fourmula the intersections with 0 are found, and then it's just a matter of finding how many integers lie between them.
fn count_possible_wins(total_seconds: f64, distance_to_beat: f64) -> u32 {
    let base = total_seconds / 2.0;
    let delta = ((total_seconds.powi(2) - 4.0 * distance_to_beat) / 4.0).sqrt();
    let top = (base + delta).ceil() as u32;
    let bot = (base - delta).floor() as u32;
    top - bot - 1
}

fn find_possible_win_products(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| count_possible_wins(g.total_seconds(), g.distance_to_beat()))
        .product()
}

fn main() {
    println!(
        "{}",
        find_possible_win_products(&[
            Game(48.0, 390.0),
            Game(98.0, 1103.0),
            Game(90.0, 1112.0),
            Game(83.0, 1360.0)
        ])
    );
    println!("{}", count_possible_wins(48989083.0, 390110311121360.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_possible_wins() {
        assert_eq!(count_possible_wins(7.0, 9.0), 4);
        assert_eq!(count_possible_wins(15.0, 40.0), 8);
        assert_eq!(count_possible_wins(30.0, 200.0), 9);
        assert_eq!(count_possible_wins(71530.0, 940200.0), 71503);
    }

    #[test]
    fn test_find_possible_win_products() {
        assert_eq!(
            find_possible_win_products(&[Game(7.0, 9.0), Game(15.0, 40.0), Game(30.0, 200.0),]),
            288
        );
    }
}
