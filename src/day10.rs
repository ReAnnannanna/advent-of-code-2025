use std::io::Read as _;

#[derive(Debug, Default, PartialEq, Eq)]
struct Machine {
    /// The *desired* indicator status.
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i64>,
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

fn press_button(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut output = state.to_vec();

    for &index in button {
        output[index] = !output[index];
    }

    output
}

fn count_pressed_buttons(state: &[bool]) -> usize {
    state.iter().filter(|pressed| **pressed).count()
}

fn configure_machine(machine: &Machine) -> Option<Vec<bool>> {
    fn recur(
        target_state: &[bool],
        // XXX: could be mutable instead of copying it on each recursion
        current_state: &[bool],
        buttons: &[Vec<usize>],
    ) -> Option<Vec<bool>> {
        if target_state == current_state {
            return Some(vec![false; buttons.len()]);
        }

        let [first, rest @ ..] = buttons else {
            // No more buttons to press.
            return None;
        };

        let pressed_state = press_button(current_state, first);
        match (
            recur(target_state, current_state, rest).map(|mut presseds| {
                presseds.insert(0, false);
                presseds
            }),
            recur(target_state, &pressed_state, rest).map(|mut presseds| {
                presseds.insert(0, true);
                presseds
            }),
        ) {
            (Some(unpressed), Some(pressed)) => {
                if count_pressed_buttons(&unpressed) > count_pressed_buttons(&pressed) {
                    Some(pressed)
                } else {
                    Some(unpressed)
                }
            }
            (Some(presseds), None) | (None, Some(presseds)) => Some(presseds),
            (None, None) => None,
        }
    }

    let starting_state = vec![false; machine.indicators.len()];
    recur(&machine.indicators, &starting_state, &machine.buttons)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(&input);

    let result: usize = machines
        .into_iter()
        .map(|machine| configure_machine(&machine).expect("each machine should have a solution"))
        .map(|solution| solution.into_iter().filter(|pressed| *pressed).count())
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
        assert_eq!(
            configure_machine(&machine),
            Some(vec![false, false, false, false, true, true])
        );

        let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(
            configure_machine(&machine),
            Some(vec![false, false, true, true, true])
        );

        let machine =
            parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(
            configure_machine(&machine),
            Some(vec![false, true, true, false])
        );
    }
}
