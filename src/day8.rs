use std::{collections::HashSet, io::Read as _};

type Distance = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
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
            let z = iter
                .next()
                .expect("invalid input: no coordinates on line")
                .parse()
                .expect("invalid input: z is not a number");
            Coord { x, y, z }
        })
        .collect()
}

fn distance_squared(a: &Coord, b: &Coord) -> Distance {
    (a.x - b.x).abs().pow(2) + (a.y - b.y).abs().pow(2) + (a.z - b.z).abs().pow(2)
}

fn sorted_combinations(coords: &'_ [Coord]) -> Vec<(&'_ Coord, &'_ Coord, Distance)> {
    // build a list of all combinations
    let mut combinations = coords
        .iter()
        .enumerate()
        .flat_map(|(index, a)| {
            coords[index + 1..]
                .iter()
                .map(move |b| (a, b, distance_squared(a, b)))
        })
        .collect::<Vec<_>>();

    // sort list by distance
    combinations.sort_by(|a, b| a.2.cmp(&b.2));

    combinations
}

fn build_circuits<'a>(
    combinations: impl Iterator<Item = &'a (&'a Coord, &'a Coord, Distance)>,
) -> Vec<HashSet<&'a Coord>> {
    let mut circuits: Vec<HashSet<&'a Coord>> = vec![];

    // Note it doesn't merge circuits once they are connected together.
    for (a, b, _) in combinations {
        let a_circuit = circuits.iter().position(|circuit| circuit.contains(a));
        let b_circuit = circuits.iter().position(|circuit| circuit.contains(b));

        match (a_circuit, b_circuit) {
            (Some(a_circuit), Some(b_circuit)) if a_circuit == b_circuit => {
                // do nothing
            }
            (Some(a_circuit), Some(b_circuit)) => {
                let (a_circuit, b_circuit) = (
                    a_circuit.min(b_circuit),
                    a_circuit.max(b_circuit),
                );

                let b_boxes = circuits.remove(b_circuit);
                circuits[a_circuit].extend(b_boxes);
            }
            (Some(a_circuit), None) => {
                circuits[a_circuit].insert(*b);
            }
            (None, Some(b_circuit)) => {
                circuits[b_circuit].insert(*a);
            }
            (None, None) => {
                circuits.push(HashSet::from([*a, *b]));
            }
        }
    }

    circuits.sort_by_key(|circuit| circuit.len());
    circuits
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let coords = parse_input(&input);

    // build a list of all combinations
    let combinations = sorted_combinations(&coords);

    // build circuits in that order
    let circuits = build_circuits(combinations.iter().take(1_000));

    let result = circuits
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.len())
        .product::<usize>();

    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::build_circuits;
    use super::parse_input;
    use super::sorted_combinations;
    use super::Coord;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
        "#
            ),
            vec![
                Coord {
                    x: 162,
                    y: 817,
                    z: 812
                },
                Coord {
                    x: 57,
                    y: 618,
                    z: 57
                },
                Coord {
                    x: 906,
                    y: 360,
                    z: 560
                },
                Coord {
                    x: 592,
                    y: 479,
                    z: 940
                },
                Coord {
                    x: 352,
                    y: 342,
                    z: 300
                },
            ],
        );
    }

    #[test]
    fn test_sorted_combinations() {
        let coords = parse_input(
            r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
        "#,
        );

        let combinations = sorted_combinations(&coords);

        assert_eq!(
            (combinations[0].0, combinations[0].1),
            (
                &Coord {
                    x: 162,
                    y: 817,
                    z: 812
                },
                &Coord {
                    x: 425,
                    y: 690,
                    z: 689
                },
            )
        );
        assert_eq!(
            (combinations[2].0, combinations[2].1),
            (
                &Coord {
                    x: 906,
                    y: 360,
                    z: 560
                },
                &Coord {
                    x: 805,
                    y: 96,
                    z: 715
                },
            )
        );
    }

    #[test]
    fn sample() {
        let coords = parse_input(
            r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
        "#,
        );

        let combinations = sorted_combinations(&coords);

        let circuits = build_circuits(combinations.iter().take(10));
        assert_eq!(
            circuits
                .iter()
                .rev()
                .take(3)
                .map(|circuit| circuit.len())
                .product::<usize>(),
            40
        );
    }
}
