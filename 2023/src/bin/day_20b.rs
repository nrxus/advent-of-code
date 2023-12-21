use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let mut broadcaster: Vec<&str> = vec![];
    let mut out_to_inpus: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut configuration: HashMap<_, _> = input
        .trim()
        .lines()
        .filter_map(|l| {
            let (source, destinations) = l.split_once(" -> ").unwrap();
            let destinations: Vec<_> = destinations.split(", ").collect();
            let module = match &source[0..1] {
                "%" => Module::FlipFlop { state: false },
                "&" => Module::Conjuction {
                    inputs: HashMap::new(),
                },
                _ if source == "broadcaster" => {
                    broadcaster = destinations;
                    return None;
                }
                _ => unreachable!(),
            };

            let name = &source[1..];
            destinations
                .iter()
                .for_each(|&d| out_to_inpus.entry(d).or_insert(vec![]).push(name));
            Some((name, (module, destinations)))
        })
        .collect();

    configuration.iter_mut().for_each(|(name, (module, _))| {
        let Module::Conjuction { inputs } = module else {
            return;
        };
        let Some(found_inputs) = out_to_inpus.get(name) else {
            return;
        };
        *inputs = found_inputs.iter().map(|&name| (name, false)).collect();
    });

    let mut inputs = out_to_inpus.remove("rx").unwrap();
    let rx_input = inputs.pop().unwrap();
    assert!(
        inputs.is_empty(),
        "optimized solution only works on single input rx"
    );
    let Some((Module::Conjuction { inputs }, _)) = configuration.remove(rx_input) else {
        panic!("optimized solution only works if single input is a conjuction");
    };
    let mut needed_highs: HashSet<&str> = inputs.into_iter().map(|(name, _)| name).collect();
    let mut button_nums = HashSet::new();

    'outer: for i in 1.. {
        // button
        let mut pulses: VecDeque<_> = broadcaster
            .iter()
            .map(|&name| ("broadcaster", name, false))
            .collect();

        while let Some((source, name, is_high)) = pulses.pop_front() {
            if is_high {
                if needed_highs.remove(source) {
                    button_nums.insert(i);
                    if needed_highs.is_empty() {
                        break 'outer;
                    }
                }
            }

            let Some((module, outputs)) = configuration.get_mut(name) else {
                continue;
            };

            match module {
                Module::FlipFlop { state } => {
                    if !is_high {
                        *state = !*state;
                        pulses.extend(outputs.iter().map(|&o| (name, o, *state)));
                    }
                }
                Module::Conjuction { inputs } => {
                    *inputs.get_mut(source).unwrap() = is_high;
                    let next = !inputs.values().all(|s| *s);
                    pulses.extend(outputs.iter().map(|&o| (name, o, next)));
                }
            }
        }
    }

    // assume the cycles of buttons are all completely independent
    lcm(button_nums.into_iter())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module<'a> {
    FlipFlop {
        state: bool,
    },
    Conjuction {
        // assumes broadcaster is never the input
        inputs: HashMap<&'a str, bool>,
    },
}

fn lcm(numbers: impl Iterator<Item = usize>) -> usize {
    numbers
        .reduce(|a, b| {
            let gcd = gcd(a, b);
            a / gcd * b
        })
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let remainder = a % b;
        gcd(b, remainder)
    }
}

common::read_main!();
