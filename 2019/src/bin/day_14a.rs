use regex::Regex;
use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let line_regex = Regex::new(r"(?P<reactants>.+) => (?P<product>.+)").unwrap();
    let chemical_regex = Regex::new(r"(?P<quantity>\d+) (?P<name>.+)").unwrap();

    let nanofactory: HashMap<_, _> = input
        .trim()
        .lines()
        .map(|l| {
            let captures = line_regex.captures(l).unwrap();
            let reactants: Vec<_> = captures
                .name("reactants")
                .unwrap()
                .as_str()
                .split(",")
                .map(|reactant| {
                    let captures = chemical_regex.captures(reactant.trim()).unwrap();
                    let quantity = captures.name("quantity").unwrap().as_str().parse().unwrap();
                    let name = captures.name("name").unwrap().as_str();
                    Reactant { quantity, name }
                })
                .collect();
            let product = captures.name("product").unwrap().as_str();
            let captures = chemical_regex.captures(product.trim()).unwrap();
            let quantity = captures.name("quantity").unwrap().as_str().parse().unwrap();
            let name = captures.name("name").unwrap().as_str();
            (
                name,
                Formula {
                    quantity,
                    reactants,
                },
            )
        })
        .collect();

    let mut missing: HashMap<_, _> = [("FUEL", 1)].iter().cloned().collect();

    let mut extras: HashMap<&str, usize> = HashMap::new();
    let mut ores_needed = 0;

    while let Some((&chemical_needed, &quantity_needed)) = missing.iter().next() {
        //"pop" from the map of missing chemicals
        //a hack in the absence of a proper pop in HashMap
        missing.remove(chemical_needed).unwrap();

        if chemical_needed == "ORE" {
            ores_needed += quantity_needed;
            continue;
        }

        let formula = &nanofactory[chemical_needed];
        let more_needed = (quantity_needed % formula.quantity) > 0;
        let multiplier = quantity_needed / formula.quantity + more_needed as usize;

        formula
            .reactants
            .iter()
            .map(|r| (r.name, r.quantity * multiplier))
            .filter_map(|(r_name, r_quantity)| match extras.remove(r_name) {
                None => Some((r_name, r_quantity)),
                Some(extra_quantity) => match extra_quantity.cmp(&r_quantity) {
                    std::cmp::Ordering::Equal => None,
                    std::cmp::Ordering::Greater => {
                        extras.insert(r_name, extra_quantity - r_quantity);
                        None
                    }
                    std::cmp::Ordering::Less => Some((r_name, r_quantity - extra_quantity)),
                },
            })
            .for_each(|(name, quantity)| *missing.entry(name).or_insert(0) += quantity);

        if more_needed {
            *extras.entry(chemical_needed).or_insert(0) +=
                formula.quantity * multiplier - quantity_needed;
        }
    }

    ores_needed
}

#[derive(Debug, Clone, Copy)]
struct Reactant<'s> {
    name: &'s str,
    quantity: usize,
}

#[derive(Debug)]
struct Formula<'s> {
    quantity: usize,
    reactants: Vec<Reactant<'s>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let input = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

        assert_eq!(solve(input), 31);

        let input = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

        assert_eq!(solve(input), 165);
    }

    #[test]
    fn medium() {
        let input = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        assert_eq!(solve(input), 13312);

        let input = r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

        assert_eq!(solve(input), 180697);
    }

    #[test]
    fn large() {
        let input = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

        assert_eq!(solve(input), 2210736);
    }
}

common::read_main!();
