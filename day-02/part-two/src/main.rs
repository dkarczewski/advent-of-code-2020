use std::convert::TryFrom;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Example input data: "1-3 a: abcde"
// `1-3 a` means that the password must contain `a` ONLY at first or third
// position.
struct PasswordPolicy {
    pub first_letter_position: u8,
    pub second_letter_position: u8,
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
        let positions = split_line.get(0).unwrap();
        let first_letter = split_line
            .get(1)
            .unwrap()
            .chars()
            .into_iter()
            .next()
            .ok_or_else(|| String::from("problem with take letter"))?;
        let password = split_line.get(2).unwrap().to_string();

        let split_positions = positions.split('-').collect::<Vec<&str>>();
        if split_positions.len() != 2 {
            let error_message = format!("problem with split positions: {}", positions);
            return Err(error_message);
        }
        let first_letter_position = split_positions
            .get(0)
            .unwrap()
            .parse::<u8>()
            .map_err(|e| format!("unable to parse first position, {}", e))?;
        let second_letter_position = split_positions
            .get(1)
            .unwrap()
            .parse::<u8>()
            .map_err(|e| format!("unable to parse second position, {}", e))?;
        Ok(Self {
            first_letter_position,
            second_letter_position,
            letter: first_letter,
            password,
        })
    }
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let password_length = self.password.len();
        let first_letter_position = usize::from(self.first_letter_position);
        let second_letter_position = usize::from(self.second_letter_position);

        if password_length < first_letter_position || password_length < second_letter_position {
            // Password is to short or invalid letter position.
            return false;
        }

        // The position of the letter stored in file is numbered from 1.
        // The position of the letter in `chars` is numbered from 0.
        let first_letter_position = first_letter_position - 1;
        let second_letter_position = second_letter_position - 1;

        // This `unwrap()` is safe because we've checked length of the
        // password before.
        let first_letter = self.password.chars().nth(first_letter_position).unwrap();
        let second_letter = self.password.chars().nth(second_letter_position).unwrap();

        // XOR. A valid character ONLY needs to be in the first or second position.
        (first_letter == self.letter) ^ (second_letter == self.letter)
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
            String::from("2-9 c: ccccccccc"), // invalid
            String::from("1-3 a: a"),         // invalid
            String::from("3-1 a: a"),         // invalid
        ];

        let valid_passwords = input
            .into_iter()
            .flat_map(super::PasswordPolicy::try_from)
            .fold(
                0,
                |acc, password| if password.is_valid() { acc + 1 } else { acc },
            );
        assert_eq!(valid_passwords, 1);
    }
}
