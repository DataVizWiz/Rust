use std::io::Error;
use std::fs::File;
use csv::{Reader, ReaderBuilder, Trim};

fn csv_builder(file: &str) -> Result<Reader<File>, Error>{
    let mut builder = ReaderBuilder::new();
    
    builder
        .delimiter(b',')
        .has_headers(true)
        .trim(Trim::All);

    // Error propogation higher up the call stack
    let result = builder.from_path(file)?;
    Ok(result)
}

fn process_rows(mut result: Reader<File>) -> Result<(), Error> {
    for record in result.records() {
        let string_record = record?;
        println!("{:?}", string_record);
    }
    Ok(())
}

fn main() {
    let file = "Iris.csv";
    
    match csv_builder(file) {
        Ok(x) => {
            if let Err(e) = process_rows(x) {
                println!("Error processing records: {e}");
            }
        },
        Err(e) => println!("Error building file: {e}")
    };
}
