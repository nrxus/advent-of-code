use common::read_main;
use md5::Digest;

fn solve(input: &str) -> u32 {
    let input = input.trim();

    (1..)
        .find(|i| {
            let mut hasher = md5::Md5::new();
            hasher.update(format!("{input}{i}"));
            let result = hasher.finalize();
            result[0..3] == [0, 0, 0]
        })
        .unwrap()
}

read_main!();
