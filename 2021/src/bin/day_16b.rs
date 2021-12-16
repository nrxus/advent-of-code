fn solve(input: &str) -> u64 {
    let input: String = input
        .trim()
        .chars()
        .flat_map(|b| format!("{:04b}", b.to_digit(16).unwrap()).into_bytes())
        .map(|b| b as char)
        .collect();

    let mut input = input.as_ref();

    evaluate(&mut input)
}

fn evaluate(bits: &mut &str) -> u64 {
    extract_bits(bits, 3);
    let type_id = read_bits(bits, 3);

    match type_id {
        0 => get_subpackets(bits).into_iter().sum(),
        1 => get_subpackets(bits).into_iter().product(),
        2 => get_subpackets(bits).into_iter().min().unwrap(),
        3 => get_subpackets(bits).into_iter().max().unwrap(),
        4 => read_literal(bits),
        5 => {
            let packets = get_subpackets(bits);
            assert!(packets.len() == 2);
            (packets[0] > packets[1]) as u64
        }
        6 => {
            let packets = get_subpackets(bits);
            assert!(packets.len() == 2);
            (packets[0] < packets[1]) as u64
        }
        7 => {
            let packets = get_subpackets(bits);
            assert!(packets.len() == 2);
            (packets[0] == packets[1]) as u64
        }
        _ => unreachable!(),
    }
}

fn get_subpackets(bits: &mut &str) -> Vec<u64> {
    let length_type = extract_bits(bits, 1);
    if length_type == "0" {
        sum_bitlen_versions(bits)
    } else {
        sum_packetlen_versions(bits)
    }
}

fn sum_packetlen_versions(bits: &mut &str) -> Vec<u64> {
    (0..read_bits(bits, 11)).map(|_| evaluate(bits)).collect()
}

fn sum_bitlen_versions(bits: &mut &str) -> Vec<u64> {
    let bit_length = read_bits(bits, 15);
    let mut subpackets = extract_bits(bits, bit_length as usize);
    let mut results = vec![];
    while !subpackets.is_empty() {
        results.push(evaluate(&mut subpackets));
    }
    results
}

fn read_literal(bits: &mut &str) -> u64 {
    let mut literal = String::new();

    loop {
        let is_last = extract_bits(bits, 1) == "0";
        literal.push_str(extract_bits(bits, 4));
        if is_last {
            break u64::from_str_radix(&literal, 2).unwrap();
        }
    }
}

fn read_bits(bits: &mut &str, len: usize) -> u64 {
    let extracted = extract_bits(bits, len);
    u64::from_str_radix(extracted, 2).unwrap()
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
    fn sum() {
        let input = r"C200B40A82";
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn product() {
        let input = r"04005AC33890";
        assert_eq!(solve(input), 54);
    }

    #[test]
    fn minimum() {
        let input = r"880086C3E88112";
        assert_eq!(solve(input), 7);
    }

    #[test]
    fn maximum() {
        let input = r"CE00C43D881120";
        assert_eq!(solve(input), 9);
    }

    #[test]
    fn less_than() {
        let input = r"D8005AC2A8F0";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn greater_than() {
        let input = r"F600BC2D8F";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn equal() {
        let input = r"9C005AC2F8F0";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn many() {
        let input = r"9C0141080250320F1802104A08";
        assert_eq!(solve(input), 1);
    }
}

common::read_main!();
