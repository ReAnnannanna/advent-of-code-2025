use std::io::Read as _;

use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use z3::{Solver, ast::Int};

type Joltage = u16;

#[derive(Debug, Default, PartialEq, Eq)]
struct Machine {
    /// The *desired* indicator status.
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<Joltage>,
}

fn parse_machine(input: &str) -> Machine {
    let mut machine = Machine::default();

    for component in input.split(' ') {
        match component.as_bytes() {
            [b'[', indicators @ .., b']'] => {
                machine.indicators = indicators.iter().map(|c| *c == b'#').collect();
            }
            [b'(', .., b')'] => {
                let component = &component[1..component.len() - 1];
                machine.buttons.push(
                    component
                        .split(',')
                        .map(|index| {
                            index
                                .parse()
                                .expect("invalid input: button wiring not an integer")
                        })
                        .collect(),
                );
            }
            [b'{', .., b'}'] => {
                let component = &component[1..component.len() - 1];
                machine.joltages = component
                    .split(',')
                    .map(|index| {
                        index
                            .parse()
                            .expect("invalid input: joltage not an integer")
                    })
                    .collect();
            }
            _ => panic!("invalid input: unknown component kind"),
        }
    }

    machine
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.trim().lines().map(parse_machine).collect()
}

fn configure_machine(machine: &Machine) -> Option<u64> {
    // a * (0,2,3,4) + b * (2,3) + c * (0,4) + d * (0,1,2) + e * (1,2,3,4) = {7,5,12,7,2}
    // a + b + d = 7
    // d + e = 5
    let solver = Solver::new();

    let button_vars = machine
        .buttons
        .iter()
        .map(|_| Int::fresh_const("button"))
        .collect::<Vec<_>>();

    for var in &button_vars {
        solver.assert(var.ge(0));
    }

    for (index, target_joltage) in machine.joltages.iter().enumerate() {
        // sum = each button that activates this index

        let sum = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, effect)| effect.contains(&index))
            .map(|(bindex, _)| &button_vars[bindex])
            .fold(Int::from_i64(0), |acc, button| acc + button);

        solver.assert(sum.eq(*target_joltage));
    }

    solver
        .solutions(button_vars, false)
        .map(|solution| {
            solution
                .iter()
                .map(|int| int.as_u64().unwrap())
                .sum::<u64>()
        })
        .min()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(&input);

    let result: u64 = machines
        .into_par_iter()
        .map(|machine| {
            dbg!(configure_machine(&machine)).expect("each machine should have a solution")
        })
        .sum();

    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use super::configure_machine;
    use super::parse_machine;

    #[test]
    fn test_parse_machine() {
        assert_eq!(
            parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            Machine {
                indicators: vec![false, true, true, true, false, true],
                buttons: vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2]
                ],
                joltages: vec![10, 11, 11, 5, 10, 5],
            }
        );
    }

    #[test]
    fn test_configure_machine() {
        let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(configure_machine(&machine), Some(10),);

        let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(configure_machine(&machine), Some(12),);

        let machine =
            parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(configure_machine(&machine), Some(11),);
    }
}
