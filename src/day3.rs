use std::io::Read as _;

type Bank = Vec<u8>;

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .trim()
        .lines()
        .map(|bank| bank.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn maximum_bank_joltage(bank: &[u8]) -> u8 {
    bank.iter()
        .enumerate()
        .flat_map(|(index, a)| bank[index + 1..].iter().map(move |b| a * 10 + b))
        .max()
        .expect("there is always at least one entry")
}

fn find_maximum_joltage(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|bank| maximum_bank_joltage(&bank) as i64)
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let result = find_maximum_joltage(&input);
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::find_maximum_joltage;
    use super::maximum_bank_joltage;
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
123
6535
            "#
            ),
            vec![vec![1, 2, 3], vec![6, 5, 3, 5]],
        );
    }

    #[test]
    fn test_maximum_bank_joltage() {
        assert_eq!(
            maximum_bank_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        );
        assert_eq!(
            maximum_bank_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            89
        );
        assert_eq!(
            maximum_bank_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            78
        );
        assert_eq!(
            maximum_bank_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            92
        );
    }

    #[test]
    fn sample() {
        assert_eq!(
            find_maximum_joltage(
                r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#
            ),
            357
        );
    }
}
