use std::io::Read as _;

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

fn press_button(state: &[Joltage], button: &[usize]) -> Vec<Joltage> {
    let mut output = state.to_vec();

    for &index in button {
        output[index] += 1;
    }

    output
}

fn count_pressed_buttons(buttons: &[usize]) -> usize {
    buttons.iter().sum()
}

fn configure_machine(machine: &Machine) -> Option<Vec<usize>> {
    fn recur(
        target_state: &[Joltage],
        // XXX: could be mutable instead of copying it on each recursion
        current_state: &[Joltage],
        buttons: &[Vec<usize>],
    ) -> Option<Vec<usize>> {
        if target_state == current_state {
            return Some(vec![0; buttons.len()]);
        }

        if current_state.iter().zip(target_state).any(|(current, target)| current > target) {
            return None;
        }

        let [first, rest @ ..] = buttons else {
            // No more buttons to press.
            return None;
        };

        let pressed_state = press_button(current_state, first);

        let mut results = vec![];

        results.extend(recur(target_state, current_state, rest).map(|mut presseds| {
            presseds.insert(0, 0); // 0 button presses
            presseds
        }));
        results.extend(recur(target_state, &pressed_state, rest).map(|mut presseds| {
            presseds.insert(0, 1); // 1 button press
            presseds
        }));
        results.extend(recur(target_state, &pressed_state, buttons).map(|mut presseds| {
            presseds[0] += 1; // repeated button press
            presseds
        }));

        results.into_iter()
            .min_by_key(|buttons| count_pressed_buttons(buttons))
    }

    let starting_state = vec![0; machine.indicators.len()];
    recur(&machine.joltages, &starting_state, &machine.buttons)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(&input);

    let result: usize = machines
        .into_iter()
        .map(|machine| configure_machine(&machine).expect("each machine should have a solution"))
        .map(|solution| count_pressed_buttons(&solution))
        .sum();

    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use super::configure_machine;
    use super::parse_machine;
    use super::count_pressed_buttons;

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
            configure_machine(&machine).map(|list| count_pressed_buttons(&list)),
            Some(10),
        );

        let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(
            configure_machine(&machine).map(|list| count_pressed_buttons(&list)),
            Some(12),
        );

        let machine =
            parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(
            configure_machine(&machine).map(|list| count_pressed_buttons(&list)),
            Some(11),
        );
    }

    #[test]
    fn should_not_lock_up() {
        let machine = parse_machine("[..#.##] (0,1,3,4,5) (3) (0,1,3,5) (3,5) (1,5) (0,2,3,5) (0,1,2,3) (0,2,4) {25,12,13,57,14,38}");
        assert_eq!(
            configure_machine(&machine).map(|list| count_pressed_buttons(&list)),
            Some(11),
        );
    }
}

// a * (0,2,3,4) + b * (2,3) + c * (0,4) + d * (0,1,2) + e * (1,2,3,4) = {7,5,12,7,2}
