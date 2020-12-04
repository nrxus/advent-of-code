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
            .filter_map(|field| {
                let mut field = field.split(':');
                let name = field.next().unwrap();
                let name = match name {
                    "byr" => Requirement::BirthYear,
                    "iyr" => Requirement::IssueYear,
                    "eyr" => Requirement::ExpirationYear,
                    "hgt" => Requirement::Height,
                    "hcl" => Requirement::HairColor,
                    "ecl" => Requirement::EyeColor,
                    "pid" => Requirement::PassportId,
                    "cid" => return None,
                    _ => panic!("whaaaat?"),
                };

                let value = field.next().unwrap();
                Some((name, value))
            })
            .fold([false; 7], |mut r, (name, value)| {
                r[name as usize] = match name {
                    Requirement::BirthYear => {
                        let year: u16 = value.parse().unwrap();
                        year >= 1920 && year <= 2002
                    }
                    Requirement::IssueYear => {
                        let year: u16 = value.parse().unwrap();
                        year >= 2010 && year <= 2020
                    }
                    Requirement::ExpirationYear => {
                        let year: u16 = value.parse().unwrap();
                        year >= 2020 && year <= 2030
                    }
                    Requirement::Height => {
                        let units = &value[value.len() - 2..];
                        if units == "cm" || units == "in" {
                            match value[0..value.len() - 2].parse::<u16>() {
                                Err(_) => false,
                                Ok(height) => {
                                    if units == "cm" {
                                        height >= 150 && height <= 193
                                    } else {
                                        height >= 59 && height <= 76
                                    }
                                }
                            }
                        } else {
                            false
                        }
                    }
                    Requirement::HairColor => {
                        let mut chars = value.chars();
                        if chars.next().unwrap() == '#' {
                            chars.all(|c| {
                                c.is_digit(10)
                                    || c == 'a'
                                    || c == 'b'
                                    || c == 'c'
                                    || c == 'd'
                                    || c == 'e'
                                    || c == 'f'
                            })
                        } else {
                            false
                        }
                    }
                    Requirement::EyeColor => {
                        value == "amb"
                            || value == "blu"
                            || value == "brn"
                            || value == "gry"
                            || value == "grn"
                            || value == "hzl"
                            || value == "oth"
                    }
                    Requirement::PassportId => {
                        value.len() == 9 && value.chars().all(|c| c.is_digit(10))
                    }
                };
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

enum Requirement {
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
    fn valid() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(solve(input), 4);
    }

    #[test]
    fn invalid() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(solve(input), 0);
    }
}

common::read_main!();
