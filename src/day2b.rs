use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

fn parse_input(input: &str) -> impl Iterator<Item = Range> {
    input.split(',').map(|pair| {
        let (start, end) = pair.split_once('-').expect("invalid input: not a pair");
        Range {
            start: start.parse().expect("invalid input: start is not numeric"),
            end: end.parse().expect("invalid input: end is not numeric"),
        }
    })
}

/// Returns true if the `id` in base 10 is a sequence of `digits` repeating.
fn is_repeating(mut id: i64, digits: i64, pow: i64) -> bool {
    if pow > id {
        return false;
    }

    while id > 0 {
        if id % pow != digits {
            return false;
        }

        id /= pow;
    }

    true
}

fn is_valid_id(id: i64) -> bool {
    let ndigits = id.ilog10() + 1;
    for n in 1..=ndigits / 2 {
        // Take a `chunk` from the left;
        let chunk = id / 10i64.pow(ndigits - n);
        if is_repeating(id, chunk, 10i64.pow(n)) {
            return false;
        }
    }
    true
}

fn sum_invalid_ids(input: &str) -> i64 {
    parse_input(input)
        .flat_map(|range| range.start..=range.end)
        .filter(|id| !is_valid_id(*id))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let invalid_id_sum = sum_invalid_ids(input.trim());
    println!("{invalid_id_sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Range;
    use super::is_repeating;
    use super::is_valid_id;
    use super::parse_input;
    use super::sum_invalid_ids;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("11-22,95-115,998-1012").collect::<Vec<_>>(),
            vec![
                Range { start: 11, end: 22 },
                Range {
                    start: 95,
                    end: 115
                },
                Range {
                    start: 998,
                    end: 1012
                },
            ],
        );
    }

    #[test]
    fn test_repeating() {
        assert!(!is_repeating(1, 1, 10));
        assert!(is_repeating(11111, 1, 10));
        assert!(!is_repeating(121212, 1, 10));
        assert!(is_repeating(121212, 12, 100));
        assert!(is_repeating(824824824, 824, 1000));
    }

    #[test]
    fn test_valid_id() {
        assert!(is_valid_id(1));
        assert!(is_valid_id(4));
        assert!(is_valid_id(1234));
        assert!(!is_valid_id(1212));
        assert!(is_valid_id(131212));
        assert!(is_valid_id(12121));
        assert!(!is_valid_id(11111));
        assert!(is_valid_id(40404));
        assert!(!is_valid_id(38593859));
        assert!(!is_valid_id(1188511885));
        assert!(is_valid_id(188511885));
        assert!(is_valid_id(118851188));
        assert!(!is_valid_id(2121212121));
    }

    #[test]
    fn test_sum_invalid() {
        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265,
        );
    }
}
