fn solve(input: &str) -> u32 {
    let input: String = input
        .trim()
        .chars()
        .flat_map(|b| format!("{:04b}", b.to_digit(16).unwrap()).into_bytes())
        .map(|b| b as char)
        .collect();

    let mut input = input.as_ref();

    sum_versions(&mut input)
}

fn sum_versions(bits: &mut &str) -> u32 {
    let version = read_bits(bits, 3);
    let type_id = read_bits(bits, 3);

    let sub_versions = if type_id == 4 {
        consume_literal(bits);
        0
    } else {
        let length_type = extract_bits(bits, 1);
        if length_type == "0" {
            sum_bitlen_versions(bits)
        } else {
            sum_packetlen_versions(bits)
        }
    };

    version + sub_versions
}

fn sum_packetlen_versions(bits: &mut &str) -> u32 {
    (0..read_bits(bits, 11)).map(|_| sum_versions(bits)).sum()
}

fn sum_bitlen_versions(bits: &mut &str) -> u32 {
    let bit_length = read_bits(bits, 15);
    let mut subpackets = extract_bits(bits, bit_length as usize);
    let mut sub_versions = 0;
    while !subpackets.is_empty() {
        sub_versions += sum_versions(&mut subpackets);
    }
    sub_versions
}

fn consume_literal(bits: &mut &str) {
    loop {
        let is_last = extract_bits(bits, 1) == "0";
        extract_bits(bits, 4);
        if is_last {
            break;
        }
    }
}

fn read_bits(bits: &mut &str, len: usize) -> u32 {
    let extracted = extract_bits(bits, len);
    u32::from_str_radix(extracted, 2).unwrap()
}

fn extract_bits<'s>(bits: &mut &'s str, len: usize) -> &'s str {
    let len = bits.len().min(len);

    let out: &str = &bits[0..len];
    *bits = &bits[len..];

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = r"D2FE28";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn example_one() {
        let input = r"8A004A801A8002F478";
        assert_eq!(solve(input), 16);
    }

    #[test]
    fn example_two() {
        let input = r"620080001611562C8802118E34";
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn example_three() {
        let input = r"C0015000016115A2E0802F182340";
        assert_eq!(solve(input), 23);
    }

    #[test]
    fn example_four() {
        let input = r"A0016C880162017C3686B18A3D4780";
        assert_eq!(solve(input), 31);
    }
}

common::read_main!();
