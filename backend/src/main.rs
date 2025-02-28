use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, RequestPartsExt, Router,
};
use serde::{Serialize, Deserialize};
use reqwest::{Error, header::USER_AGENT};
use std::collections::HashMap;

#[derive(Serialize)]
struct Card {
    data: &'static str,
}

#[derive(Serialize)]
struct ScryfallResponse {
    object: &'static str,
    total_cards: &'static str,
    has_more: &'static str,
    next_page: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/site/api/{version}/setup", get(setup_db))
        .route("/site/api/{version}/cards", get(get_cards));

    let app = app.fallback(api_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// setup the mongodb with a call to the scryfall api
///
/// Currently does not strip out any data and downloads all the images for the cards
async fn setup_db() -> Result<(StatusCode, Json<ScryfallResponse>), Error> {
    let current_url = String::from("https://api.scryfall.com/cards/search?q=game%3Apaper");
    let client = reqwest::Client::new();
    let response = client.get(current_url)
        // .header(USER_AGENT, "MtG RCM")
        .send()
        .await?;
    let cards: ScryfallResponse = response.json().await?;

    Ok((StatusCode::OK, Json(cards)))
}

/// Returns the data on the first 100 cards in the mongodb database
async fn get_cards() -> (StatusCode, Json<Card>) {
    let response = Card {
        data: "Hello, world!",
    };

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
