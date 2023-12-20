use std::{collections::HashMap, ops::Range};

fn solve(input: &str) -> usize {
    let (workflows, _) = input.trim().split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line.split_once('{').unwrap();
            let rules = rules.strip_suffix('}').unwrap();
            let rules: Vec<_> = rules
                .split(',')
                .map(|r| match r.split_once(':') {
                    Some((check, destination)) => {
                        let check = if let Some((part, limit)) = check.split_once('<') {
                            Check {
                                part,
                                op: Operation::LessThan,
                                limit: limit.parse().unwrap(),
                            }
                        } else {
                            let (part, limit) = check.split_once('>').unwrap();
                            Check {
                                part,
                                op: Operation::GreaterThan,
                                limit: limit.parse().unwrap(),
                            }
                        };
                        Rule {
                            destination,
                            check: Some(check),
                        }
                    }
                    None => Rule {
                        destination: r,
                        check: None,
                    },
                })
                .collect();

            (name, rules)
        })
        .collect();

    let range = 1..4001_u16;
    let ranges: HashMap<&str, Range<u16>> = HashMap::from_iter([
        ("x", range.clone()),
        ("m", range.clone()),
        ("a", range.clone()),
        ("s", range.clone()),
    ]);
    let mut universes = vec![(ranges, "in")];
    let mut sum: usize = 0;
    while let Some((mut ranges, workflow)) = universes.pop() {
        let workflow = match workflow {
            "A" => {
                sum += ranges
                    .values()
                    .map(|range| range.end - range.start)
                    .map(|num| num as usize)
                    .product::<usize>();
                continue;
            }
            "R" => continue,
            workflow => workflow,
        };

        for rule in workflows.get(workflow).unwrap() {
            match &rule.check {
                Some(check) => {
                    let range = ranges.get_mut(check.part).unwrap();
                    debug_assert!(range.end > range.start);
                    let true_range = match check.op {
                        Operation::GreaterThan => {
                            let true_range = check.limit + 1..range.end;
                            *range = range.start..check.limit + 1;
                            true_range
                        }
                        Operation::LessThan => {
                            let true_range = range.start..check.limit;
                            *range = check.limit..range.end;
                            true_range
                        }
                    };

                    let valid_false_range = range.end > range.start;
                    if true_range.end > true_range.start {
                        let mut ranges = ranges.clone();
                        ranges.insert(check.part, true_range);
                        universes.push((ranges, rule.destination));
                    }

                    if !valid_false_range {
                        break;
                    }
                }
                None => {
                    universes.push((ranges, rule.destination));
                    break;
                }
            }
        }
    }

    sum
}

#[derive(Debug)]
struct Rule<'s> {
    destination: &'s str,
    check: Option<Check<'s>>,
}

#[derive(Debug)]
struct Check<'s> {
    part: &'s str,
    op: Operation,
    limit: u16,
}

#[derive(Debug)]
enum Operation {
    GreaterThan,
    LessThan,
}

common::read_main!();

#[test]
fn example() {
    let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
    assert_eq!(solve(input), 167409079868000);
}
