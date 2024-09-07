use csv::{ReaderBuilder, Trim};
use std::process::exit;

#[derive(serde::Deserialize, Debug)]
struct Car {
    #[serde(rename(deserialize = "Brand"))]
    brand: String,
    #[serde(rename(deserialize = "Model"))]
    model: String,
    #[serde(rename(deserialize = "Year"))]
    year: u32
}

fn main() {
    let file = "data.csv";
    let mut builder = ReaderBuilder::new();
    builder.delimiter(b',')
        .trim(Trim::All);
    
    // Returns Result<Reader<File>, Error>
    let result = builder.from_path(file);

    if result.is_err() {
        println!("{} does not exist.", file);
        exit(9);
    }

    // "Unwrap" to get Reader<File> inside Result<T, E>
    let mut reader = result.unwrap();

    // Each item in the iterator is a Result<D, Error>
    // "Unwrap" the string record inside
    for record in reader.deserialize() {
        let car: Car = record.unwrap();
        println!("Car: {}, {}, {}", car.brand, car.model, car.year);
    }
}