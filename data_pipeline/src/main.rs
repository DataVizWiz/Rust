use data_pipeline::movies::metadata::Movie;
use data_pipeline::movies::ratings::Rating;

fn main() {
    let mut movies: Vec<Movie> = Movie::as_vec();
    let ratings_map = Rating::as_map();

    //To modify the ratings field you need a mutable reference to each Movie in the vector
    for movie in &mut movies {
        // Use if let for optional handling. Dereference the values using &
        if let Some(&rating) = ratings_map.get(&movie.id) {
            movie.rating = Some(rating);
        } else {
            movie.rating = None;
        }
    }

    let fv_star_rom: Vec<&Movie> = movies
        .iter()
        .filter(|&movie| movie.rating == Some(5.0))
        .filter(|&movie| movie.genres.contains("Romance"))
        .collect();

    Movie::insert_rows(fv_star_rom);
}
