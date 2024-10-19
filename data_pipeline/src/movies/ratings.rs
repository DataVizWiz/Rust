use crate::utils::io::read_csv;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)] // For development only
#[derive(Deserialize)]
pub struct Rating {
    #[serde(rename(deserialize = "userId"))]
    user_id: i32,
    #[serde(rename(deserialize = "movieId"))]
    movie_id: i32,
    rating: f32,
    timestamp: i32,
}

impl Rating {
    pub fn as_map() -> HashMap<i32, f32> {
        let mut rdr = read_csv("data/ratings_small.csv").expect("Error reading file");

        let mut map: HashMap<i32, f32> = HashMap::new();

        for res in rdr.deserialize() {
            let rating: Rating = res.unwrap();
            map.insert(rating.movie_id, rating.rating);
        }
        map
    }
}
