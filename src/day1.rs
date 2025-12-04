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
            "L" => current -= distance,
            "R" => current += distance,
            c => {
                panic!("i don't know what to do with {c}");
            }
        }

        current = wrap(current);

        if current == 0 {
            count += 1;
        }
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
        assert_eq!(find_password(lines).expect("never fails"), 3);
    }
}
