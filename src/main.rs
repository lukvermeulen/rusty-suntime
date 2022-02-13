use reqwest;
use serde::Deserialize;
use std::{io, panic};

#[derive(Deserialize, Debug)]
struct Suntime {
    sunrise: String,
    sunset: String,
    solar_noon: String,
    day_length: String,
    civil_twilight_begin: String,
    civil_twilight_end: String,
    nautical_twilight_begin: String,
    nautical_twilight_end: String,
    astronomical_twilight_begin: String,
    astronomical_twilight_end: String,
}

#[derive(Deserialize, Debug)]
struct SuntimeResponse {
    results: Suntime,
    status: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    lat: String,
    lon: String,
    display_name: String,
}

type LocationResponse = Vec<Location>;

#[tokio::main]
async fn main() {
    println!("Hello to rusty-suntime!\n");

    // Ask user for location
    println!("Enter the location you want to get sunrise / sunset for: ");
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("You did not enter a correct string.");

    println!("\nLets get the sunrise / sunset for {}.", location);

    // get lat and long for location
    let request_url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1&email=youremail@gmail.com",
        location
    );
    let derived_location: Location;
    let response = reqwest::get(request_url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // println!("asdf: {:?}", response.text().await);
            match response.json::<LocationResponse>().await {
                Ok(parsed) => {
                    derived_location = Location {
                        lat: parsed[0].lat.clone(),
                        lon: parsed[0].lon.clone(),
                        display_name: parsed[0].display_name.clone(),
                    }
                }
                Err(error) => {
                    panic!("Couldn't parse the API response. {:?}", error)
                }
            }
        }
        default => {
            panic!(
                "Couldn't talk to nominatim openstreetmap api. \n {}",
                default
            )
        }
    }

    // get suntime for lat and long
    let suntime: Suntime;

    let request_url = format!(
        "https://api.sunrise-sunset.org/json?lat={}&lng={}",
        derived_location.lat, derived_location.lon
    );
    let response = reqwest::get(request_url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // println!("asdf: {:?}", response.text().await);
            match response.json::<SuntimeResponse>().await {
                Ok(parsed) => {
                    suntime = parsed.results;
                    println!(
                        "Suntimes for {} in UTC:\n
                                Sunrise: {}\n
                                Sunset: {}",
                        derived_location.display_name, suntime.sunrise, suntime.sunset
                    );
                }
                Err(error) => println!("Couldn't parse the API response. {:?}", error),
            }
        }
        _ => {
            panic!("Couldn't talk to sunrise / sunset api.")
        }
    }
}
