use types::Atom;
use types::ParsedPuzzle;
use types::Puzzle;

use std::fs;
use std::io;
use std::process;

//Possible errors in parse_puzzle_size and parse_puzzle
pub static ERR_NUMBER_PER_LINE: &str = "Numbers per lines have to be equal to size";
pub static ERR_NUMBER_INVALID: &str = "Invalid number";
pub static ERR_SIZE_INVALID: &str = "Invalid size";
pub static ERR_SIZE_SYNTAX: &str = "Size declaration syntax invalid";
pub static ERR_IO_STDIN: &str = "IO Error : Failed to read stdin. Exiting";
pub static ERR_OPEN_FILE: &str = "Failed to open file. Exiting";

//Possible errors in is_puzzle_correct
pub static ERR_NUMBER_OF_LINES: &str = "Invalid number of lines in n-puzzle";
pub static ERR_INVALID_ELMT: &str = "One or more elements are superior to (size * size - 1)";
pub static ERR_DUPLICATED_VALUE: &str = "One or more elements are duplicated";
pub static ERR_SIZE: &str = "Size must be superior or equal to 3";

fn is_puzzle_correct(puzzle: &ParsedPuzzle, filename: &String) -> bool {
    let max_number: Atom = puzzle.size * puzzle.size;

    if puzzle.size < 3 {
        println!("File : {} : {}", filename, ERR_SIZE);
        return false;
    }
    if puzzle.container.len() != max_number as usize {
        println!("File : {} : {}", filename, ERR_NUMBER_OF_LINES);
        return false;
    }
    if let Some(_) = puzzle
        .container
        .clone()
        .into_iter()
        .find(|x| *x > max_number - 1)
    {
        println!("File : {} : {}", filename, ERR_INVALID_ELMT);
        return false;
    }
    for i in 0..max_number as usize {
        for j in (i + 1)..max_number as usize {
            if puzzle.container[i] == puzzle.container[j] {
                println!("File : {} : {}", filename, ERR_DUPLICATED_VALUE);
                return false;
            }
        }
    }
    true
}

fn parse_puzzle_size(size: &mut Atom, line: &Vec<String>) -> Option<Vec<&'static str>> {
    let mut vec_err: Vec<&'static str> = Vec::new();
    let mut had_parse_size = false;

    for word in line {
        match (word.parse::<Atom>(), had_parse_size) {
            (Ok(0...2), false) => vec_err.push(&ERR_SIZE_INVALID),
            (Ok(val), false) => {
                *size = val;
                had_parse_size = true;
            }
            (Err(_), false) => vec_err.push(&ERR_NUMBER_INVALID),
            (_, true) => vec_err.push(&ERR_SIZE_SYNTAX),
        }
    }
    if vec_err.len() == 0 {
        return None;
    }
    Some(vec_err)
}

fn parse_puzzle(puzzle: &mut Puzzle, size: &Atom, line: &Vec<String>) -> Option<Vec<&'static str>> {
    let mut vec_err: Vec<&'static str> = Vec::new();
    let mut vec_cur_line: Vec<Atom> = Vec::new();

    for word in line {
        match word.parse::<Atom>() {
            Ok(val) => vec_cur_line.push(val),
            Err(_) => vec_err.push(&ERR_NUMBER_INVALID),
        }
    }
    if vec_cur_line.len() == *size as usize && vec_err.len() == 0 {
        puzzle.append(&mut vec_cur_line);
        return None;
    }
    if line.len() != *size as usize {
        vec_err.push(&ERR_NUMBER_PER_LINE);
    }
    Some(vec_err)
}

fn parse_line(puzzle: &mut ParsedPuzzle, line: &str) -> Option<Vec<&'static str>> {
    //setup line to be parsed
    let v_str: Vec<&str> = line.trim().split("#").collect();
    let mut iter = v_str[0].split_whitespace();
    let mut vec_splited: Vec<String> = Vec::new();
    while let Some(word) = iter.next() {
        vec_splited.push(String::from(word));
    }

    //Parsing input
    match puzzle.size {
        0 => return parse_puzzle_size(&mut puzzle.size, &vec_splited),
        _ => return parse_puzzle(&mut puzzle.container, &puzzle.size, &vec_splited),
    };
}

