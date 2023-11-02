use std::env;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pool: SqlitePool,
}
impl Database {
    pub async fn new() -> Result<Self> {
        let db = Database {
            pool: SqlitePool::connect("sqlite::memory:").await?,
        };
        sqlx::query(
            r#"
                CREATE TABLE quotes(
                  quote TEXT NOT NULL,
                  character TEXT NOT NULL,
                  nation TEXT NOT NULL,
                  bending TEXT NOT NULL,
                  episode TEXT NOT NULL,
                  book TEXT NOT NULL
                );
            "#,
        )
        .execute(&db.pool)
        .await?;

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .from_path(&env::var("QUOTES_PATH")?)?;

        for result in rdr.deserialize() {
            let quote: Quote = result?;
            sqlx::query!(
                r#"INSERT INTO quotes VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
                quote.quote,
                quote.character,
                quote.nation,
                quote.bending,
                quote.episode,
                quote.book,
            )
            .execute(&db.pool)
            .await?;
        }

        Ok(db)
    }

    pub async fn random(&self, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(r#"SELECT * FROM quotes ORDER BY RANDOM() LIMIT ?1"#, num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.quote,
                character: q.character,
                nation: q.nation,
                bending: q.bending,
                episode: q.episode,
                book: q.book,
            })
            .collect())
    }

    pub async fn character(&self, name: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE character = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            name,
            num
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.quote,
                character: q.character,
                nation: q.nation,
                bending: q.bending,
                episode: q.episode,
                book: q.book,
            })
            .collect())
    }
}
