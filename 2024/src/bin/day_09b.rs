use std::num::NonZero;

fn solve(input: &str) -> usize {
    let input = input.trim().as_bytes();
    assert!(input.len() % 2 == 1);

    let mut files: Vec<File> = Vec::with_capacity(input.len() / 2 + 1);
    let mut spaces: Vec<Space> = Vec::with_capacity(input.len() / 2);

    input
        .chunks(2)
        .enumerate()
        .scan(0_usize, |blocks_so_far, (id, blocks)| {
            let file = NonZero::new(blocks[0] - b'0').map(|size| {
                let location = *blocks_so_far;
                *blocks_so_far += size.get() as usize;
                File { location, size, id }
            });

            let space = blocks
                .get(1)
                .map(|n| n - b'0')
                .and_then(NonZero::new)
                .map(|size| {
                    let location = *blocks_so_far;
                    *blocks_so_far += size.get() as usize;
                    Space {
                        location,
                        size: size.get(),
                    }
                });

            Some((file, space))
        })
        .for_each(|(file, space)| {
            if let Some(file) = file {
                files.push(file);
            }

            if let Some(space) = space {
                spaces.push(space);
            }
        });

    files
        .into_iter()
        .rev()
        .map(|mut file| {
            let space = spaces
                .iter_mut()
                .take_while(|s| s.location < file.location)
                .find(|s| s.size >= file.size.get());

            if let Some(space) = space {
                file.location = space.location;
                space.location = space.location + file.size.get() as usize;
                space.size = space.size - file.size.get();
            }

            file.checksum()
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Space {
    location: usize,
    size: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    location: usize,
    size: NonZero<u8>,
    id: usize,
}

impl File {
    pub fn checksum(self) -> usize {
        let Some(prev) = self.location.checked_sub(1) else {
            return 0;
        };
        if self.location == 0 {
            return 0;
        }
        let size = self.size.get() as usize;
        self.id * (prev * size + (size * size + size) / 2)
    }
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(solve(r"2333133121414131402"), 2858);
}
