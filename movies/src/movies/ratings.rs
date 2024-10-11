use crate::utils::io::read_csv;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)] // For development only
#[derive(Deserialize)]
pub struct Rating {
    #[serde(rename(deserialize = "userId"))]
    user_id: u32,
    #[serde(rename(deserialize = "movieId"))]
    movie_id: u32,
    rating: f32,
    timestamp: u32,
}

impl Rating {
    pub fn as_map() -> HashMap<u32, f32> {
        let mut rdr = read_csv("data/ratings_small.csv").expect("Error reading file");

        let mut map: HashMap<u32, f32> = HashMap::new();

        for res in rdr.deserialize() {
            let rating: Rating = res.unwrap();
            map.insert(rating.movie_id, rating.rating);
        }
        map
    }
}
