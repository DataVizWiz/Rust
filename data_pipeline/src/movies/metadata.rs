use crate::utils::io::read_csv;
use chrono::NaiveDate;
use serde::Deserialize;

#[allow(dead_code)] //For development only
#[derive(Deserialize, Debug)]
pub struct Movie {
    //Handle empty values with Option enum
    adult: String,
    belongs_to_collection: Option<String>,
    pub budget: u32,
    pub genres: String,
    homepage: Option<String>,
    pub id: u32,
    imdb_id: String,
    original_language: String,
    pub original_title: String,
    pub overview: String,
    popularity: f32,
    poster_path: Option<String>,
    production_companies: String,
    production_countries: String,
    release_date: Option<NaiveDate>,
    revenue: f32,
    runtime: Option<f32>,
    spoken_languages: String,
    status: String,
    tagline: String,
    title: String,
    video: String,
    vote_average: f32,
    vote_count: u32,
    pub rating: Option<f32>,
}

impl Movie {
    pub fn as_vec() -> Vec<Movie> {
        let mut rdr = read_csv("data/movies_metadata.csv").expect("Error reading file");
        let mut movies: Vec<Movie> = Vec::new();

        for res in rdr.deserialize() {
            let movie: Movie = res.expect("Invalid movie record");

            if movie.budget > 0 {
                movies.push(movie);
            }
        }
        movies
    }
}
