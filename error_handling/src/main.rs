use std::fs::File;
use std::io::{Error, BufReader, BufRead};


fn read_file() -> Result<File, Error> {
    // Propogate errors higher up the call stack
    let result = File::open("numbers.txt")?;
    Ok(result)
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    match read_file() {
        Ok(x) => {
            for line in BufReader::new(x)
                .lines() {
                    let content = line.unwrap();

                    match content.parse::<i32>() {
                        Ok(n) => vec.push(n),
                        Err(e) => println!("Invalid line: {e}"),
                     }
                }
        },
        Err(e) => println!("Error opening file: {e:?}.")
    };

    println!("{:?}", vec);
}

// fn main() {


//     for i in file.it {
//         println!(i);
//     } 
// }