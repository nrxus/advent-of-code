use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    let (workflows, ratings) = input.trim().split_once("\n\n").unwrap();

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

    ratings
        .lines()
        .map(|l| {
            let line = l
                .strip_prefix('{')
                .and_then(|l| l.strip_suffix('}'))
                .unwrap();
            line.split(',')
                .map(|rating| {
                    let (part, rating) = rating.split_once('=').unwrap();
                    (part, rating.parse::<u16>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .filter(|ratings| {
            let mut next_workflow = "in";
            loop {
                let workflow = match next_workflow {
                    "A" => return true,
                    "R" => return false,
                    part => part,
                };

                let passed_rule = workflows
                    .get(workflow)
                    .unwrap()
                    .iter()
                    .find(|r| match &r.check {
                        Some(check) => {
                            let quantity = ratings.get(check.part).unwrap();
                            match check.op {
                                Operation::GreaterThan => *quantity > check.limit,
                                Operation::LessThan => *quantity < check.limit,
                            }
                        }
                        None => true,
                    })
                    .unwrap();
                next_workflow = passed_rule.destination;
            }
        })
        .flat_map(|r| r.into_values())
        .map(|v| v as u32)
        .sum()
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
    assert_eq!(solve(input), 19114);
}