fn generate_puzzle(content: &String, filename: &String) -> Option<ParsedPuzzle> {
    let mut had_error = false;
    let mut puzzle = ParsedPuzzle {
        container: Vec::new(),
        size: 0,
    };
    let mut splited_file = content.lines();
    let mut line_nb: u64 = 1;

    while let Some(line) = splited_file.next() {
        match parse_line(&mut puzzle, &line) {
            None => {}
            Some(v_err) => {
                had_error = true;
                for e in v_err {
                    println!("File : {} : at line {} : {}", filename, line_nb, e);
                    if puzzle.size == 0 {
                        return None;
                    }
                }
            }
        }
        line_nb = line_nb + 1;
    }
    if had_error {
        return None;
    }
    match is_puzzle_correct(&puzzle, filename) {
        true => Some(puzzle),
        false => None,
    }
}

fn generate_puzzle_from_stdin() -> Option<ParsedPuzzle> {
    let mut had_error = false;
    let mut puzzle = ParsedPuzzle {
        container: Vec::new(),
        size: 0,
    };
    let mut line_nb: u64 = 1;

    loop {
        let mut buff = String::new();
        match io::stdin().read_line(&mut buff) {
            Ok(n) => {
                //check stdin exit pattern
                buff = buff.trim().to_string();
                if n == 0 {
                    break;
                }
                //parsing
                match parse_line(&mut puzzle, &buff) {
                    None => {}
                    Some(v_err) => {
                        had_error = true;
                        for e in v_err {
                            println!(
                                "File : {} : at line {} : {}",
                                "stdin".to_string(),
                                line_nb,
                                e
                            );
                            if puzzle.size == 0 {
                                return None;
                            }
                        }
                    }
                }
                line_nb = line_nb + 1;
            }
            Err(_) => {
                println!("{}", ERR_IO_STDIN);
                process::exit(1);
            }
        }
    }
    if had_error {
        return None;
    }
    match is_puzzle_correct(&puzzle, &"stdin".to_string()) {
        true => Some(puzzle),
        false => None,
    }
}

