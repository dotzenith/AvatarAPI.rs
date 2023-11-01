use crate::database;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Result {
    num: usize,
    quotes: Vec<database::Quote>,
}

#[derive(Deserialize)]
pub struct Params {
    name: String,
    num: Option<u8>,
}

pub async fn random(State(database): State<database::Database>) -> Response {
    match database.random(5).await {
        Ok(res) => {
            let result = Json(Result {
                num: res.len(),
                quotes: res,
            });
            (StatusCode::OK, result).into_response()
        }

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error happened while trying to get random quotes",
        )
            .into_response(),
    }
}

pub async fn character(
    Query(params): Query<Params>,
    State(database): State<database::Database>,
) -> Response {
    match database
        .character(&params.name, params.num.unwrap_or(5))
        .await
    {
        Ok(res) => {
            let status = match res.len() {
                0 => StatusCode::BAD_REQUEST,
                _ => StatusCode::OK,
            };
            let result = Json(Result {
                num: res.len(),
                quotes: res,
            });
            (status, result).into_response()
        }

        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
