// [TODO] Handle multiple error types using Option or custom enum
// Topics for improvement
// - Deserialization
// - Traits
// - Enums/Option

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::env::{self, VarError};
use std::error::Error;

#[derive(Deserialize)]
struct Cat {
    fact: String,
    length: u32,
}

fn get_api_key() -> Result<String, VarError> {
    let key = env::var("CAT_FACTS_API_KEY")?;
    Ok(key)
}

fn set_header_values() -> HeaderMap {
    // Multimap of header names to values (key-value pair)
    let mut headers = HeaderMap::new();

    // Parse the value to a HeaderValue type
    // Required as .insert takes val: HeaderValue
    let header_val = "application/json".parse::<HeaderValue>();

    // .parse returns Result<T, E>
    match header_val {
        Ok(val) => {
            headers.insert("accept", val);
        }
        Err(e) => println!("Error parsing header value: {}", e),
    }

    // match as method returns Result<T, E>
    match get_api_key() {
        Ok(key) => {
            // Then parse the API key as HeaderValue
            match key.parse::<HeaderValue>() {
                Ok(x) => {
                    headers.insert("X-CSRF-TOKEN", x);
                    ()
                }
                Err(e) => println!("Error parsing API key: {}", e),
            };
        }
        Err(e) => println!("Error getting API key: {}", e),
    };

    headers
}

fn get_https_result() -> Result<Response, reqwest::Error> {
    let url = "https://catfact.ninja/fact";
    let header = set_header_values();

    let https_client = Client::new();

    let https_result = https_client
        // Methods return a RequestBuilder (excl send)
        .get(url)
        .headers(header)
        .send()?;

    Ok(https_result)
}

fn response_body() -> Result<String, reqwest::Error> {
    let response = get_https_result()?;
    let body = response.text()?;
    Ok(body)
}

// Use Box trait to handle multiple errors
// Downsides:
// - Stores errors on the heap as the type is not known
// - Errors are only known at runtime rather than compile time
fn json_response() -> Result<Cat, Box<dyn Error>> {
    let body = response_body()?; // Returns reqwest err

    // Deserialize string to JSON into a Cat struct
    let json = serde_json::from_str::<Cat>(&body)?; // Returns serde err
    Ok(json)
}

fn main() {
    let json = json_response();

    match json {
        Ok(x) => {
            println!("Cat fact: {}", x.fact);
            println!("Fact length: {}", x.length);
        }
        Err(err) => println!("Error parsing JSON response: {}", err),
    }
}
