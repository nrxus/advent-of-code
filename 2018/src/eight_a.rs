fn solve(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap());

    let mut builder = vec![Node::Header];
    let mut sum = 0;
    while let Some(n) = builder.pop() {
        match n {
            Node::Header => {
                let children = input.next().unwrap();
                let metadata = input.next().unwrap();
                builder.push(Node::Metadata(metadata));
                builder.extend(std::iter::repeat(Node::Header).take(children));
            }
            Node::Metadata(count) => {
                sum += input.by_ref().take(count).sum::<usize>();
            }
        }
    }
    sum
}

#[derive(Clone, Copy)]
enum Node {
    Header,
    Metadata(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(solve(input), 138);
    }
}

common::bootstrap!(8);
