use crate::database;
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(validate(greater = 0), derive(Debug, Deserialize, Copy, Clone))]
pub struct Number(u8);

#[derive(Serialize, Deserialize)]
pub struct QuoteResult {
    pub num: usize,
    pub quotes: Vec<database::Quote>,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnResult {
    pub num: usize,
    pub values: Vec<String>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    value: String,
    num: Option<Number>,
}

#[derive(Deserialize)]
pub struct RandomParams {
    num: Option<Number>,
}

// Handlers for Quotes
pub async fn random(Query(params): Query<RandomParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .random(params.num.unwrap_or(Number::new(5).unwrap()).into_inner())
            .await,
    )
    .await
}

pub async fn character(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .character(
                &params.value,
                params.num.unwrap_or(Number::new(5).unwrap()).into_inner(),
            )
            .await,
    )
    .await
}

pub async fn nation(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .nation(
                &params.value,
                params.num.unwrap_or(Number::new(5).unwrap()).into_inner(),
            )
            .await,
    )
    .await
}

pub async fn bending(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .bending(
                &params.value,
                params.num.unwrap_or(Number::new(5).unwrap()).into_inner(),
            )
            .await,
    )
    .await
}

pub async fn episode(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .episode(
                &params.value,
                params.num.unwrap_or(Number::new(5).unwrap()).into_inner(),
            )
            .await,
    )
    .await
}

pub async fn book(Query(params): Query<QueryParams>, State(database): State<database::Database>) -> Response {
    wrap(
        database
            .book(
                &params.value,
                params.num.unwrap_or(Number::new(5).unwrap()).into_inner(),
            )
            .await,
    )
    .await
}

async fn wrap(res: Result<Vec<database::Quote>>) -> Response {
    match res {
        Ok(res) => {
            let status = match res.len() {
                0 => StatusCode::BAD_REQUEST,
                _ => StatusCode::OK,
            };
            let result = Json(QuoteResult {
                num: res.len(),
                quotes: res,
            });
            (status, result).into_response()
        }

        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

// Handlers to get all unique values of a column
pub async fn all_characters(State(database): State<database::Database>) -> Response {
    all(&database, database::Column::Character).await
}

pub async fn all_nation(State(database): State<database::Database>) -> Response {
    all(&database, database::Column::Nation).await
}

pub async fn all_bending(State(database): State<database::Database>) -> Response {
    all(&database, database::Column::Bending).await
}

pub async fn all_episode(State(database): State<database::Database>) -> Response {
    all(&database, database::Column::Episode).await
}

pub async fn all_book(State(database): State<database::Database>) -> Response {
    all(&database, database::Column::Book).await
}

async fn all(db: &database::Database, column: database::Column) -> Response {
    match db.get_all(column).await {
        Ok(res) => {
            let result = Json(ColumnResult {
                num: res.len(),
                values: res,
            });
            (StatusCode::OK, result).into_response()
        }

        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
