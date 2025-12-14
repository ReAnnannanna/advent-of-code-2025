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

#[derive(Debug, Clone, Copy)]
struct Rect {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

fn segment_intersects_rectangle(rect: Rect, [a, b]: [Coord; 2]) -> bool {
    let (a, b) = (
        Coord {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
        },
        Coord {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        },
    );

    // Segments along the border of the rectangle do not count as "intersecting".
    if a.y == b.y && (a.y == rect.min_y || a.y == rect.max_y) {
        return false;
    }
    if a.x == b.x && (a.x == rect.min_x || a.x == rect.max_x) {
        return false;
    }

    b.x > rect.min_x && a.x < rect.max_x && b.y > rect.min_y && a.y < rect.max_y
}

fn is_fully_covered_rectangle(a: &'_ Coord, b: &'_ Coord, coords: &'_ [Coord]) -> bool {
    let min_x = a.x.min(b.x);
    let min_y = a.y.min(b.y);
    let max_x = a.x.max(b.x);
    let max_y = a.y.max(b.y);

    let rect = Rect {
        min_x,
        min_y,
        max_x,
        max_y,
    };

    let closing_segment = match (coords.first(), coords.last()) {
        (Some(first), Some(last)) => [*last, *first],
        _ => panic!("less than 2 coordinates, so no closing segment"),
    };

    let segments = coords
        .windows(2)
        .chain(std::iter::once(&closing_segment[..]));
    for segment in segments {
        let [a, b] = segment else {
            unreachable!("line segment has 2 elements due to .windows()");
        };

        // If a line segment intersects with the rectangle,
        // *one* side of the segment must be covered, while the other side must be empty.
        // Thus the entire rectangle cannot be covered.
        if segment_intersects_rectangle(rect, [*a, *b]) {
            return false;
        }
    }

    true
}

fn biggest_rectangle(coords: &'_ [Coord]) -> (&'_ Coord, &'_ Coord, i64) {
    // Build a sorted list of all possible rectangles first
    // -> go through in descending order
    // -> return the first one
    let mut rectangles = coords
        .iter()
        .enumerate()
        .flat_map(|(index, a)| {
            coords[index + 1..]
                .iter()
                .map(move |b| (a, b, rectangle_area(a, b)))
        })
        .collect::<Vec<_>>();

    rectangles.sort_by(|(_, _, a_area), (_, _, b_area)| a_area.cmp(b_area));

    *rectangles
        .iter()
        .rfind(|(a, b, _area)| is_fully_covered_rectangle(a, b, coords))
        .expect("should have at least one fully covered rectangle")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let coords = parse_input(&input);

    for slice in coords.windows(2) {
        let a = slice[0];
        let b = slice[1];
        if (a.x - b.x).abs() == 1 || (a.y - b.y).abs() == 1 {
            panic!("my code makes an assumption that this doesn't happen");
        }
    }

    let (_, _, area) = biggest_rectangle(&coords);

    // Old discarded strategy:
    // We don't know the precise area that's covered until the loop is complete.
    // So fill in the entire grid.
    // Iterate from largest rectangle down to the smallest, each time checking if the whole area
    // has tiles.
    // Return as soon as you find one.

    println!("{area}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Coord;
    use super::Rect;
    use super::biggest_rectangle;
    use super::parse_input;
    use super::segment_intersects_rectangle;

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
    fn intersections() {
        // .....#XXX#
        // .....XXXXX
        // #XXXX#XXX#
        let rect = Rect {
            min_x: 0,
            min_y: 0,
            max_x: 9,
            max_y: 2,
        };
        assert!(!segment_intersects_rectangle(
            rect,
            [Coord { x: 5, y: 0 }, Coord { x: 9, y: 0 },]
        ));
        assert!(!segment_intersects_rectangle(
            rect,
            [Coord { x: 9, y: 0 }, Coord { x: 9, y: 2 },]
        ));
        assert!(!segment_intersects_rectangle(
            rect,
            [Coord { x: 0, y: 2 }, Coord { x: 5, y: 2 },]
        ));
        assert!(segment_intersects_rectangle(
            rect,
            [Coord { x: 5, y: 0 }, Coord { x: 5, y: 2 },]
        ));
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
            (&Coord { x: 9, y: 5 }, &Coord { x: 2, y: 3 }, 24)
        );
    }
}
