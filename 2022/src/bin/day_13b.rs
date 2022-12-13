use common::read_main;

fn solve(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .trim()
        .split("\n\n")
        .flat_map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            [left, right]
        })
        .chain(["[[2]]", "[[6]]"].into_iter())
        .map(|packet| Packet {
            raw: &packet[1..packet.len() - 1],
        })
        .collect();

    packets.sort_unstable();
    let index_one = packets.iter().position(|p| p.raw == "[2]").unwrap() + 1;
    let index_two = packets.iter().position(|p| p.raw == "[6]").unwrap() + 1;
    index_one * index_two
}

#[derive(Debug, Eq)]
struct Packet<'s> {
    raw: &'s str,
}

impl<'s> Ord for Packet<'s> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut me = self.iter();
        let mut other = other.iter();
        let me_iter = me.by_ref();
        let other_iter = other.by_ref();

        loop {
            // we cannot use .zip because we need to know which one
            // exhausted
            let (left, right) = match (me_iter.next(), other_iter.next()) {
                (None, None) => break std::cmp::Ordering::Equal,
                (None, Some(_)) => break std::cmp::Ordering::Less,
                (Some(_), None) => break std::cmp::Ordering::Greater,
                (Some(left), Some(right)) => (left, right),
            };

            let res = match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => a.cmp(&b),
                (Value::Integer(a), Value::List(b)) => {
                    // TODO: make it not allocate
                    let a = a.to_string();
                    let a = Packet { raw: a.as_str() };
                    a.cmp(&b)
                }
                (Value::List(a), Value::Integer(b)) => {
                    // TODO: make it not allocate
                    let b = b.to_string();
                    let b = Packet { raw: b.as_str() };
                    a.cmp(&b)
                }
                (Value::List(a), Value::List(b)) => a.cmp(&b),
            };

            if !res.is_eq() {
                break res;
            }
        }
    }
}

impl<'s> Packet<'s> {
    pub fn iter(&self) -> impl Iterator<Item = Value> + '_ {
        let mut contents = self.raw;

        std::iter::from_fn(move || {
            if contents.is_empty() || &contents[0..1] == "]" {
                return None;
            }
            if &contents[0..1] == "," {
                contents = &contents[1..];
            }
            if &contents[0..1] == "[" {
                contents = &contents[1..];
                let mut unmatched: u8 = 0;
                let end = contents
                    .bytes()
                    .position(|c| {
                        if c == b']' {
                            match unmatched.checked_sub(1) {
                                Some(u) => unmatched = u,
                                None => return true,
                            }
                        } else if c == b'[' {
                            unmatched += 1;
                        }

                        false
                    })
                    .unwrap();
                let (next, rest) = contents.split_at(end);
                contents = &rest[1..];
                Some(Value::List(Packet { raw: next }))
            } else {
                let index = contents
                    .bytes()
                    .position(|c| c == b',')
                    .unwrap_or(contents.len());
                let (next, rest) = contents.split_at(index);

                if rest.is_empty() {
                    contents = rest;
                } else {
                    contents = &rest[1..];
                }

                Some(Value::Integer(next.parse().unwrap()))
            }
        })
    }
}

#[derive(Debug)]
enum Value<'s> {
    Integer(u8),
    List(Packet<'s>),
}

impl<'s> PartialEq for Packet<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl<'s> PartialOrd for Packet<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        assert_eq!(solve(input), 140);
    }
}

read_main!();
