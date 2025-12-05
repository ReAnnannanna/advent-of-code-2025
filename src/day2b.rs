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

fn is_valid_id(id: i64) -> bool {
    let id = id.to_string();
    // Chunk the ID up in slices of digits up to size len/2,
    for n in 1..=id.len() / 2 {
        let mut iter = id.as_bytes().chunks_exact(n);
        let Some(first) = iter.next() else {
            break;
        };

        // and check if all chunks are equal, indicating a repeating number
        if iter.all(|other| other == first) && iter.remainder().is_empty() {
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
    fn test_valid_id() {
        assert!(is_valid_id(1));
        assert!(is_valid_id(4));
        assert!(is_valid_id(1234));
        assert!(!is_valid_id(1212));
        assert!(is_valid_id(131212));
        assert!(is_valid_id(12121));
        assert!(!is_valid_id(11111));
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
