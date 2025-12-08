use std::io::Read as _;
use std::ops::RangeInclusive;

#[derive(Debug, Default, PartialEq, Eq)]
struct Inventory {
    fresh_ranges: Vec<RangeInclusive<i64>>,
    available_ids: Vec<i64>,
}

fn parse_input(input: &str) -> Inventory {
    let mut inventory = Inventory::default();
    let mut lines = input.trim().lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (min, max) = line
            .split_once('-')
            .expect("invalid input: range should be int-int");
        let min = min.parse().expect("invalid input: min is not number");
        let max = max.parse().expect("invalid input: max is not number");

        inventory.fresh_ranges.push(min..=max);
    }

    for line in lines {
        let id = line.parse().expect("invalid input: expected numeric id");
        inventory.available_ids.push(id);
    }

    inventory
}

fn combine_fresh_ranges(ranges: &mut Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    ranges.sort_by_key(|range| *range.start());

    let mut iter = ranges.iter();
    let Some(first) = iter.next() else {
        return vec![];
    };
    let mut combined_ranges = vec![first.clone()];

    for range in iter {
        let prev = combined_ranges
            .last_mut()
            .expect("combined_ranges constructed with one element");

        if prev.end() >= range.start() {
            // Avoid lowering the end of the `prev` range - could miss overlaps that way
            if prev.end() < range.end() {
                *prev = *prev.start()..=*range.end();
            }
        } else {
            combined_ranges.push(range.clone());
        }
    }

    combined_ranges
}

fn count_fresh_ingredients(mut inventory: Inventory) -> usize {
    combine_fresh_ranges(&mut inventory.fresh_ranges)
        .into_iter()
        .map(|range| range.count())
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let inventory = parse_input(&input);
    let result = count_fresh_ingredients(inventory);
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Inventory;
    use super::combine_fresh_ranges;
    use super::count_fresh_ingredients;
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
            "#
            ),
            Inventory {
                fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18,],
                available_ids: vec![1, 5, 8, 11, 17, 32],
            },
        );
    }

    #[test]
    fn test_combine_ranges() {
        assert_eq!(
            combine_fresh_ranges(&mut vec![3..=5, 10..=14, 16..=20, 12..=18]),
            vec![3..=5, 10..=20],
        );

        assert_eq!(
            combine_fresh_ranges(&mut vec![10..=20, 11..=12, 19..=22]),
            vec![10..=22],
        );
    }

    #[test]
    fn test_count_fresh_ingredients() {
        let inventory = Inventory {
            fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
            available_ids: vec![],
        };
        assert_eq!(count_fresh_ingredients(inventory), 14);
    }
}
