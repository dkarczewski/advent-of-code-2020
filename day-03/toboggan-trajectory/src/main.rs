use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct MovementScheme {
    right: usize,
    down: usize,
}

trait TobogganTrajectory {
    fn calculate_encountered_trees(self, movement_scheme: MovementScheme) -> usize;
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

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("unable to open file, {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let movement_scheme = MovementScheme { right: 3, down: 1 };
    let encountered_trees = reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .calculate_encountered_trees(movement_scheme);

    println!("Encountered trees: {}", encountered_trees);
}

impl<I> TobogganTrajectory for I
where
    I: Iterator<Item = String>,
{
    fn calculate_encountered_trees(self, movement_scheme: MovementScheme) -> usize {
        // Tree is defined as character '#'
        const TREE: char = '#';

        let mut position_to_check = 0;

        // `Skip` because we don't need to check first `n` lines.
        // In these lines there is only a transition to the next line.
        // There is no move to the right.
        // We can check lines that are a combination of defined movement.
        self.skip(movement_scheme.down)
            .step_by(movement_scheme.down)
            .filter(|line| {
                let line_length = line.len();
                position_to_check = (position_to_check + movement_scheme.right) % line_length;
                if let Some(symbol) = line.chars().nth(position_to_check) {
                    symbol == TREE
                } else {
                    false
                }
            })
            .count()
    }
}

#[cfg(test)]
mod example_data {
    use super::MovementScheme;
    use super::TobogganTrajectory;

    #[test]
    fn without_wrapping() {
        let input = vec![
            String::from("............."),
            String::from("............."),
            String::from("......#......"),
            String::from(".........#..."),
            String::from("............#"),
        ];
        let movement_scheme = MovementScheme { right: 3, down: 1 };
        let encountered_trees = input
            .into_iter()
            .calculate_encountered_trees(movement_scheme);

        assert_eq!(encountered_trees, 3);
    }

    #[test]
    fn with_wrapping() {
        let input = vec![
            String::from("......."),
            String::from("...#..."),
            String::from("......#"),
            String::from("..#...."),
            String::from(".....#."),
        ];
        let movement_scheme = MovementScheme { right: 3, down: 1 };
        let encountered_trees = input
            .into_iter()
            .calculate_encountered_trees(movement_scheme);

        assert_eq!(encountered_trees, 4);
    }

    #[test]
    fn example_data() {
        let input = vec![
            String::from("..##.........##.........##.........##.........##.........##......."),
            String::from("#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.."),
            String::from(".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#."),
            String::from("..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#"),
            String::from(".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#."),
            String::from("..#.##.......#.##.......#.##.......#.##.......#.##.......#.##....."),
            String::from(".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#"),
            String::from(".#........#.#........#.#........#.#........#.#........#.#........#"),
            String::from("#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#..."),
            String::from("#...##....##...##....##...##....##...##....##...##....##...##....#"),
            String::from(".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"),
        ];
        let movement_scheme = MovementScheme { right: 3, down: 1 };
        let encountered_trees = input
            .into_iter()
            .calculate_encountered_trees(movement_scheme);

        assert_eq!(encountered_trees, 7);
    }
}
