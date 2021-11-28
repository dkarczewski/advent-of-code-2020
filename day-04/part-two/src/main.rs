use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::Path;

struct Passport(HashMap<String, String>);

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Program must be executed with one argument: [file_name]");
        return;
    }

    // First argument is name of binary file.
    // Usefully is second argument which is `file name` with data.
    let file_name = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("unable to get file name");
            return;
        }
    };
    let path = Path::new(file_name);
    let raw_data_file = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to read data from file, {}", e);
            return;
        }
    };

    let valid_passports = count_valid_passports(raw_data_file);
    println!("Valid passports: {}", valid_passports);
}

fn count_valid_passports(raw_string: String) -> usize {
    raw_string
        .split("\n\n")
        .map(|raw_data| {
            raw_data
                .split_ascii_whitespace()
                .map(|key_value| {
                    let mut pair_data = key_value.split(':');
                    let key = pair_data.next().unwrap().to_string();
                    let value = pair_data.next().unwrap().to_string();
                    (key, value)
                })
                .collect::<Passport>()
        })
        .filter(Passport::is_valid)
        .count()
}

impl Passport {
    /// Key for `birth year`.
    const BYR: &'static str = "byr";
    /// Key for `issue year`.
    const IYR: &'static str = "iyr";
    /// Key for `expiration year`.
    const EYR: &'static str = "eyr";
    /// Key for `height`.
    const HGT: &'static str = "hgt";
    /// Key for `hair color`.
    const HCL: &'static str = "hcl";
    /// Key for `eye color`.
    const ECL: &'static str = "ecl";
    /// Key for `passport ID`.
    const PID: &'static str = "pid";
    /// Key for `country ID`.
    #[allow(dead_code)]
    const CID: &'static str = "cid";

    const REQUIRED_KEYS: [&'static str; 7] = [
        Self::BYR,
        Self::IYR,
        Self::EYR,
        Self::HGT,
        Self::HCL,
        Self::ECL,
        Self::PID,
        // Field `cid` is not necessary - it's optional.
    ];

    pub fn is_valid(&self) -> bool {
        self.contains_required_keys()
            && self.is_valid_birth_year()
            && self.is_valid_issue_year()
            && self.is_valid_expiration_year()
            && self.is_valid_height()
            && self.is_valid_hair_color()
            && self.is_valid_eye_color()
            && self.is_valid_passport_id()
    }

    fn contains_required_keys(&self) -> bool {
        Self::REQUIRED_KEYS
            .into_iter()
            .all(|key| self.0.contains_key(key))
    }

