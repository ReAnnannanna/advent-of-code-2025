use std::io::Read as _;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    numbers: Vec<i64>,
    operation: Operation,
}

fn parse_input(input: &str) -> impl Iterator<Item = Problem> {
    let mut lines = input.trim().lines();
    let Some(operations_line) = lines.next_back() else {
        panic!("invalid input: zero lines");
    };

    let mut numbers = lines
        .map(|line| line.split_ascii_whitespace())
        .collect::<Vec<_>>();

    operations_line
        .split_ascii_whitespace()
        .map(|column| match column {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("invalid input: unsupported operator {column}"),
        })
        .map(move |operation| Problem {
            numbers: numbers
                .iter_mut()
                .map(|line| {
                    line.next()
                        .expect("invalid input: mismatching line lengths")
                        .parse::<i64>()
                        .expect("invalid input: not a number on a number row")
                })
                .collect(),
            operation,
        })
}

fn solve_problem(problem: Problem) -> i64 {
    match problem.operation {
        Operation::Multiply => problem.numbers.into_iter().product(),
        Operation::Add => problem.numbers.into_iter().sum(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let result: i64 = parse_input(&input).into_iter().map(solve_problem).sum();
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Operation;
    use super::Problem;
    use super::parse_input;
    use super::solve_problem;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
        "#
            )
            .collect::<Vec<_>>(),
            vec![
                Problem {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiply,
                },
                Problem {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Add,
                },
                Problem {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiply,
                },
                Problem {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Add,
                },
            ]
        );
    }

    #[test]
    fn test_solve_problem() {
        assert_eq!(
            solve_problem(Problem {
                numbers: vec![64, 23, 314],
                operation: Operation::Add,
            },),
            401
        );
        assert_eq!(
            solve_problem(Problem {
                numbers: vec![51, 387, 215],
                operation: Operation::Multiply,
            },),
            4243455
        );
    }
}
