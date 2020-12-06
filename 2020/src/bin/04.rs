//! Day 4: Passport processing.

use aoc2020::Input;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let infile: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(infile)?;
    let lines = split_passports(&input);

    let mut passports: Vec<Vec<(String, String)>> = vec![];
    for line in lines {
        let passport = parse_fields(&line);
        passports.push(passport);
    }

    // Part 1: Figure out how many passports are valid.
    let valid: Vec<Vec<(String, String)>> = passports
        .iter()
        .filter(|p| is_passport_valid(p))
        .map(|v| v.clone())
        .collect();
    println!("Valid passports: {}", valid.len());

    // Part 2: Figure out how many passports are valid, based on the
    // extended validation rules.
    let valid: Vec<Vec<(String, String)>> = passports
        .iter()
        .filter(|p| is_passport_valid2(p))
        .map(|v| v.clone())
        .collect();
    println!("Valid passports: {}", valid.len());

    Ok(())
}

// Coerce the input to be a vector of strings, where each string is guaranteed
// to be a full passport record.
fn split_passports(input: &Input) -> Vec<String> {
    let mut passports = vec![];
    let mut buf = vec![];
    for line in input.raw_lines().unwrap() {
        if line.len() == 0 {
            let s = buf.join(" ").to_owned();
            passports.push(s);
            buf.clear();
        } else {
            buf.push(line);
        }
    }
    let s = buf.join(" ").to_owned();
    passports.push(s);
    passports
}

// Given a single-line string containing all of the fields in a passport,
// returns a vector of `Field`s.
// For `s`, pass in an element from the vector returned by `split_passports`.
fn parse_fields(s: &str) -> Vec<(String, String)> {
    let mut fields = vec![];
    let fv: Vec<&str> = s.split_ascii_whitespace().collect();
    for field in fv {
        let ff: Vec<&str> = field.split(':').collect();
        fields.push((ff[0].to_string(), ff[1].to_string()));
    }
    fields
}

fn is_passport_valid(passport: &Vec<(String, String)>) -> bool {
    let mut fields: u8 = 0;
    for field in passport {
        fields |= match field.0.as_str() {
            "byr" => 1,
            "iyr" => 1 << 1,
            "eyr" => 1 << 2,
            "hgt" => 1 << 3,
            "hcl" => 1 << 4,
            "ecl" => 1 << 5,
            "pid" => 1 << 6,
            "cid" => 1 << 7,
            _ => 0,
        };
    }

    fields == 0b0111_1111 || fields == 0b1111_1111
}

fn is_passport_valid2(passport: &Vec<(String, String)>) -> bool {
    for field in passport {
        if !is_field_valid(field) {
            return false;
        }
    }
    is_passport_valid(passport)
}

fn is_field_valid(field: &(String, String)) -> bool {
    match field.0.as_str() {
        "byr" => match field.1.parse::<usize>() {
            Ok(n) => {
                if n < 1920 || n > 2002 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        },
        "iyr" => match field.1.parse::<usize>() {
            Ok(n) => {
                if n < 2010 || n > 2020 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        },
        "eyr" => match field.1.parse::<usize>() {
            Ok(n) => {
                if n < 2020 || n > 2030 {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        },
        "hgt" => match &field.1[field.1.len() - 2..] {
            "cm" => match field.1[..field.1.len() - 2].parse::<usize>() {
                Ok(n) => {
                    if n < 150 || n > 193 {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            },
            "in" => match field.1[..field.1.len() - 2].parse::<usize>() {
                Ok(n) => {
                    if n < 59 || n > 76 {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        },
        "hcl" => {
            if field.1.len() != 7 {
                return false;
            }

            // Make sure the first character is an octothorpe.
            if let Some(c) = field.1.chars().nth(0) {
                if c != '#' {
                    return false;
                }
            }

            // Make sure the next 6 characters are one of 0-9 or a-f.
            for c in field.1[1..].trim().chars() {
                match c {
                    'a' | 'b' | 'c' | 'd' | 'e' | 'f' | '0' | '1' | '2' | '3' | '4' | '5' | '6'
                    | '7' | '8' | '9' => {}
                    _ => {
                        return false;
                    }
                }
            }
        }
        "ecl" => match field.1.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => {
                return false;
            }
        },
        "pid" => {
            if field.1.len() != 9 {
                return false;
            }

            for c in field.1.chars() {
                if !c.is_ascii_digit() {
                    return false;
                }
            }
        }
        "cid" => {
            // Not checked.
        }
        _ => {
            return false;
        }
    }
    true
}

#[test]
fn test_split_passports() {
    let raw_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    let input = Input::from(raw_input);
    let raw_passports = split_passports(&input);

    assert_eq!(raw_passports.len(), 4);
    assert_eq!(
        raw_passports[0],
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
    );
}

#[test]
fn test_parse_fields() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
    let fields = parse_fields(input);
    let fields: Vec<(&str, &str)> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    assert_eq!(fields.len(), 8);
    assert_eq!(
        fields,
        [
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("cid", "147"),
            ("hgt", "183cm"),
        ]
    );
}

#[test]
fn test_is_passport_valid2_invalid() {
    let raw_invalid = vec![
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
        "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
    ];
    for input in raw_invalid {
        let fields = parse_fields(input);
        assert!(!is_passport_valid2(&fields));
    }
}

#[test]
fn test_is_passport_valid2_valid() {
    let raw_valid = vec![
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
        "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
    ];
    for input in raw_valid {
        let fields = parse_fields(input);
        assert!(is_passport_valid2(&fields));
    }
}
