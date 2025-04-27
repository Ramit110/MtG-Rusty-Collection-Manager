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
    let scryfall_data = reqwest::Client::new()
        .get(String::from("https://api.scryfall.com/bulk-data"))
        .header(USER_AGENT, "Scryfall Cacher")
        .header(ACCEPT, "application/json")
        .send().await.expect("API Error")
        .json::<ScryfallBulkDataResponse>().await.expect("JSON Parse Error")
        .data;
    println!("Got data from Scryfall!");

    println!("Creating initial folder!");
    create_dir("/initial").expect("Erustc --explain E0432rror Making Initial Folder");
    println!("Created initial folder!");

    println!("Getting the JSON files!");
    let mut handles = vec![];
    for dataset in scryfall_data {
        let handle = tokio::spawn(async move {
            println!("starting download of: {0}", dataset.r#type);
            let mut output_file = File::create(format!("/initial/{0}.json", dataset.r#type).as_str())
                .expect(format!("Error making file for {0}!", dataset.r#type).as_str());
        
            let scryfall_data_files = reqwest::Client::new()
                .get(&dataset.download_uri)
                .send().await.expect(format!("Error Getting Data File for {0}", dataset.r#type).as_str())
                .text().await.expect(format!("Error Parsing File For {0}", dataset.r#type).as_str());
        
            output_file.write(scryfall_data_files.as_bytes())
                .expect(format!("Error writing to file {0}!", dataset.r#type).as_str());
        
            println!("Finished Downloading: {0}", dataset.r#type);
            Ok("Success!")
        });
        handles.push(handle);
    }
    for handle in handles {
        let _: Result<&'static str, &'static str>  = handle.await.unwrap();
    }
    println!("Got the JSON files!");

    println!("Now to make it Mongo Importable!");
}
