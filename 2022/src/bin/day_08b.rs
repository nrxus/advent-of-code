use common::read_main;

fn solve(input: &str) -> usize {
    let input = input.trim();
    let num_rows = input.lines().count();
    let trees: Vec<_> = input
        .lines()
        .flat_map(|row| row.as_bytes())
        .map(|b| b - b'0')
        .collect();

    let trees = Rectangle {
        data: trees,
        num_rows,
    };

    trees.max_score()
}

struct Rectangle<T> {
    data: Vec<T>,
    num_rows: usize,
}

impl<T: Ord> Rectangle<T> {
    pub fn max_score(&self) -> usize {
        let num_cols = self.num_cols();
        let get_pos = |index: usize| (index % num_cols, index / num_cols);
        self.data
            .iter()
            .enumerate()
            .map(|(index, height)| {
                let (col_index, row_index) = get_pos(index);
                let start_row = index - col_index;
                let left_score = sub_score(self.data[start_row..index].iter().rev(), height);
                let right_score = sub_score(
                    self.data[(index + 1)..(start_row + num_cols)].iter(),
                    height,
                );
                let up_score = sub_score(
                    (0..row_index)
                        .rev()
                        .map(|row_index| &self.data[row_index * num_cols + col_index]),
                    height,
                );
                let down_score = sub_score(
                    ((row_index + 1)..self.num_rows)
                        .map(|row_index| &self.data[row_index * num_cols + col_index]),
                    height,
                );
                left_score * right_score * up_score * down_score
            })
            .max()
            .unwrap()
    }

    pub fn num_cols(&self) -> usize {
        self.data.len() / self.num_rows
    }
}

fn sub_score<T: Ord>(iter: impl Iterator<Item = T>, max: T) -> usize {
    let mut sum = 0;

    for other in iter {
        sum += 1;
        if other >= max {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"30373
25512
65332
33549
35390
";
        assert_eq!(solve(input), 8);
    }
}

read_main!();
