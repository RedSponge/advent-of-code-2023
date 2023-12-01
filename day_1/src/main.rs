use std::fs;

const DIGIT_MAPPING: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn find_digit(mut line: &str, scan_forwards: bool) -> Option<u32> {
    while !line.is_empty() {
        let found = DIGIT_MAPPING.iter().find(|(text, _digit)| {
            if scan_forwards {
                line.starts_with(text)
            } else {
                line.ends_with(text)
            }
        });

        if let Some((_text, digit)) = found {
            return Some(*digit);
        }

        if scan_forwards {
            line = &line[1..]
        } else {
            line = &line[..line.len() - 1];
        }
    }

    None
}

fn get_calibration_value(line: &str) -> Option<u32> {
    let first_digit = find_digit(line, true)?;
    let last_digit = find_digit(line, false)?;

    Some(first_digit * 10 + last_digit)
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
        assert_eq!(get_calibration_value("two1nine"), Some(29));
        assert_eq!(get_calibration_value("eightwothree"), Some(83));
        assert_eq!(get_calibration_value("abcone2threexyz"), Some(13));
        assert_eq!(get_calibration_value("xtwone3four"), Some(24));
        assert_eq!(get_calibration_value("4nineeightseven2"), Some(42));
        assert_eq!(get_calibration_value("zoneight234"), Some(14));
        assert_eq!(get_calibration_value("7pqrstsixteen"), Some(76));
        assert_eq!(get_calibration_value("three"), Some(33));
        assert_eq!(
            get_calibration_value("seven8sevenptdlvvgssixvjvzpvsp7fivefourtwoned"),
            Some(71)
        );
    }

    #[test]
    fn test_get_calibration_sum() {
        assert_eq!(
            get_calibration_sum(&fs::read_to_string("example.txt").unwrap()),
            Some(281)
        );
    }
}
