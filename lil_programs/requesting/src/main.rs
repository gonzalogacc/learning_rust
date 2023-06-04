use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artist: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_url: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}


#[tokio::main]
async fn main() {
    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track,artist", query = "Babasonicos");
    println!("Url is {url}");

    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("AUTHORIZATION", "Bearer BQCjmibrIZbODLAVTGLk7h4S7TMLQV_x6CZDB5CASWKZcNobZx-FAfhyroAwJfG6kyOBYsxYLap4hZgRHZE8pzevT75Q58accOGpGFEFWNI_4DlvrUA")
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();
    
    println!("Respnse is {:?}", response);
    print_type_of(&response);

    //match response {
    //    Ok(r) => {
    //        match r.status() {
    //            reqwest::StatusCode::OK => println!("OK"),
    //            _ => println!("Dentro de la chingada")
    //        }
    //    },
    //    _ => println!("Error"),
    //}

    match response.status() {
        reqwest::StatusCode::OK => {
            //match response.json::<APIResponse>().await {
            match response.json::<serde_json::Value>().await {
                Ok(parsed) => println!("Parsed {:?}", parsed),
                Err(_) => println!("Error en la parseada"),
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        },
        _ => {
            panic!("SOmethign is wrong here");
        },
    };
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
