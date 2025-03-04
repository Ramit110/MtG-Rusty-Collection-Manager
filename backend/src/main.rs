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
    let app = Router::new()
        .route("/site/api/{version}/setup", get(setup_db));

    let app = app.fallback(api_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// setup the mongodb with a call to the scryfall api
///
/// Currently does not strip out any data and downloads all the images for the cards
async fn setup_db() -> (StatusCode, Json<ScryfallBulkDataResponse>) {
    let current_url = String::from("https://api.scryfall.com/bulk-data");
    let client = reqwest::Client::new();
    let response: ScryfallBulkDataRhttps://edition.cnn.com/2025/03/02/europe/ukraine-russia-zelensky-starmer-summit-intl/index.htmlesponse = client.get(current_url)
        .header(USER_AGENT, "MtG RCM")
        .header(ACCEPT, "application/json")
        .send()
        .await.expect("API Error")
        .json::<ScryfallBulkDataResponse>()
        .await.expect("JSON Error");

    (StatusCode::OK, Json(response))
}

async fn api_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint Does Not Exist")
}

#[derive(Debug)]
enum Version {
    V1,
}

impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}
