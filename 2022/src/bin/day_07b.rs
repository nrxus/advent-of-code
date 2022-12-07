use std::collections::{HashMap, VecDeque};

use common::read_main;

fn solve(input: &str) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const NEEDED_SPACE: usize = 30_000_000;
    const MAX_USED_SPACE: usize = TOTAL_SPACE - NEEDED_SPACE;

    let mut fs: Vec<Directory> = vec![Directory::new(None)];
    let mut current: usize = 0;

    // assumes first two lines are always `cd /` and `$ ls`
    input
        .trim()
        .lines()
        .skip(2)
        .filter(|output| *output != "$ ls")
        .for_each(|output| {
            let (first, rest) = output.split_once(' ').unwrap();
            match first {
                "$" => {
                    let (_cd, dir) = rest.split_once(' ').unwrap();
                    if dir == ".." {
                        current = fs[current].parent.expect("missing parent");
                    } else {
                        current = *fs[current].children.get(dir).unwrap();
                    }
                }
                "dir" => {
                    let new_inode = fs.len();
                    fs.push(Directory::new(Some(current)));
                    assert!(fs[current].children.insert(rest, new_inode).is_none());
                }
                size => {
                    fs[current].direct_size += size.parse::<usize>().unwrap();
                }
            }
        });

    let mut sizes = HashMap::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((0, fs[0].clone()));
    while let Some((inode, dir)) = frontier.pop_back() {
        let sum = dir
            .children
            .values()
            .fold(Some(dir.direct_size), |sum, &child| {
                match (sum, sizes.get(&child)) {
                    (_, None) => {
                        frontier.push_back((child, fs[child].clone()));
                        None
                    }
                    (None, Some(_)) => None,
                    (Some(sum), Some(size)) => Some(sum + size),
                }
            });

        match sum {
            Some(s) => {
                sizes.insert(inode, s);
            }
            None => frontier.push_front((inode, dir)),
        }
    }

    let used_space = sizes.get(&0).unwrap();
    let Some(min_to_free) = used_space.checked_sub(MAX_USED_SPACE) else {
        return 0;
    };

    let mut sizes: Vec<_> = sizes.into_values().collect();
    sizes.sort();
    sizes.into_iter().find(|s| *s >= min_to_free).unwrap()
}

#[derive(Clone, Debug, Default)]
struct Directory<'s> {
    direct_size: usize,
    children: HashMap<&'s str, usize>,
    parent: Option<usize>,
}

impl<'s> Directory<'s> {
    pub fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        assert_eq!(solve(input), 24_933_642);
    }
}

read_main!();
