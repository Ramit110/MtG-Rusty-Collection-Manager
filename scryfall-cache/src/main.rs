use reqwest::header::{ACCEPT, CONTENT_LENGTH, RANGE, USER_AGENT, HeaderValue};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::{File, create_dir};
use std::thread;

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

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self, &'static str> {
        if buffer_size == 0 {
            return Err("invalid buffer_size, give a value greater than zero")
        }
        if start > end {
            return Err("invalid start and end, start is greater than end.")
        }
        Ok(PartialRangeIter { start, end, buffer_size, })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
      if self.start > self.end {
        None
      } else {
        let prev_start = self.start;
        self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
        Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
            .expect("string provided by format!"))
      }
    }
  }
  

#[tokio::main]
async fn main() {
    println!("Getting Data from Scryfall!");
    let scryfall_data = bulk_files();
    println!("Got data from Scryfall!");

    println!("Creating initial folder!");
    create_dir("/initial").ok();
    println!("Created initial folder!");

    println!("Getting the JSON files!");

}

fn download_file(data: ScryfallBulkData) -> Result<bool, Box<dyn Error>> {
    /*
    const CHUNK_SIZE: u128 = data.size.overflowing_div(50).0;
    let client = reqwest::blocking::Client::new();
    let response = client.head(data.uri).send()?;
    let length = response.headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
      
    let mut output_file = File::create("/initial/")?;
      
    println!("starting download of:");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send()?;
      
        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            error_chain::bail!("Unexpected server response: {}", status)
        }
        std::io::copy(&mut response, &mut output_file)?;
    }
      
    let content = response.text()?;
    std::io::copy(&mut content.as_bytes(), &mut output_file)?;

    println!("Success! Finished Downloading:");
    */
    Ok(true)
} 

/// setup the mongodb with a call to the scryfall api
///
/// Currently does not strip out any data and downloads all the images for the cards
async fn bulk_files() -> Result<ScryfallBulkDataResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(String::from("https://api.scryfall.com/bulk-data"))
        .header(USER_AGENT, "Scryfall Cacher")
        .header(ACCEPT, "application/json")
        .send()
        .await.expect("API Error")
        .json::<ScryfallBulkDataResponse>()
        .await.expect("JSON Error");

    Ok(response)
}
