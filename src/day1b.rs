// Track a `current` value starting at 50
// Get input like:
// - L1
// - R764
// - L21
// just - + current then % 100
// count += 1; if `current` = 0

fn wrap(mut current: i32) -> i32 {
    while current < 0 {
        current += 100;
    }
    current % 100
}

fn find_password(
    lines: impl Iterator<Item = std::io::Result<String>>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut current = 50;
    let mut count = 0;

    for line in lines {
        // L123 or R123
        let line = line?;
        let (dir, distance) = line.split_at(1);

        let distance = distance.parse::<i32>()?;

        match dir {
            "L" => {
                // If we were _at_ zero, then a small leftward rotation does not _cross_ zero.
                // If we were at any other number, then a small leftward rotation _does_ cross
                // zero.
                let was_zero = current == 0;

                current -= distance;

                if current <= 0 {
                    count += if was_zero { 0 } else { 1 } + i64::from(current / -100);
                }
            }
            "R" => {
                current += distance;

                if current >= 100 {
                    count += i64::from(current / 100);
                }
            }
            c => {
                panic!("i don't know what to do with {c}");
            }
        }

        current = wrap(current);
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = find_password(std::io::stdin().lines())?;

    println!("{count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_rotations() {
        let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#
        .trim();

        let lines = input.lines().map(|line| Ok(line.to_owned()));
        assert_eq!(find_password(lines).expect("never fails"), 6);
    }

    #[test]
    fn multiple_rotations_rightward() {
        let input = r#"R749"#.trim();

        let lines = input.lines().map(|line| Ok(line.to_owned()));
        assert_eq!(find_password(lines).expect("never fails"), 7);
    }

    #[test]
    fn multiple_rotations_leftward() {
        let input = r#"L749"#.trim();

        let lines = input.lines().map(|line| Ok(line.to_owned()));
        assert_eq!(find_password(lines).expect("never fails"), 7);
    }

    #[test]
    fn multiple_rotations_from_0() {
        let input = r#"
L50
R749
        "#
        .trim();

        let lines = input.lines().map(|line| Ok(line.to_owned()));
        assert_eq!(find_password(lines).expect("never fails"), 8);
    }
}
