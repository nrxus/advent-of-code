use std::collections::HashSet;

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

    let num_cols = trees.num_cols();
    let mut visible = HashSet::new();

    for row_index in 0..num_rows {
        let newly_seen = height_filterer(trees.row(row_index).iter().enumerate())
            .chain(height_filterer(
                trees.row(row_index).iter().enumerate().rev(),
            ))
            .map(|col_index| (col_index, row_index));

        visible.extend(newly_seen);
    }

    for col_index in 0..num_cols {
        let columns = (0..num_rows).map(|row_index| {
            let index = row_index * num_cols + col_index;
            (row_index, &trees.data[index])
        });

        let rev_columns = (0..num_rows)
            .map(|row_index| {
                let index = row_index * num_cols + col_index;
                (row_index, &trees.data[index])
            })
            .rev();

        visible.extend(
            height_filterer(columns)
                .map(|row_index| (col_index, row_index))
                .chain(height_filterer(rev_columns).map(|row_index| (col_index, row_index))),
        )
    }

    visible.len()
}

fn height_filterer<'h>(
    mut heights: impl Iterator<Item = (usize, &'h u8)> + 'h,
) -> impl Iterator<Item = usize> + 'h {
    let first = heights.next().unwrap();
    let mut max_height = first.1;

    heights
        .filter_map(move |(col_index, height)| {
            if height > max_height {
                max_height = height;
                Some(col_index)
            } else {
                None
            }
        })
        .chain(std::iter::once(first.0))
}

struct Rectangle<T> {
    data: Vec<T>,
    num_rows: usize,
}

impl<T> Rectangle<T> {
    pub fn row(&self, row: usize) -> &[T] {
        let num_cols = self.num_cols();
        let start = row * num_cols;
        &self.data[start..(start + num_cols)]
    }

    pub fn num_cols(&self) -> usize {
        self.data.len() / self.num_rows
    }
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
        assert_eq!(solve(input), 21);
    }
}

read_main!();
