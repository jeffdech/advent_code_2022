use std::collections::HashSet;
use itertools::Itertools;

fn find_n_unique(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .positions(|w| w.iter().collect::<HashSet<_>>().len() == n)
        .map(|idx| idx + n)
        .next()
        .unwrap()
}

fn find_header(s: &str) -> usize {
    find_n_unique(s, 4)
}

fn find_message(s: &str) -> usize {
    find_n_unique(s, 14)
}

fn main() {
    let text = include_str!("input.txt");
    let hidx = find_header(text);
    let midx = find_message(text);
    println!("Header position = {}", hidx);
    println!("Message position = {}", midx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_cases() {
        let inputs = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];

        let outputs = vec![5, 6, 10, 11];

        for (ip, op) in std::iter::zip(inputs, outputs) {
            assert_eq!(find_header(ip), op);
        }
    }

    #[test]
    fn test_message_cases() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        ];

        let outputs = vec![19, 23, 23, 29, 26];

        for (ip, op) in std::iter::zip(inputs, outputs) {
            assert_eq!(find_message(ip), op);
        }
    }
}