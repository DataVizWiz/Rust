use std::fs::File;
use std::io::{Error, BufReader, BufRead};


fn read_file() -> Result<File, Error> {
    // Propogate errors higher up the call stack
    let result = File::open("numbers.txt")?;
    Ok(result)
}

fn parse_lines(file: File) -> Result<Vec<i32>, Error> {
    let mut numbers: Vec<i32> = Vec::new();

    for line in BufReader::new(file)
        .lines() {
            // As line returns a Result<T, E> we can use ? to propogate errors
            let content = line?;

            match content.parse::<i32>() {
                Ok(n) => numbers.push(n),
                Err(_) => println!("'{content}' is an invalid record."),
            }
        }

    Ok(numbers)
}

// Pass in a slice reference
fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    match read_file() {
        Ok(x) => {
            // Parse the file content into a vector of integers
            match parse_lines(x) {
                Ok(numbers) => {
                    if !numbers.is_empty() {
                        let sum = sum_numbers(&numbers);
                        println!("Sum: {sum}");
                    } else {
                        println!("No valid numbers to sum.");
                    }
                },
                Err(e) => println!("Error parsing file: {e}.")
            }
        },
        Err(e) => println!("Error opening file: {e}.")
    };
}