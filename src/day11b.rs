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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct VisitState {
    dac: bool,
    fft: bool,
}

struct Traverse {
    results: HashMap<(VisitState, String, String), usize>,
}

impl Traverse {
    fn new() -> Self {
        Self {
            results: Default::default(),
        }
    }

    fn traverse(
        &mut self,
        graph: &DeviceGraph,
        state: VisitState, // XXX: always true for now
        from: &str,
        to: &str,
    ) -> usize {
        let k = (state, from.to_string(), to.to_string());
        if let Some(cached_result) = self.results.get(&k) {
            return *cached_result;
        }
        let count = self.traverse_impl(graph, state, from, to);
        self.results.insert(k, count);
        count
    }

    fn traverse_impl(
        &mut self,
        graph: &DeviceGraph,
        state: VisitState, // XXX: always true for now
        from: &str,
        to: &str,
    ) -> usize {
        let Some(outputs) = graph.get(from) else {
            panic!("reference to unknown device {from}");
        };

        // Iterate the outputs and recurse

        let mut count = 0;
        for output in outputs {
            if output == to {
                // Collect the path IF the path crosses through "fft" and "dac"
                if state.dac && state.fft {
                    count += 1;
                }
            } else {
                let state = VisitState {
                    dac: state.dac || output == "dac",
                    fft: state.fft || output == "fft",
                };
                // Could cache this, but it takes like 25 microseconds soooo
                count += self.traverse(graph, state, output, to);
            }
        }

        count
    }
}

fn traverse_graph(
    graph: &DeviceGraph,
    state: VisitState, // XXX: always true for now
    from: &str,
    to: &str,
) -> usize {
    Traverse::new().traverse(graph, state, from, to)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let graph = parse_input(&input);

    let s = std::time::Instant::now();
    let result = traverse_graph(
        &graph,
        VisitState {
            dac: false,
            fft: false,
        },
        "svr",
        "out",
    );
    dbg!(s.elapsed());
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::traverse_graph;
    use super::VisitState;
    use std::collections::HashMap;

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
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
        "#,
        );
        assert_eq!(
            traverse_graph(
                &graph,
                VisitState {
                    dac: false,
                    fft: false,
                },
                "svr",
                "out"
            ),
            2
        );
    }
}
