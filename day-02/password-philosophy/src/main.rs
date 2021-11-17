use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Example input data: "1-3 a: abcde"
// `1-3 a` means that the password must contain `a` at least `1` time and at
// most `3` times.
struct PasswordPolicy {
    pub min_repeat: u8,
    pub max_repeat: u8,
    pub letter: char,
    pub password: String,
}

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
    let lines = match parse_file_to_vector(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("unable to parse data from file, {}", e);
            return;
        }
    };
    let correct_passwords =
        lines
            .into_iter()
            .flat_map(PasswordPolicy::try_from)
            .fold(
                0,
                |acc, password| if password.is_valid() { acc + 1 } else { acc },
            );
    println!("Correct passwords: {}", correct_passwords);
}

impl TryFrom<String> for PasswordPolicy {
    type Error = String;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let split_line = line.split(' ').collect::<Vec<&str>>();
        // Line with password policy must be split into vector with three elements.
        // Example: "1-3 a: abcde" -> ["1-3", "a:", "abcde"]
        if split_line.len() != 3 {
            let error_message = format!("problem with split raw line: {}", line);
            return Err(error_message);
        }

        // This `unwrap()` is safe because we've checked the size of the
        // vector before.
        let min_max = split_line.get(0).unwrap();
        let first_letter = split_line
            .get(1)
            .unwrap()
            .chars()
            .into_iter()
            .next()
            .ok_or_else(|| String::from("problem with take letter"))?;
        let password = split_line.get(2).unwrap().to_string();

        let split_min_max = min_max.split('-').collect::<Vec<&str>>();
        if split_min_max.len() != 2 {
            let error_message = format!("problem with split min/max: {}", min_max);
            return Err(error_message);
        }
        let min_repeat = split_min_max
            .get(0)
            .unwrap()
            .parse::<u8>()
            .map_err(|e| format!("unable to parse min repeat, {}", e))?;
        let max_repeat = split_min_max
            .get(1)
            .unwrap()
            .parse::<u8>()
            .map_err(|e| format!("unable to parse max repeat, {}", e))?;
        Ok(Self {
            min_repeat,
            max_repeat,
            letter: first_letter,
            password,
        })
    }
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let letter_counter = self.password.chars().into_iter().fold(0, |acc, letter| {
            if letter == self.letter {
                acc + 1
            } else {
                acc
            }
        });
        letter_counter >= self.min_repeat && letter_counter <= self.max_repeat

        // Second version
        // let letter_counter = self
        //     .password
        //     .chars()
        //     .into_iter()
        //     .filter(|letter| *letter == self.letter)
        //     .count();

        // letter_counter >= usize::from(self.min_repeat)
        //     && letter_counter <= usize::from(self.max_repeat)
    }
}

fn parse_file_to_vector(file_name: &Path) -> std::io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.unwrap_or_else(|_| "".to_owned()))
        .collect::<Vec<String>>())
}

#[cfg(test)]
mod example_data {
    #[test]
    fn example_data() {
        let input = vec![
            String::from("invalid-data 1"),   // invalid
            String::from("invalid-data 1 2"), // invalid
            String::from("1-3 a: abcde"),     // valid
            String::from("1-3 b: cdefg"),     // invalid
            String::from("2-9 c: ccccccccc"), // valid
        ];

        let valid_passwords = input
            .into_iter()
            .flat_map(super::PasswordPolicy::try_from)
            .fold(
                0,
                |acc, password| if password.is_valid() { acc + 1 } else { acc },
            );
        assert_eq!(valid_passwords, 2);
    }
}
