use regex::Regex;

fn solve(rules: &str) -> usize {
    let parent_regex = Regex::new(r"(?P<parent>.*) bags contain (?P<contents>.*)\.").unwrap();

    let mut rules: Vec<_> = rules
        .trim()
        .lines()
        .map(|rule| {
            let captures = parent_regex.captures(rule).unwrap();
            let parent = captures.name("parent").unwrap().as_str();
            let contents = captures.name("contents").unwrap().as_str();
            (parent, contents)
        })
        .collect();

    let mut frontier = vec!["shiny gold bag"];
    let mut found = 0;

    while frontier.len() > 0 {
        let mut new_frontier = vec![];

        rules.retain(|(p, c)| {
            let contains = frontier.iter().any(|b| c.contains(b));
            if contains {
                new_frontier.push(*p);
            }
            !contains
        });

        found += new_frontier.len();
        frontier = new_frontier;
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve(input), 4);
    }
}

common::read_main!();
