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

fn count_fresh_ingredients(inventory: &Inventory) -> usize {
    inventory
        .available_ids
        .iter()
        .filter(|id| {
            inventory
                .fresh_ranges
                .iter()
                .any(|range| range.contains(id))
        })
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let inventory = parse_input(&input);
    let result = count_fresh_ingredients(&inventory);
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Inventory;
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
    fn test_count_fresh_ingredients() {
        let inventory = Inventory {
            fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
            available_ids: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(count_fresh_ingredients(&inventory), 3);
    }
}
