use reqwest;
use serde_json::json;


#[tokio::main]
async fn main() {
    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track,artist", query = "Babasonicos");
    println!("Url is {url}");

    let client = reqwest::Client::new();
    let response = client.get(url)
        .header("AUTHORIZATION", "Bearer BQDIS0A8vLP3RBoGUhW-rPF7QQwcQUoCWugVOjbIgDKoIne8SpcvcMYLoJo8vqzB-j_2ao2wzj5VsR-v9CKu4G4EzG0c1XI8Fidam8hImELYSL7yOvg")
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    let parseado = match response.status() {
        reqwest::StatusCode::OK => {
            println!("This is the OK response");
            match response.json().await {
                Ok(parsed) => parsed,
                Err(_) => json!(null),
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauth get another token");
        },
        _ => {
            panic!("SOmethign is wrong here");
        },
    };

    println!("El objeto que se esta imprimiendo es {:?} listo", parseado);
    println!("El objeto que se esta imprimiendo es {:?} listo", parseado["artists"].as_array());
}

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}
