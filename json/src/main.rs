use csv;
use csv::Writer;
use csv::{Reader, ReaderBuilder, Trim};
use flatten_json_object::ArrayFormatting;
use flatten_json_object::Flattener;
use json_objects_to_csv::Json2Csv;
use polars::prelude::*;
use serde_json::{Map, Value};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn read_csv(file: &str) -> Result<Reader<File>, csv::Error> {
    let mut builder = ReaderBuilder::new();
    builder.delimiter(b',').has_headers(true).trim(Trim::All);

    let result = builder.from_path(file)?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let src_df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("movies_metadata.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let pattern = r"'id': (\d+)";

    let df = src_df
        .lazy()
        .with_column(
            when(
                col("belongs_to_collection")
                    .str()
                    .contains_literal(lit("'id'")),
            )
            .then(
                col("belongs_to_collection")
                    .str()
                    .extract(lit(pattern), 1)
                    .str()
                    .strip_chars(lit(" ")),
            )
            .otherwise(lit(""))
            .alias("collection_id"),
        )
        .collect()?;

    println!("{}", df);

    Ok(())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut reader = read_csv("movies_metadata.csv")?;
//     let mut json_strs: Vec<String> = Vec::new();
//     let mut coll_ids: Vec<Option<&String>> = Vec::new();

//     let double: char = '"';

//     for result in reader.records() {
//         let record = result?;
//         let json_rec = record.get(1).unwrap().to_string();

//         // let temp = json_rec.split("'id': ").nth(1).unwrap().split(',').nth(0).map(String::from);
//         // println!("\nItem: {:?}", temp);

//         let extract_id: Vec<String> = json_rec
//             .lines()
//             .filter_map(|line| {
//                 if !line.contains("id") {
//                     return None;
//                 }
//                 json_rec
//                     .split("'id': ")
//                     .nth(1)
//                     .unwrap()
//                     .split(',')
//                     .nth(0)
//                     .map(String::from)
//             })
//             .collect();

//         let coll_id = extract_id.get(0);

//         println!("Item: {:?}", coll_id);
//         coll_ids.push(coll_id);

//         // json_strs.push(json_rec.replace("'", &double.to_string().as_str()));
//     }

//     // for (idx, ele) in json_strs.iter().enumerate() {

//     //    println!("Index: {}", idx);

//     //    if ele.len() == 0 {
//     //       println!("Empty!: {}", ele);
//     //    } else {
//     //       println!("JSON String: {}", ele);
//     //    }
//     //    // let json_obj: Value = serde_json::from_str(&ele)?;
//     //    // println!("{:?}", json_obj);
//     // }

//     Ok(())
// }

// let map_obj: Map<String, Value> = json_obj.as_object().unwrap().clone();
// let map_len = map_obj.len();

// let mut headers: Vec<String> = Vec::with_capacity(map_len);
// let mut row: Vec<String> = Vec::with_capacity(map_len);
// println!("{}", idx);
// if idx == 0 {
//    println!("APpend headers");
//    println!("APpend row");
// } else {
//    println!("APpend just row");
// }

// for key in map_obj {
//    headers.push(key.0);
//    row.push(key.1.to_string().replace('"', ""));
// }
// println!("Headers: {:?}", headers);
// println!("Row: {:?}", row);

// let mut wtr = Writer::from_path("foo.csv")?;
// wtr.write_record(&headers)?;
// wtr.write_record(&row)?;
// wtr.flush()?;
