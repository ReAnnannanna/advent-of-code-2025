use std::{cmp::Ordering, io::Read as _};

type Bank = Vec<u8>;

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .trim()
        .lines()
        .map(|bank| bank.bytes().map(|b| b - b'0').collect())
        .collect()
}

/// Comparator function that ranks the highest, left-most digit as the greatest.
fn compare(
    (left_index, left_digit): &(usize, &u8),
    (right_index, right_digit): &(usize, &u8),
) -> Ordering {
    left_digit
        .cmp(right_digit)
        .then_with(|| left_index.cmp(right_index).reverse())
}

fn maximum_bank_joltage(mut bank: &[u8]) -> i64 {
    let mut joltage = 0;

    // Each iteration, find the highest digit that will still leave us enough digits to complete
    // the sequence.
    for n in (0..12).rev() {
        let (index, digit) = bank[..bank.len() - n]
            .iter()
            .enumerate()
            .max_by(compare)
            .unwrap();

        joltage += *digit as i64 * 10i64.pow(n as u32);

        bank = &bank[index + 1..];
    }

    joltage
}

fn find_maximum_joltage(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|bank| maximum_bank_joltage(&bank))
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
            987654321111
        );
        assert_eq!(
            maximum_bank_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        );
        assert_eq!(
            maximum_bank_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(
            maximum_bank_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
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
            3121910778619
        );
    }
}
