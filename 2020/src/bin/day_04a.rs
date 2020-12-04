use std::str::FromStr;

fn solve(map: &str) -> usize {
    map.trim()
        .split("\n\n")
        .filter_map(|p| Passport::from_str(p).ok())
        .count()
}

struct Passport;
struct Error;

impl FromStr for Passport {
    type Err = Error;

    fn from_str(passport: &str) -> Result<Self, Error> {
        let passed = passport
            .split_whitespace()
            .map(|field| field.split(':').next().unwrap())
            .fold([false; 7], |mut r, field| {
                use Requirements::*;

                let index = match field {
                    "byr" => BirthYear,
                    "iyr" => IssueYear,
                    "eyr" => ExpirationYear,
                    "hgt" => Height,
                    "hcl" => HairColor,
                    "ecl" => EyeColor,
                    "pid" => PassportId,
                    "cid" => return r,
                    _ => panic!("whaaaat?"),
                } as usize;

                r[index] = true;

                r
            })
            .iter()
            .all(|&f| f);

        if passed {
            Ok(Passport)
        } else {
            Err(Error)
        }
    }
}

enum Requirements {
    BirthYear = 0,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        assert_eq!(solve(input), 2);
    }
}

common::read_main!();
