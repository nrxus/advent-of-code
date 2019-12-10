fn solve(input: &str) -> usize {
    process(input, 25, 6)
}

fn process(image: &str, width: usize, height: usize) -> usize {
    let layer_with_most_zeroes = image
        .trim()
        .as_bytes()
        .chunks(width * height)
        .min_by_key(|layer| layer.iter().filter(|&&d| d == b'0').count())
        .unwrap();

    let num_ones = layer_with_most_zeroes
        .iter()
        .filter(|&&d| d == b'1')
        .count();
    let num_twos = layer_with_most_zeroes
        .iter()
        .filter(|&&d| d == b'2')
        .count();

    num_ones * num_twos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "123456789012";
        assert_eq!(process(input, 3, 2), 1);
    }
}

common::read_main!();
