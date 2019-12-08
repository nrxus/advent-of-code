use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let mut graph =
        input
            .trim()
            .lines()
            .fold(HashMap::<&str, Vec<&str>>::new(), |mut graph, edge| {
                let mut edge = edge.split(")");
                let around = edge.next().unwrap();
                let in_orbit = edge.next().unwrap();

                graph
                    .entry(in_orbit)
                    .and_modify(|e| e.push(around))
                    .or_insert_with(|| vec![around]);
                graph
                    .entry(around)
                    .and_modify(|e| e.push(in_orbit))
                    .or_insert_with(|| vec![in_orbit]);

                graph
            });

    let start = graph
        .remove("YOU")
        .expect("YOU was not orbiting anything")
        .pop()
        .unwrap();
    let end = graph
        .remove("SAN")
        .expect("SAN was not orbiting anything")
        .pop()
        .unwrap();

    let mut frontier = vec![(start, 0)];
    let mut explored = HashSet::new();
    explored.insert(start);
    explored.insert("YOU");

    while let Some((body, len)) = frontier.pop() {
        if body == end {
            return len;
        }
        let len = len + 1;

        let new_frontier: Vec<&str> = graph[body]
            .iter()
            .filter(|&b| !explored.contains(b))
            .cloned()
            .collect();

        frontier.extend(new_frontier.iter().map(|&b| (b, len)));
        explored.extend(new_frontier);
    }

    panic!("no path was found from YOU to SAN")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

        assert_eq!(solve(input), 4);
    }
}

common::read_main!();
