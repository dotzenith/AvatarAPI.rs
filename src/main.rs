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
        // Quotes
        .route("/api/quotes", get(handlers::random))
        .route("/api/quotes/character", get(handlers::character))
        .route("/api/quotes/nation", get(handlers::nation))
        .route("/api/quotes/bending", get(handlers::bending))
        .route("/api/quotes/episode", get(handlers::episode))
        .route("/api/quotes/book", get(handlers::book))
        // All distinct in a column
        .route("/api/all/character", get(handlers::all_characters))
        .route("/api/all/nation", get(handlers::all_nation))
        .route("/api/all/bending", get(handlers::all_bending))
        .route("/api/all/episode", get(handlers::all_episode))
        .route("/api/all/book", get(handlers::all_book))
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
    use axum::response::Response;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_all_quote_endpoints_with_valid_inputs_check_num() {
        assert_eq!(get_quote_request_body(send_quote_request_get_response("/api/quotes", "").await).await.num, 5);
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/character", "value=Aang").await)
                .await
                .num,
            5
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/nation", "value=Fire").await)
                .await
                .num,
            5
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/bending", "value=All").await)
                .await
                .num,
            5
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/episode", "value=Imprisoned").await)
                .await
                .num,
            5
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/book", "value=Earth").await)
                .await
                .num,
            5
        );
    }

    #[tokio::test]
    async fn test_quote_non_default_num() {
        assert_eq!(get_quote_request_body(send_quote_request_get_response("/api/quotes", "num=6").await).await.num, 6);
    }

    #[tokio::test]
    async fn test_quote_invalid_num() {
        assert_eq!(send_quote_request_get_response("/api/quotes", "num=256").await.status(), StatusCode::BAD_REQUEST);
        assert_eq!(send_quote_request_get_response("/api/quotes", "num=0").await.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_quote_endpoints_with_invalid_value() {
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/character", "value=Ong").await)
                .await
                .num,
            0
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/nation", "value=Swamp").await)
                .await
                .num,
            0
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/bending", "value=Blood").await)
                .await
                .num,
            0
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/episode", "value=Hello").await)
                .await
                .num,
            0
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/book", "value=Moon").await)
                .await
                .num,
            0
        );
    }

    #[tokio::test]
    async fn test_quote_endpoint_correct_num_despite_query_param() {
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/character", "value=Koh&num=5").await)
                .await
                .num,
            2
        );
    }

    #[tokio::test]
    async fn test_quote_endpoints_with_valid_inputs_check_value() {
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/character", "value=Aang").await)
                .await
                .quotes
                .first()
                .unwrap()
                .character,
            "Aang"
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/nation", "value=Fire").await)
                .await
                .quotes
                .first()
                .unwrap()
                .nation,
            "Fire"
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/bending", "value=None").await)
                .await
                .quotes
                .first()
                .unwrap()
                .bending,
            "None"
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/episode", "value=Imprisoned").await)
                .await
                .quotes
                .first()
                .unwrap()
                .episode,
            "Imprisoned"
        );
        assert_eq!(
            get_quote_request_body(send_quote_request_get_response("/api/quotes/book", "value=Fire").await)
                .await
                .quotes
                .first()
                .unwrap()
                .book,
            "Fire"
        );
    }

    #[tokio::test]
    async fn test_column_endpoints_for_an_expected_value() {
        assert!(send_column_request_get_body("/api/all/character")
            .await
            .values
            .contains(&"Aang".to_string()));
        assert!(send_column_request_get_body("/api/all/nation")
            .await
            .values
            .contains(&"Water".to_string()));
        assert!(send_column_request_get_body("/api/all/bending")
            .await
            .values
            .contains(&"All".to_string()));
        assert!(send_column_request_get_body("/api/all/episode")
            .await
            .values
            .contains(&"The Avatar Returns".to_string()));
        assert!(send_column_request_get_body("/api/all/book")
            .await
            .values
            .contains(&"Fire".to_string()));
    }

    async fn send_quote_request_get_response(uri: &str, query: &str) -> Response {
        app()
            .await
            .oneshot(
                Request::builder()
                    .uri(format!("{}?{}", uri, query))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn get_quote_request_body(response: Response) -> handlers::QuoteResult {
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: handlers::QuoteResult = serde_json::from_slice(&body).unwrap();
        body
    }

    async fn send_column_request_get_body(uri: &str) -> handlers::ColumnResult {
        let response = app()
            .await
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: handlers::ColumnResult = serde_json::from_slice(&body).unwrap();
        body
    }
}
