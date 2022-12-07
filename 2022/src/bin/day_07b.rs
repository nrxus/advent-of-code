use std::collections::HashMap;

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
                        let size = fs[current].size;
                        current = fs[current].parent.expect("missing parent");
                        fs[current].size += size;
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
                    fs[current].size += size.parse::<usize>().unwrap();
                }
            }
        });

    // add size of current dir all the way back to root
    while let Some(parent) = fs[current].parent {
        fs[parent].size += fs[current].size;
        current = parent;
    }

    let used_space = fs[0].size;
    let Some(min_to_free) = used_space.checked_sub(MAX_USED_SPACE) else {
        return 0;
    };

    let mut sizes: Vec<_> = fs.into_iter().map(|dir| dir.size).collect();
    sizes.sort();
    sizes.into_iter().find(|s| *s >= min_to_free).unwrap()
}

#[derive(Clone, Debug, Default)]
struct Directory<'s> {
    size: usize,
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
