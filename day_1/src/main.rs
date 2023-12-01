use std::fs;

fn get_calibration_value(line: &str) -> Option<u32> {
    let mut iter = line.chars().filter(|ch| ch.is_digit(10));
    let first_digit = iter.next()?;
    let last_digit = iter.last().or(Some(first_digit))?;

    let first_digit = first_digit
        .to_digit(10)
        .expect("Filter should've made this impossible!");
    let last_digit = last_digit
        .to_digit(10)
        .expect("Filter should've made this impossible!");
    return Some(first_digit * 10 + last_digit);
}

fn get_calibration_sum(text: &str) -> Option<u32> {
    text.lines().map(get_calibration_value).sum()
}

fn main() {
    println!(
        "{:?}",
        get_calibration_sum(&fs::read_to_string("input.txt").unwrap())
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(get_calibration_value("1abc2"), Some(12));
        assert_eq!(get_calibration_value("pqr3stu8vwx"), Some(38));
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), Some(15));
        assert_eq!(get_calibration_value("treb7uchet"), Some(77));
        assert_eq!(get_calibration_value("trebuchet"), None);
    }

    #[test]
    fn test_get_calibration_sum() {
        assert_eq!(
            get_calibration_sum(&fs::read_to_string("example.txt").unwrap()),
            Some(142)
        );
    }
}
