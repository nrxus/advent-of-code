fn solve(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap());

    let mut builder = vec![Node::Header];
    let mut adder = vec![];
    while let Some(n) = builder.pop() {
        match n {
            Node::Header => {
                let children = input.next().unwrap();
                let metadata = input.next().unwrap();
                builder.push(Node::Metadata(metadata));
                builder.extend(std::iter::repeat(Node::Header).take(children));
                if children == 0 {
                    adder.push(NodeAdder::Metadata)
                } else {
                    adder.push(NodeAdder::Children(vec![]));
                }
            }
            Node::Metadata(count) => {
                let metadata = input.by_ref().take(count);
                let value = match adder.pop().unwrap() {
                    NodeAdder::Metadata => metadata.sum::<usize>(),
                    NodeAdder::Children(children) => metadata
                        .filter_map(|i| i.checked_sub(1))
                        .filter_map(|i| children.get(i))
                        .sum(),
                };
                if adder.is_empty() {
                    return value;
                }
                adder
                    .iter_mut()
                    .rev()
                    .find_map(|a| match a {
                        NodeAdder::Metadata => None,
                        NodeAdder::Children(ref mut children) => Some(children),
                    })
                    .unwrap()
                    .push(value);
            }
        }
    }
    0
}

#[derive(Clone, Copy, Debug)]
enum Node {
    Header,
    Metadata(usize),
}

#[derive(Debug)]
enum NodeAdder {
    Children(Vec<usize>),
    Metadata,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(solve(input), 66);
    }
}

common::read_main!();
