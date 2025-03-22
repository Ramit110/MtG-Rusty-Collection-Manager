use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Serialize, Deserialize};
use std::fs::{File, create_dir};
use std::io::Write;

#[derive(Deserialize, Serialize)]
struct ScryfallBulkData {
    object: String,
    id: String,
    uri: String,
    r#type: String,
    name: String,
    description: String,
    download_uri: String,
    updated_at: String,
    size: u128,
    content_type: String,
    content_encoding: String,
}

#[derive(Deserialize, Serialize)]
struct ScryfallBulkDataResponse {
    object: String,
    has_more: bool,
    data: Vec<ScryfallBulkData>,
}

#[tokio::main]
async fn main() {
    println!("Getting Data from Scryfall!");
    let client = reqwest::Client::new();
    let scryfall_data = client.get(String::from("https://api.scryfall.com/bulk-data"))
        .header(USER_AGENT, "Scryfall Cacher")
        .header(ACCEPT, "application/json")
        .send()
        .await.expect("API Error")
        .json::<ScryfallBulkDataResponse>()
        .await.expect("JSON Error");
    println!("Got data from Scryfall!");

    println!("Creating initial folder!");
    create_dir("/initial").ok();
    println!("Created initial folder!");

    println!("Getting the JSON files!");
    let mut handles = vec![];
    for dataset in scryfall_data.data {
        let handle = tokio::spawn(async {
            let result = download_file(dataset).await;
            result
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await.unwrap();
    }
    println!("Got the JSON files!");
}

async fn download_file(dataset: ScryfallBulkData) -> Result<&'static str, &'static str> {
    println!("Working on: {0}!", dataset.download_uri);
    let client = reqwest::Client::new();
    let mut output_file = File::create(format!("/initial/{0}.json", dataset.r#type).as_str())
        .expect(format!("Error making file for {0}!", dataset.r#type).as_str());

    println!("starting download of: {0}", dataset.r#type);
    let scryfall_data = client.get(&dataset.download_uri)
        .send().await.expect("Error API call failed")
        .text().await.expect("File Error");
    output_file.write(scryfall_data.as_bytes())
        .expect(format!("Error writing to file {0}!", dataset.r#type).as_str());
    println!("Finished Downloading: {0}", dataset.r#type);

    Ok("Success!")
} 
