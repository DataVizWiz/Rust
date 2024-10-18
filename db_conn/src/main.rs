use postgres::{Client, NoTls};
use std::env;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() {
    let conn_string = env::var("DB_CONN_STRING").expect("Key not found");
    let mut client = Client::connect(&conn_string, NoTls).unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
        )
        .unwrap();

    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
        )
        .unwrap();

    let author = Author {
        _id: 0,
        name: "Metin".to_string(),
        country: "UK".to_string(),
    };

    client
        .execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )
        .unwrap();
}
