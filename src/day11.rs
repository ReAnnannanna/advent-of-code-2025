use std::collections::HashMap;
use std::io::Read as _;

type DeviceGraph = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> DeviceGraph {
    input
        .trim()
        .lines()
        .map(|line| {
            let Some((device_name, outputs)) = line.split_once(": ") else {
                panic!(r#"invalid input: expected "name: outputs""#);
            };

            (
                device_name.to_string(),
                outputs
                    .split(' ')
                    .map(|output| output.to_string())
                    .collect(),
            )
        })
        .collect()
}

fn traverse_graph(graph: &DeviceGraph, from: &str, to: &str) -> usize {
    let Some(outputs) = graph.get(from) else {
        panic!("reference to unknown device {from}");
    };

    // Iterate the outputs and recurse

    let mut count = 0;
    for output in outputs {
        if output == to {
            // Collect the path
            count += 1;
        } else {
            // Could cache this, but it takes like 25 microseconds soooo
            count += traverse_graph(graph, output, to);
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let graph = parse_input(&input);

    let result = traverse_graph(&graph, "you", "out");
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::parse_input;
    use super::traverse_graph;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(
                r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
        "#
            ),
            HashMap::from([
                (
                    "aaa".to_string(),
                    vec!["you".to_string(), "hhh".to_string()]
                ),
                (
                    "you".to_string(),
                    vec!["bbb".to_string(), "ccc".to_string()]
                ),
                (
                    "bbb".to_string(),
                    vec!["ddd".to_string(), "eee".to_string()]
                ),
            ])
        );
    }

    #[test]
    fn sample() {
        let graph = parse_input(
            r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
        "#,
        );
        assert_eq!(traverse_graph(&graph, "you", "out"), 5);
    }
}
