use crate::database;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub num: usize,
    pub quotes: Vec<database::Quote>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    value: String,
    num: Option<u8>,
}

#[derive(Deserialize)]
pub struct RandomParams {
    num: Option<u8>,
}

pub async fn random(Query(params): Query<RandomParams>, State(database): State<database::Database>) -> Response {
    match database.random(params.num.unwrap_or(5)).await {
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

pub async fn character(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    match database.character(&params.value, params.num.unwrap_or(5)).await {
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

pub async fn nation(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    match database.nation(&params.value, params.num.unwrap_or(5)).await {
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

pub async fn bending(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    match database.bending(&params.value, params.num.unwrap_or(5)).await {
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

pub async fn episode(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    match database.episode(&params.value, params.num.unwrap_or(5)).await {
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

pub async fn book(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    match database.book(&params.value, params.num.unwrap_or(5)).await {
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
