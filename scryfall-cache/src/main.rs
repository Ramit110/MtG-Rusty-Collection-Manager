use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, RequestPartsExt, Router,
};
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    let scryfall_data = bulk_files();
}

/// setup the mongodb with a call to the scryfall api
///
/// Currently does not strip out any data and downloads all the images for the cards
async fn bulk_files() -> Json<ScryfallBulkDataResponse> {
    let current_url = String::from("https://api.scryfall.com/bulk-data");
    let client = reqwest::Client::new();
    let response: ScryfallBulkDataResponse = client.get(current_url)
        .header(USER_AGENT, "Scryfall Cacher")
        .header(ACCEPT, "application/json")
        .send()
        .await.expect("API Error")
        .json::<ScryfallBulkDataResponse>()
        .await.expect("JSON Error");

    Json(response)
}
