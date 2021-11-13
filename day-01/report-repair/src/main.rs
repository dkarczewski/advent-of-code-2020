use combinations::Combinations;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    let data = match parse_file_to_vector(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("unable to parse data from file, {}", e);
            return;
        }
    };

    let output = product_data_that_sum_to_2020(data);
    println!("{:?}", output);
}

fn product_data_that_sum_to_2020(data: Vec<usize>) -> Vec<usize> {
    // Make all combinations which have two elements.
    let data_combination = Combinations::new(data, 2);
    data_combination
        .filter(|elements| elements.iter().sum::<usize>() == 2020)
        .map(|elements| elements.into_iter().product())
        .collect::<Vec<usize>>()
}

fn parse_file_to_vector(file_name: &Path) -> std::io::Result<Vec<usize>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.ok().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0))
        .collect::<Vec<usize>>())
}

#[cfg(test)]
mod example_data {
    #[test]
    fn example_data() {
        // 1721 + 299 = 2020
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let output = crate::product_data_that_sum_to_2020(input);
        assert_eq!(output[0], 514579);
    }
}
