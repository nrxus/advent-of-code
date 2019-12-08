use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let graph = input
        .trim()
        .lines()
        .fold(HashMap::<&str, &str>::new(), |mut graph, edge| {
            let mut edge = edge.split(")");
            let around = edge.next().unwrap();
            let in_orbit = edge.next().unwrap();

            if let Some(_) = graph.insert(in_orbit, around) {
                panic!("multiple orbits found");
            }

            graph
        });

    let mut orbit_cache = HashMap::new();

    graph
        .keys()
        .map(|body| num_orbits(body, &graph, &mut orbit_cache))
        .sum()
}

fn num_orbits<'s>(
    body: &'s str,
    graph: &HashMap<&'s str, &'s str>,
    cache: &mut HashMap<&'s str, usize>,
) -> usize {
    cache.get(body).cloned().unwrap_or_else(|| {
        let count = graph
            .get(body)
            .map(|around| 1 + num_orbits(around, graph, cache))
            .unwrap_or(0);
        cache.insert(body, count);
        count
    })
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
K)L";

        assert_eq!(solve(input), 42);
    }
}

common::read_main!();
