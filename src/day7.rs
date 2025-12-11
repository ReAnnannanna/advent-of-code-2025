use std::io::Read as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Start,
    Empty,
    Splitter,
    Beam,
}

fn parse_input(input: &str) -> Vec<Vec<CellState>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => CellState::Start,
                    '.' => CellState::Empty,
                    '^' => CellState::Splitter,
                    '|' => CellState::Beam,
                    _ => panic!("invalid input: unknown cell state {c}"),
                })
                .collect()
        })
        .collect()
}

fn update_state(prev_line: &[CellState], next_line: &mut [CellState]) -> usize {
    assert_eq!(prev_line.len(), next_line.len());

    let mut split_count = 0;
    for (index, prev_cell) in prev_line.iter().enumerate() {
        let next_cell = next_line[index];
        match (prev_cell, next_cell) {
            (CellState::Beam | CellState::Start, CellState::Splitter) => {
                if index > 0 && next_line[index - 1] == CellState::Empty {
                    next_line[index - 1] = CellState::Beam;
                }
                if let Some(next_state) = next_line.get_mut(index + 1)
                    && *next_state == CellState::Empty
                {
                    *next_state = CellState::Beam;
                }

                split_count += 1;
            }
            (CellState::Beam | CellState::Start, CellState::Empty) => {
                next_line[index] = CellState::Beam;
            }
            (_, CellState::Start) => unreachable!("start must be on the top line"),
            (_, CellState::Beam) => {} // might just have been changed to a beam
            (CellState::Empty | CellState::Splitter, _) => {}
        }
    }

    split_count
}

#[cfg(test)]
fn project_beams(mut grid: Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    for index in 1..grid.len() {
        let [first, second] = grid.get_disjoint_mut([index - 1, index]).unwrap();

        update_state(&*first, second);
    }

    grid
}

fn count_splits(mut grid: Vec<Vec<CellState>>) -> usize {
    let mut split_count = 0;
    for index in 1..grid.len() {
        let [first, second] = grid.get_disjoint_mut([index - 1, index]).unwrap();

        split_count += update_state(&*first, second);
    }

    split_count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let result = count_splits(parse_input(&input));
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CellState;
    use super::count_splits;
    use super::parse_input;
    use super::project_beams;

    #[test]
    fn test_parse_input() {
        use CellState::*;

        assert_eq!(
            parse_input(
                r#"
..S..
.....
..^..
.^.^.
        "#
            ),
            vec![
                vec![Empty, Empty, Start, Empty, Empty],
                vec![Empty, Empty, Empty, Empty, Empty],
                vec![Empty, Empty, Splitter, Empty, Empty],
                vec![Empty, Splitter, Empty, Splitter, Empty],
            ],
        );
    }

    #[test]
    fn test_project_beams() {
        use CellState::*;

        assert_eq!(
            project_beams(vec![
                vec![Empty, Empty, Start, Empty, Empty],
                vec![Empty, Empty, Empty, Empty, Empty],
                vec![Empty, Empty, Splitter, Empty, Empty],
                vec![Empty, Splitter, Empty, Splitter, Empty],
            ]),
            vec![
                vec![Empty, Empty, Start, Empty, Empty],
                vec![Empty, Empty, Beam, Empty, Empty],
                vec![Empty, Beam, Splitter, Beam, Empty],
                vec![Beam, Splitter, Beam, Splitter, Beam],
            ],
        );
    }

    #[test]
    fn test_project_beams_does_not_overwrite_splitters() {
        use CellState::*;

        assert_eq!(
            project_beams(vec![
                vec![Empty, Empty, Start, Empty, Empty],
                vec![Empty, Empty, Empty, Empty, Empty],
                vec![Empty, Empty, Splitter, Splitter, Empty],
                vec![Empty, Splitter, Empty, Splitter, Empty],
            ]),
            vec![
                vec![Empty, Empty, Start, Empty, Empty],
                vec![Empty, Empty, Beam, Empty, Empty],
                vec![Empty, Beam, Splitter, Splitter, Empty],
                vec![Beam, Splitter, Beam, Splitter, Empty],
            ],
        );
    }

    #[test]
    fn sample() {
        let grid = parse_input(
            r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        "#,
        );

        assert_eq!(count_splits(grid), 21);
    }
}
