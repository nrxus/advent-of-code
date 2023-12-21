use std::collections::{HashMap, VecDeque};

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
        let Some(found_inputs) = out_to_inpus.remove(name) else {
            return;
        };
        *inputs = found_inputs.into_iter().map(|name| (name, false)).collect();
    });

    let original_configuration = configuration.clone();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut previous = HashMap::new();
    previous.insert(0, (0, 0));
    for i in 1..=PUSH_TIMES {
        // button
        low_pulses += 1;
        let mut pulses: VecDeque<_> = broadcaster
            .iter()
            .map(|&name| ("broadcaster", name, false))
            .collect();

        while let Some((source, name, is_high)) = pulses.pop_front() {
            if is_high {
                high_pulses += 1;
            } else {
                low_pulses += 1;
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

        if configuration == original_configuration {
            let remainder = dbg!(PUSH_TIMES % i);
            let multiplier = dbg!(PUSH_TIMES / i);
            let (low_remainder, high_remainder) = previous.remove(&remainder).unwrap();
            low_pulses = low_pulses * multiplier + low_remainder;
            high_pulses = high_pulses * multiplier + high_remainder;

            break;
        }
        previous.insert(i, (low_pulses, high_pulses));
    }

    low_pulses * high_pulses
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

const PUSH_TIMES: usize = 1000;

common::read_main!();

#[test]
fn example_one() {
    let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
    assert_eq!(solve(input), 32000000);
}

#[test]
fn example_two() {
    let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
    assert_eq!(solve(input), 11687500);
}
