use csv::{ReaderBuilder, Trim};
use std::process::exit;

fn main() {
    let file = "data.csv";
    let mut builder = ReaderBuilder::new();
    builder.delimiter(b',')
        .has_headers(false)
        .trim(Trim::All);
    
    // Returns Result<Reader<File>, Error>
    let result = builder.from_path(file);

    if result.is_err() {
        println!("{} does not exist.", file);
        exit(9);
    }

    // "Unwrap" to get Reader<File> inside Result<T, E>
    let mut reader = result.unwrap();

    // Each item in the iterator is a Result<StringRecord, Error>
    // "Unwrap" the string record inside
    for record in reader.records() {
        let car = record.unwrap();

        // .get returns Option<&str>
        // "Unwrap" the string inside
        let brand = car.get(0).unwrap();
        let model = car.get(1).unwrap();
        let year = car.get(2).unwrap();

        println!("Car: {}, {}, {}", brand, model, year);
    }
}