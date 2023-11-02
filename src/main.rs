mod database;
mod handlers;

use axum::{routing::get, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).compact().init();

    let addr = "0.0.0.0:3000";
    tracing::info!("Started listening on: {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn app() -> Router {
    let db = database::Database::new().await.unwrap();
    Router::new()
        .route("/api/quotes", get(handlers::random))
        .route("/api/quotes/character", get(handlers::character))
        .route("/api/quotes/nation", get(handlers::nation))
        .route("/api/quotes/bending", get(handlers::bending))
        .route("/api/quotes/episode", get(handlers::episode))
        .route("/api/quotes/book", get(handlers::book))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_all_endpoints_with_valid_inputs_check_num() {
        assert_eq!(send_request_get_body("/api/quotes", "").await.num, 5);
        assert_eq!(
            send_request_get_body("/api/quotes/character", "value=Aang").await.num,
            5
        );
        assert_eq!(send_request_get_body("/api/quotes/nation", "value=Fire").await.num, 5);
        assert_eq!(send_request_get_body("/api/quotes/bending", "value=All").await.num, 5);
        assert_eq!(
            send_request_get_body("/api/quotes/episode", "value=Imprisoned")
                .await
                .num,
            5
        );
        assert_eq!(send_request_get_body("/api/quotes/book", "value=Earth").await.num, 5);
    }

    #[tokio::test]
    async fn test_non_default_num() {
        assert_eq!(send_request_get_body("/api/quotes", "num=6").await.num, 6);
    }

    #[tokio::test]
    async fn test_endpoints_with_invalid_value() {
        assert_eq!(send_request_get_body("/api/quotes/character", "value=Ong").await.num, 0);
        assert_eq!(send_request_get_body("/api/quotes/nation", "value=Swamp").await.num, 0);
        assert_eq!(send_request_get_body("/api/quotes/bending", "value=Blood").await.num, 0);
        assert_eq!(send_request_get_body("/api/quotes/episode", "value=Hello").await.num, 0);
        assert_eq!(send_request_get_body("/api/quotes/book", "value=Moon").await.num, 0);
    }

    #[tokio::test]
    async fn test_correct_num_despite_query_param() {
        assert_eq!(
            send_request_get_body("/api/quotes/character", "value=Koh&num=5")
                .await
                .num,
            2
        );
    }

    #[tokio::test]
    async fn test_endpoints_with_valid_inputs_check_value() {
        assert_eq!(
            send_request_get_body("/api/quotes/character", "value=Aang")
                .await
                .quotes
                .first()
                .unwrap()
                .character,
            "Aang"
        );
        assert_eq!(
            send_request_get_body("/api/quotes/nation", "value=Fire")
                .await
                .quotes
                .first()
                .unwrap()
                .nation,
            "Fire"
        );
        assert_eq!(
            send_request_get_body("/api/quotes/bending", "value=None")
                .await
                .quotes
                .first()
                .unwrap()
                .bending,
            "None"
        );
        assert_eq!(
            send_request_get_body("/api/quotes/episode", "value=Imprisoned")
                .await
                .quotes
                .first()
                .unwrap()
                .episode,
            "Imprisoned"
        );
        assert_eq!(
            send_request_get_body("/api/quotes/book", "value=Fire")
                .await
                .quotes
                .first()
                .unwrap()
                .book,
            "Fire"
        );
    }

    async fn send_request_get_body(uri: &str, query: &str) -> handlers::Result {
        let response = app()
            .await
            .oneshot(
                Request::builder()
                    .uri(format!("{}?{}", uri, query))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: handlers::Result = serde_json::from_slice(&body).unwrap();
        body
    }
}
