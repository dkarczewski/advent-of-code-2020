use std::collections::HashSet;
use std::env;
use std::path::Path;

struct Passport(HashSet<String>);

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
                .map(|key_value| key_value.split(':').next().unwrap().to_string())
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
        Self::REQUIRED_KEYS
            .into_iter()
            .all(|key| self.0.contains(key))
    }
}

impl FromIterator<String> for Passport {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let mut keys = HashSet::new();
        for key in iter {
            keys.insert(key);
        }
        Passport(keys)
    }
}

#[cfg(test)]
mod example_data {
    use super::count_valid_passports;

    #[test]
    fn example_data() {
        // Simulates data saved in file.
        // Each line is ended with newline ('\n').
        let lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
            "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
            "\n",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
            "hcl:#cfa07d byr:1929\n",
            "\n",
            "hcl:#ae17e1 iyr:2013\n",
            "eyr:2024\n",
            "ecl:brn pid:760753108 byr:1931\n",
            "hgt:179cm\n",
            "\n",
            "hcl:#cfa07d eyr:2025 pid:166559648\n",
            "iyr:2011 ecl:brn hgt:59in\n",
        ];
        let mut one_string = String::new();
        for line in lines {
            one_string.push_str(line);
        }
        let valid_passports = count_valid_passports(one_string);
        assert_eq!(valid_passports, 2);
    }
}
