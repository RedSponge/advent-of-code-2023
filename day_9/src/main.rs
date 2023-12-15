use std::fs;

fn compute_diff_pyramid(vals: &[i32]) -> Vec<Vec<i32>> {
    let mut steps: Vec<Vec<i32>> = vec![];
    steps.push(vals.into());
    loop {
        let last_steps = steps.last().expect("Steps can't be empty!");
        if last_steps.iter().all(|&v| v == 0) {
            break steps;
        }
        let mut new_vec = vec![];
        for i in 1..last_steps.len() {
            new_vec.push(last_steps[i] - last_steps[i - 1]);
        }
        steps.push(new_vec);
    }
}

fn extrapolate_history(vals: &[i32]) -> i32 {
    let diff_pyramid = compute_diff_pyramid(vals);
    diff_pyramid
        .iter()
        .rev()
        .map(|v| v.last().expect("All histories shouldn't be empty!"))
        .sum()
}

fn extrapolate_history_backwards(vals: &[i32]) -> i32 {
    let diff_pyramid = compute_diff_pyramid(vals);
    diff_pyramid
        .iter()
        .rev()
        .map(|v| v.first().expect("All histories should be non-zero!"))
        .fold(0, |acc, el| el - acc)
}

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_extrapolation_sum(s: &str) -> i32 {
    parse_input(s).iter().map(|v| extrapolate_history(&v)).sum()
}

fn find_extrapolation_sum_backwards(s: &str) -> i32 {
    parse_input(s)
        .iter()
        .map(|v| extrapolate_history_backwards(&v))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", find_extrapolation_sum_backwards(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pyramid() {
        assert_eq!(
            compute_diff_pyramid(&[0, 3, 6, 9, 12, 15]),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_extrapolate_history() {
        assert_eq!(extrapolate_history(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate_history(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate_history(&[10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(extrapolate_history(&[-1, -4, -7]), -10);
    }

    #[test]
    fn test_find_extrapolation_sum() {
        assert_eq!(
            find_extrapolation_sum(&fs::read_to_string("example.txt").unwrap()),
            114
        )
    }

    #[test]
    fn test_extrapolate_history_backwards() {
        assert_eq!(extrapolate_history_backwards(&[1, 4, 7]), -2);
        assert_eq!(extrapolate_history_backwards(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
