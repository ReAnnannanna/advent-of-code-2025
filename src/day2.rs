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
    let ndigits = id.ilog10() + 1;
    if ndigits.is_multiple_of(2) {
        let tenth = 10i64.pow(ndigits / 2);
        let left = id / tenth;
        let right = id % tenth;
        left != right
    } else {
        true
    }
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
        assert!(is_valid_id(1234));
        assert!(!is_valid_id(1212));
        assert!(is_valid_id(12121));
        assert!(is_valid_id(11111));
        assert!(!is_valid_id(1188511885));
    }

    #[test]
    fn test_sum_invalid() {
        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554,
        );
    }
}
