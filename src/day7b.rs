use std::fmt::Display;
use std::io::Read as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Start,
    Empty,
    Splitter,
    /// Contains number of timelines in which the beam exists
    Beam(usize),
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Start => f.write_str("S"),
            CellState::Empty => f.write_str("."),
            CellState::Splitter => f.write_str("^"),
            CellState::Beam(n) => write!(f, "{n}"),
        }
    }
}

impl CellState {
    fn add_beams(&mut self, count: usize) {
        match self {
            Self::Empty => *self = Self::Beam(count),
            Self::Beam(base) => *base += count,
            Self::Splitter => {}
            _ => unreachable!("unexpected start state"),
        }
    }
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
                    _ => panic!("invalid input: unknown cell state {c}"),
                })
                .collect()
        })
        .collect()
}

fn update_state(prev_line: &[CellState], next_line: &mut [CellState]) {
    assert_eq!(prev_line.len(), next_line.len());

    for (index, prev_cell) in prev_line.iter().enumerate() {
        let next_cell = next_line[index];
        match (prev_cell, next_cell) {
            (CellState::Start, CellState::Empty) => {
                next_line[index] = CellState::Beam(1);
            }
            (CellState::Beam(count), CellState::Splitter) => {
                if index > 0 {
                    next_line[index - 1].add_beams(*count);
                }
                if let Some(next_state) = next_line.get_mut(index + 1) {
                    next_state.add_beams(*count);
                }
            }
            (_, CellState::Start) => unreachable!("start must be on the top line"),
            (CellState::Beam(count), _) => {
                next_line[index].add_beams(*count);
            }
            (_, CellState::Beam(_)) => {} // might just have been changed to a beam
            (CellState::Start, CellState::Splitter) => {
                unreachable!("start must have at least one empty space")
            }
            (CellState::Empty | CellState::Splitter, _) => {}
        }
    }
}

fn project_beam(mut grid: Vec<Vec<CellState>>) -> Vec<Vec<CellState>> {
    for index in 1..grid.len() {
        let [first, second] = grid.get_disjoint_mut([index - 1, index]).unwrap();

        update_state(&*first, second);
    }

    grid
}

fn count_timelines(grid: Vec<Vec<CellState>>) -> usize {
    let Some(line) = grid.last() else {
        unreachable!("must have at least one line");
    };

    line.iter()
        .map(|cell| match cell {
            CellState::Beam(count) => *count,
            _ => 0,
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let result = count_timelines(project_beam(parse_input(&input)));
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CellState;
    use super::count_timelines;
    use super::parse_input;
    use super::project_beam;
    use super::update_state;

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
    fn test_update_state() {
        use CellState::*;

        let prev_line = [Empty, Empty, Start, Empty, Empty];
        let mut next_line = [Empty, Empty, Empty, Empty, Empty];
        update_state(&prev_line, &mut next_line);
        assert_eq!(next_line, [Empty, Empty, Beam(1), Empty, Empty]);

        let prev_line = [Empty, Empty, Beam(1), Empty, Empty];
        let mut next_line = [Empty, Empty, Splitter, Empty, Empty];
        update_state(&prev_line, &mut next_line);
        assert_eq!(next_line, [Empty, Beam(1), Splitter, Beam(1), Empty]);

        let prev_line = [Empty, Beam(1), Splitter, Beam(1), Empty];
        let mut next_line = [Empty, Splitter, Empty, Splitter, Empty];
        update_state(&prev_line, &mut next_line);
        assert_eq!(next_line, [Beam(1), Splitter, Beam(2), Splitter, Beam(1)]);
    }

    #[test]
    fn sample() {
        let result = count_timelines(project_beam(parse_input(
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
        )));
        assert_eq!(result, 40);
    }
}