    fn is_valid_birth_year(&self) -> bool {
        const MIN_VALID_YEAR: u16 = 1920;
        const MAX_VALID_YEAR: u16 = 2002;

        let birth_year = match self.0.get(Self::BYR) {
            Some(val) => val.parse::<u16>(),
            None => return false,
        };

        // We can use:
        // `matches!(birth_year, Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year))`
        // but `cargo fmt` does not format correctly long macro.
        #[allow(clippy::match_like_matches_macro)]
        match birth_year {
            Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year) => true,
            _ => false,
        }
    }

    fn is_valid_issue_year(&self) -> bool {
        const MIN_VALID_YEAR: u16 = 2010;
        const MAX_VALID_YEAR: u16 = 2020;

        let issue_year = match self.0.get(Self::IYR) {
            Some(val) => val.parse::<u16>(),
            None => return false,
        };

        // We can use:
        // `matches!(issue_year, Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year))`
        // but `cargo fmt` does not format correctly long macro.
        #[allow(clippy::match_like_matches_macro)]
        match issue_year {
            Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year) => true,
            _ => false,
        }
    }

    fn is_valid_expiration_year(&self) -> bool {
        const MIN_VALID_YEAR: u16 = 2020;
        const MAX_VALID_YEAR: u16 = 2030;

        let expiration_year = match self.0.get(Self::EYR) {
            Some(val) => val.parse::<u16>(),
            None => return false,
        };

        // We can use:
        // `matches!(expiration_year, Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year))`
        // but `cargo fmt` does not format correctly long macro.
        #[allow(clippy::match_like_matches_macro)]
        match expiration_year {
            Ok(year) if (MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year) => true,
            _ => false,
        }
    }

    fn is_valid_height(&self) -> bool {
        const MIN_CM_HEIGHT: u8 = 150;
        const MAX_CM_HEIGHT: u8 = 193;
        const MIN_IN_HEIGHT: u8 = 59;
        const MAX_IN_HEIGHT: u8 = 76;

        let height = match self.0.get(Self::HGT) {
            Some(val) => val,
            None => return false,
        };
        if let Some(char_position) = height.find("cm") {
            match height[0..char_position].parse::<u8>() {
                Ok(value) if (MIN_CM_HEIGHT..=MAX_CM_HEIGHT).contains(&value) => return true,
                _ => return false,
            }
        }
        if let Some(char_position) = height.find("in") {
            match height[0..char_position].parse::<u8>() {
                Ok(value) if (MIN_IN_HEIGHT..=MAX_IN_HEIGHT).contains(&value) => return true,
                _ => return false,
            }
        }
        false
    }

    fn is_valid_hair_color(&self) -> bool {
        // Regex: a `#` followed by exactly six characters `0-9` or `a-f`.
        let re = Regex::new("#([0-9a-f]){6}$").unwrap();
        let hair_color = match self.0.get(Self::HCL) {
            Some(val) => val,
            None => return false,
        };
        re.is_match(hair_color)
    }

    fn is_valid_eye_color(&self) -> bool {
        const AVAILABLE_EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let eye_color = match self.0.get(Self::ECL) {
            Some(val) => val,
            None => return false,
        };
        AVAILABLE_EYE_COLOR
            .into_iter()
            .any(|color| eye_color.contains(color))
    }

    fn is_valid_passport_id(&self) -> bool {
        // Regex: a nine-digit number, including leading zeroes.
        let re = Regex::new("^[0-9]{9}$").unwrap();
        let passport_id = match self.0.get(Self::PID) {
            Some(val) => val,
            None => return false,
        };
        re.is_match(passport_id)
    }
}

impl FromIterator<(String, String)> for Passport {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        let mut hash_map = HashMap::new();
        for (key, value) in iter {
            hash_map.insert(key, value);
        }
        Passport(hash_map)
    }
}

#[cfg(test)]
mod example_data {
    use super::count_valid_passports;

    #[test]
    fn invalid_data() {
        // Simulates data saved in file.
        // Each line is ended with newline ('\n').
        let lines = vec![
            "eyr:1972 cid:100\n",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n",
            "\n",
            "iyr:2019\n",
            "hcl:#602927 eyr:1967 hgt:170cm\n",
            "ecl:grn pid:012533040 byr:1946\n",
            "\n",
            "hcl:dab227 iyr:2012\n",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n",
            "\n",
            "hgt:59cm ecl:zzz\n",
            "eyr:2038 hcl:74454a iyr:2023\n",
            "pid:3556412378 byr:2007",
        ];
        let mut one_string = String::new();
        for line in lines {
            one_string.push_str(line);
        }
        let valid_passports = count_valid_passports(one_string);
        assert_eq!(valid_passports, 0);
    }

    #[test]
    fn valid_data() {
        // Simulates data saved in file.
        // Each line is ended with newline ('\n').
        let lines = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n",
            "hcl:#623a2f\n",
            "\n",
            "eyr:2029 ecl:blu cid:129 byr:1989\n",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n",
            "\n",
            "hcl:#888785\n",
            "hgt:164cm byr:2001 iyr:2015 cid:88\n",
            "pid:545766238 ecl:hzl\n",
            "eyr:2022\n",
            "\n",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        let mut one_string = String::new();
        for line in lines {
            one_string.push_str(line);
        }
        let valid_passports = count_valid_passports(one_string);
        assert_eq!(valid_passports, 4);
    }
}
