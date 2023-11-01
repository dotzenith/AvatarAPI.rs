use crate::database;
use serde::Serialize;
use axum::{Json, extract::State, response::{Response, IntoResponse}};

#[derive(Serialize)]
pub struct Result {
    num: usize,
    quotes: Vec<database::Quote>
}

pub async fn random(State(database): State<database::Database>) -> Response {
    match database.random(5).await {
        Ok(res) => {
            Json(Result {
                num: res.len(),
                quotes: res
            }).into_response()
        }
        Err(_) => {
            "An error happened while trying to get random quotes".into_response()
        }
    } 
}
