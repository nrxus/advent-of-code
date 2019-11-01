fn solve(input: &str) -> usize {
    length(input.trim(), 50)
}

fn length(input: &str, iterations: usize) -> usize {
    (0..iterations)
        .fold(input.to_string(), |s, _| look_and_say(s.as_str()))
        .len()
}

fn look_and_say(input: &str) -> String {
    input
        .chars()
        .fold(vec![], |mut acc, c| {
            if let Some((ch, count)) = acc.pop() {
                if ch == c {
                    acc.push((ch, count + 1));
                } else {
                    acc.push((ch, count));
                    acc.push((c, 1));
                }
            } else {
                acc.push((c, 1));
            }
            acc
        })
        .into_iter()
        .flat_map(|(ch, count)| vec![std::char::from_digit(count, 10).unwrap(), ch])
        .collect()
}

common::read_main!();
