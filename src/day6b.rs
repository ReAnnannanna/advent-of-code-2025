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
    let mut lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let Some(operations_line) = lines.pop() else {
        panic!("invalid input: zero lines");
    };

    let mut number_lines = lines
        .into_iter()
        .map(|line| line.chars().rev())
        .collect::<Vec<_>>();

    operations_line
        .split_ascii_whitespace()
        .rev()
        .map(|column| match column {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("invalid input: unsupported operator {column}"),
        })
        .map(move |operation| {
            let mut problem = Problem {
                numbers: vec![],
                operation,
            };

            let mut column = String::new();
            loop {
                for it in &mut number_lines {
                    let Some(c) = it.next() else {
                        break;
                    };

                    column.push(c);
                }

                let n = column.trim();
                if n.is_empty() {
                    // Either an empty column (next problem) or no more characters left (end of
                    // problem list).
                    break;
                }

                problem.numbers.push(
                    n.parse()
                        .expect("invalid input: not a number on a number row"),
                );
                column.clear();
            }

            problem
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

    let result: i64 = parse_input(&input).map(solve_problem).sum();
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
                    numbers: vec![4, 431, 623],
                    operation: Operation::Add,
                },
                Problem {
                    numbers: vec![175, 581, 32],
                    operation: Operation::Multiply,
                },
                Problem {
                    numbers: vec![8, 248, 369],
                    operation: Operation::Add,
                },
                Problem {
                    numbers: vec![356, 24, 1],
                    operation: Operation::Multiply,
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
