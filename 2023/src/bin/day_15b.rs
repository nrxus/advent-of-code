fn solve(input: &str) -> usize {
    const EMPTY_BOX: Vec<(&str, u32)> = Vec::new();
    let mut hash_map = HashMap {
        boxes: [EMPTY_BOX; 256],
    };

    input
        .trim()
        .split(',')
        .for_each(|r| match r.split_once('=') {
            Some((label, value)) => {
                let value = value.parse().unwrap();
                hash_map.insert(label, value);
            }
            None => {
                let label = r.strip_suffix('-').unwrap();
                hash_map.remove(label);
            }
        });

    hash_map
        .boxes
        .into_iter()
        .enumerate()
        .map(|(box_number, bucket)| {
            let box_factor = box_number + 1;
            box_factor
                * bucket
                    .into_iter()
                    .enumerate()
                    .map(|(i, (_, focal_length))| (i + 1) * focal_length as usize)
                    .sum::<usize>()
        })
        .sum()
}

#[derive(Debug)]
struct HashMap<'s, T> {
    boxes: [Vec<(&'s str, T)>; 256],
}

impl<'s, T> HashMap<'s, T> {
    pub fn insert(&mut self, label: &'s str, value: T) {
        let hash = Self::hash(label);
        let bucket = &mut self.boxes[hash as usize];
        match bucket.iter_mut().find(|(old_label, _)| *old_label == label) {
            Some((_, v)) => *v = value,
            None => bucket.push((label, value)),
        }
    }

    pub fn remove(&mut self, label: &'s str) {
        let hash = Self::hash(label);
        let bucket = &mut self.boxes[hash as usize];
        bucket.retain(|(l, _)| *l != label);
    }

    fn hash(label: &str) -> u8 {
        label.bytes().fold(0, |value, next| {
            let mut value = value as u32;
            value += next as u32;
            value *= 17;
            value %= 256;
            value as u8
        })
    }
}

common::read_main!();

#[test]
fn example() {
    let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(solve(input), 145);
}
