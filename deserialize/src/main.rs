use csv::{Reader, StringRecord};
use std::time::Instant;

#[derive(Debug, serde::Deserialize)]
struct Taxi;

fn main() {
    let start_time = Instant::now();
    
    let file = "taxi_fares.csv";
    let read_file = Reader::from_path(file);
    let mut reader = read_file.unwrap();

    let mut taxis: Vec<Taxi> = Vec::new();
    let mut invalid_rows: Vec<StringRecord> = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => {
                let taxi: Result<Taxi, _> = record.deserialize(None);
                match taxi {
                    Ok(taxi) => taxis.push(taxi),
                    Err(_) => {
                        invalid_rows.push(record);
                    }
                }
            },
            Err(e) => {
                println!("Error deserializing record: {}", e);
                return;
            }
        }
    }

    let elapsed_time = start_time.elapsed();

    println!("Successfully loaded {} records into memory.", taxis.len());
    println!("Number of invalid rows: {}", invalid_rows.len());

    println!("Time taken: {:?}", elapsed_time);
}



