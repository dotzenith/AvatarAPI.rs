use std::env;

use anyhow::Result;
use serde::Serialize;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize)]
pub struct Quote {
    pub quote: String,
    pub character: String,
    pub nation: String,
    pub bending: String,
    pub episode: String,
    pub book: String,
}

#[derive(Clone)]
pub struct Database {
    connection: SqlitePool,
}
impl Database {
    pub async fn new() -> Result<Self> {
        Ok(Database {
            connection: SqlitePool::connect(&env::var("DATABASE_URL")?).await?,
        })
    }

    #[allow(non_snake_case)]
    pub async fn random(&self, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(r#"SELECT * FROM Quotes ORDER BY RANDOM() LIMIT ?1"#, num)
            .fetch_all(&self.connection)
            .await?;

        Ok(quotes.into_iter().map(|q| {
            Quote {
                quote: q.Quote, 
                character: q.Character,
                nation: q.Nation,
                bending: q.Bending,
                episode: q.Episode,
                book: q.Book,
            }
        }).collect())
    }
}
