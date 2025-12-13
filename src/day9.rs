use std::io::Read as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(',');
            let x = iter
                .next()
                .expect("invalid input: no coordinates on line")
                .parse()
                .expect("invalid input: x is not a number");
            let y = iter
                .next()
                .expect("invalid input: no coordinates on line")
                .parse()
                .expect("invalid input: y is not a number");
            Coord { x, y }
        })
        .collect()
}

fn rectangle_area(a: &'_ Coord, b: &'_ Coord) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn biggest_rectangle(coords: &'_ [Coord]) -> (&'_ Coord, &'_ Coord, i64) {
    coords
        .iter()
        .enumerate()
        .flat_map(|(index, a)| {
            coords[index + 1..]
                .iter()
                .map(move |b| (a, b, rectangle_area(a, b)))
        })
        .max_by(|(_, _, a_area), (_, _, b_area)| a_area.cmp(b_area))
        .expect("should have at least one combination")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let coords = parse_input(&input);

    let (_, _, area) = biggest_rectangle(&coords);

    println!("{area}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Coord;
    use super::biggest_rectangle;
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
        "#
            ),
            vec![
                Coord { x: 7, y: 1 },
                Coord { x: 11, y: 1 },
                Coord { x: 11, y: 7 },
                Coord { x: 9, y: 7 },
                Coord { x: 9, y: 5 },
                Coord { x: 2, y: 5 },
                Coord { x: 2, y: 3 },
                Coord { x: 7, y: 3 },
            ],
        );
    }

    #[test]
    fn test_biggest_rectangle() {
        assert_eq!(
            biggest_rectangle(&[
                Coord { x: 7, y: 1 },
                Coord { x: 11, y: 1 },
                Coord { x: 11, y: 7 },
                Coord { x: 9, y: 7 },
                Coord { x: 9, y: 5 },
                Coord { x: 2, y: 5 },
                Coord { x: 2, y: 3 },
                Coord { x: 7, y: 3 },
            ]),
            (&Coord { x: 2, y: 5 }, &Coord { x: 11, y: 1 }, 50)
        );
    }
}
