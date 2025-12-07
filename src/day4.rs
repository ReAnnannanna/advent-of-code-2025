use std::io::Read as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Paper,
    Empty,
}

type Grid = Vec<Vec<Cell>>;

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    _ => panic!("unexpected {c} in grid"),
                })
                .collect()
        })
        .collect()
}

fn is_accessible(grid: &Grid, row_index: usize, column_index: usize) -> bool {
    // ...
    // .@.
    // ...

    let mut paper_count = 0;

    if let Some(y) = row_index.checked_sub(1) {
        let row = &grid[y];

        if let Some(x) = column_index.checked_sub(1)
            && row[x] == Cell::Paper
        {
            paper_count += 1;
        }
        if row[column_index] == Cell::Paper {
            paper_count += 1;
        }
        if row
            .get(column_index + 1)
            .is_some_and(|&cell| cell == Cell::Paper)
        {
            paper_count += 1;
        }
    }

    let row = &grid[row_index];
    if let Some(x) = column_index.checked_sub(1)
        && row[x] == Cell::Paper
    {
        paper_count += 1;
    }
    if row
        .get(column_index + 1)
        .is_some_and(|&cell| cell == Cell::Paper)
    {
        paper_count += 1;
    }

    if let Some(row) = grid.get(row_index + 1) {
        if let Some(x) = column_index.checked_sub(1)
            && row[x] == Cell::Paper
        {
            paper_count += 1;
        }
        if row[column_index] == Cell::Paper {
            paper_count += 1;
        }
        if row
            .get(column_index + 1)
            .is_some_and(|&cell| cell == Cell::Paper)
        {
            paper_count += 1;
        }
    }

    paper_count < 4
}

fn count_accessible_cells(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(column_index, cell)| (row_index, column_index, *cell))
        })
        .filter(|(row_index, column_index, cell)| {
            *cell == Cell::Paper && is_accessible(grid, *row_index, *column_index)
        })
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(&input);
    let result = count_accessible_cells(&grid);
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Cell;
    use super::is_accessible;
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
.@@.
@.@.
@@@.
..@.
            "#
            ),
            vec![
                vec![Cell::Empty, Cell::Paper, Cell::Paper, Cell::Empty],
                vec![Cell::Paper, Cell::Empty, Cell::Paper, Cell::Empty],
                vec![Cell::Paper, Cell::Paper, Cell::Paper, Cell::Empty],
                vec![Cell::Empty, Cell::Empty, Cell::Paper, Cell::Empty],
            ],
        );
    }

    #[test]
    fn test_is_accessible() {
        let grid = vec![
            vec![Cell::Empty, Cell::Paper, Cell::Paper, Cell::Empty],
            vec![Cell::Paper, Cell::Paper, Cell::Paper, Cell::Empty],
            vec![Cell::Paper, Cell::Paper, Cell::Paper, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Paper, Cell::Empty],
        ];

        assert!(is_accessible(&grid, 0, 0));
        assert!(!is_accessible(&grid, 0, 1));
        assert!(is_accessible(&grid, 2, 0));
        assert!(is_accessible(&grid, 3, 2));
        assert!(!is_accessible(&grid, 1, 1));
    }
}