pub fn parse(file: Option<&String>) -> Option<ParsedPuzzle> {
    match file {
        None => generate_puzzle_from_stdin(),
        Some(filename) => match fs::read_to_string(filename) {
            Ok(s) => generate_puzzle(&s, &filename),
            Err(_) => {
                println!("File : {} : {}", filename, ERR_OPEN_FILE);
                process::exit(1);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_puzzle_size_ok() {
        let mut size: Atom = 0;
        let vec_test: Vec<String> = vec!["3".to_string()];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle_size(&mut size, &vec_test);

        assert_eq!(vec_err, None);
        assert_eq!(size, 3);
    }

    #[test]
    fn parse_puzzle_size_too_small() {
        let mut size: Atom = 0;
        let vec_test: Vec<String> = vec!["2".to_string()];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle_size(&mut size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_SIZE_INVALID]);
        assert_eq!(size, 0);
    }

    #[test]
    fn parse_puzzle_size_negative() {
        let mut size: Atom = 0;
        let vec_test: Vec<String> = vec!["-2".to_string()];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle_size(&mut size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_NUMBER_INVALID]);
        assert_eq!(size, 0);
    }

    #[test]
    fn parse_puzzle_size_invalid_word() {
        let mut size: Atom = 0;
        let vec_test: Vec<String> = vec!["toto".to_string()];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle_size(&mut size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_NUMBER_INVALID]);
        assert_eq!(size, 0);
    }

    #[test]
    fn parse_puzzle_size_too_much_args() {
        let mut size: Atom = 0;
        let vec_test: Vec<String> = vec!["3".to_string(), "4".to_string()];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle_size(&mut size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_SIZE_SYNTAX]);
        assert_eq!(size, 3);
    }

    #[test]
    fn parse_puzzle_ok() {
        let mut puzzle: Puzzle = Vec::new();
        let size: Atom = 3;

        let vec_test: Vec<String> = vec!["3".to_string(), "4".to_string(), "5".to_string()];
        let vec_expected: Puzzle = vec![3, 4, 5];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle(&mut puzzle, &size, &vec_test);

        assert_eq!(vec_err, None);
        assert_eq!(puzzle, vec_expected);
    }

    #[test]
    fn parse_puzzle_invalid_size() {
        let mut puzzle: Puzzle = Vec::new();
        let size: Atom = 5;

        let vec_test: Vec<String> = vec!["3".to_string(), "4".to_string(), "5".to_string()];
        let vec_expected: Puzzle = vec![];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle(&mut puzzle, &size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_NUMBER_PER_LINE]);
        assert_eq!(puzzle, vec_expected);
    }

    #[test]
    fn parse_puzzle_invalid_char() {
        let mut puzzle: Puzzle = Vec::new();
        let size: Atom = 3;

        let vec_test: Vec<String> = vec!["titi".to_string(), "4".to_string(), "toto".to_string()];
        let vec_expected: Puzzle = vec![];
        let vec_err: Option<Vec<&'static str>> = parse_puzzle(&mut puzzle, &size, &vec_test);

        assert_ne!(vec_err, None);
        assert_eq!(
            vec_err.unwrap(),
            vec![ERR_NUMBER_INVALID, ERR_NUMBER_INVALID]
        );
        assert_eq!(puzzle, vec_expected);
    }

    #[test]
    fn is_puzzle_correct_ok() {
        let puzzle = ParsedPuzzle {
            container: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            size: 3,
        };
        let filename: String = String::from("testfile");

        assert_eq!(true, is_puzzle_correct(&puzzle, &filename));
    }

    #[test]
    fn is_puzzle_correct_line_failed_number_of_line() {
        let puzzle = ParsedPuzzle {
            container: vec![0, 1, 2, 3, 4, 5],
            size: 3,
        };
        let filename: String = String::from("testfile");

        assert_eq!(false, is_puzzle_correct(&puzzle, &filename));
    }

    #[test]
    fn is_puzzle_correct_failed_superior_to_max_number() {
        let puzzle = ParsedPuzzle {
            container: vec![0, 1, 2, 3, 4, 5, 6, 11, 10],
            size: 3,
        };
        let filename: String = String::from("testfile");

        assert_eq!(false, is_puzzle_correct(&puzzle, &filename));
    }

    #[test]
    fn is_puzzle_correct_failed_duplicated_value() {
        let puzzle = ParsedPuzzle {
            container: vec![0, 1, 2, 3, 4, 3, 6, 7, 8],
            size: 3,
        };
        let filename: String = String::from("testfile");

        assert_eq!(false, is_puzzle_correct(&puzzle, &filename));
    }

    #[test]
    fn parse_line_size_ok() {
        let mut puzzle = ParsedPuzzle {
            container: vec![],
            size: 0,
        };
        let test_line = String::from("3              #toto");
        let vec_err: Option<Vec<&'static str>> = parse_line(&mut puzzle, &test_line);

        assert_eq!(vec_err, None);
    }

    #[test]
    fn parse_line_size_fail() {
        let mut puzzle = ParsedPuzzle {
            container: vec![],
            size: 0,
        };
        let test_line = String::from("-3              #toto");
        let vec_err: Option<Vec<&'static str>> = parse_line(&mut puzzle, &test_line);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_NUMBER_INVALID]);
    }

    #[test]
    fn parse_line_line_ok() {
        let mut puzzle = ParsedPuzzle {
            container: vec![],
            size: 3,
        };
        let test_line = String::from("1     0               4            #toto");
        let vec_err: Option<Vec<&'static str>> = parse_line(&mut puzzle, &test_line);

        assert_eq!(vec_err, None);
    }

    #[test]
    fn parse_line_line_fail() {
        let mut puzzle = ParsedPuzzle {
            container: vec![],
            size: 3,
        };
        let test_line = String::from("1     0               caca            #toto");
        let vec_err: Option<Vec<&'static str>> = parse_line(&mut puzzle, &test_line);

        assert_ne!(vec_err, None);
        assert_eq!(vec_err.unwrap(), vec![ERR_NUMBER_INVALID]);
    }
}
