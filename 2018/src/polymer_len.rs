use std::ops::Range;

pub fn polymer_len(input: &[u8]) -> usize {
    let mut index = None;
    let mut ranges = vec![];
    let mut building = false;

    for (next, c) in input.iter().enumerate() {
        let i = match index {
            None => {
                if building {
                    building = false;
                    ranges.push(0..next);
                }
                index = Some(next);
                continue;
            }
            Some(i) => i,
        };

        let prev = input[i];
        if prev.eq_ignore_ascii_case(&c) && prev != *c {
            let i = if ranges.last().map(|r: &Range<_>| &r.end) == Some(&i) {
                ranges.pop().unwrap().start
            } else {
                i
            };
            index = i.checked_sub(1);
            building = true;
        } else if building {
            ranges.push(i + 1..next);

            index = Some(next);
            building = false;
        } else {
            index = Some(i + 1);
        }
    }

    if building {
        let start = index.map(|i| i + 1).unwrap_or(0);
        let range = start..input.len();
        ranges.push(range);
    }

    let skip_len: usize = ranges.iter().map(|r| r.end - r.start).sum();

    input.len() - skip_len
}
