use serde_json::{Value, Map};
use csv::Writer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
   let double: char = '"';

   let json_str = r#"{
                        'id': 10194, 
                        'name': 'Toy Story Collection', 
                        'poster_path': '/7G9915LfUQ2lVfwMEEhDsn3kT4B.jpg', 
                        'backdrop_path': 
                        '/9FBwqcd9IRruEDUrTdcaafOMKUq.jpg'
                     }"#
   .replace("'", &double.to_string().as_str());

   let json_obj: Value = serde_json::from_str(&json_str)?;
   let map_obj: Map<String, Value> = json_obj.as_object().unwrap().clone();
   
   let map_len = map_obj.len();
   let mut headers: Vec<String> = Vec::with_capacity(map_len);
   let mut row: Vec<String> = Vec::with_capacity(map_len);
   
   for key in map_obj {
      headers.push(key.0);
      row.push(key.1.to_string().replace('"', ""));
   }
   println!("Headers: {:?}", headers);
   println!("Row: {:?}", row);

   let mut wtr = Writer::from_path("foo.csv")?;
   wtr.write_record(&headers)?;
   wtr.write_record(&row)?;
   wtr.flush()?;

   Ok(())
}
