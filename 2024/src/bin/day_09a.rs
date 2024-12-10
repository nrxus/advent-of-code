fn solve(input: &str) -> usize {
    let input = input.trim().as_bytes();
    assert!(input.len() % 2 == 1);

    let mut left = 0;
    let mut right = input.len();
    let mut to_move: Option<(usize, usize)> = None;
    let mut checksum = 0;
    let mut prev_blocks: usize = 0;

    while left < right {
        // add current block to checksum
        let blocks = (input[left] - b'0') as usize;
        prev_blocks += blocks;
        let id_mult = (blocks * (prev_blocks - 1)) - (blocks * blocks - blocks) / 2;
        let id = left / 2;
        checksum += id * id_mult as usize;

        // fill up space
        left += 1;
        if left >= right {
            break;
        };

        let mut space = (input[left] - b'0') as usize;
        while space > 0 {
            let (blocks, id) = to_move.take().unwrap_or_else(|| {
                right -= 1;
                let blocks = (input[right] - b'0') as usize;
                let id = right / 2;
                right -= 1;
                (blocks, id)
            });

            let blocks = if blocks > space {
                let remainder = blocks - space;
                let blocks = std::mem::replace(&mut space, 0);
                to_move = Some((remainder, id));
                blocks
            } else {
                space -= blocks;
                blocks
            };

            prev_blocks += blocks;
            let id_mult = (blocks * (prev_blocks - 1)) - (blocks * blocks - blocks) / 2;
            checksum += id * id_mult as usize;
        }
        left += 1;
    }

    if let Some((blocks, id)) = to_move.take() {
        prev_blocks += blocks;
        let id_mult = (blocks * (prev_blocks - 1)) - (blocks * blocks - blocks) / 2;
        checksum += id * id_mult as usize;
    }

    checksum
}

common::read_main!();

#[test]
fn example() {
    assert_eq!(solve(r"2333133121414131402"), 1928);
}
