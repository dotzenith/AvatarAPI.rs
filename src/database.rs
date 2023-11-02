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
        Ok(Database {
            pool: SqlitePool::connect(&env::var("DATABASE_URL")?).await?,
        })
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

    pub async fn character(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE character = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            value,
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

    pub async fn nation(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE nation = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            value,
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

    pub async fn bending(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE bending = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            value,
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

    pub async fn episode(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE episode = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            value,
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

    pub async fn book(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query!(
            r#"SELECT * FROM quotes WHERE book = ?1 ORDER BY RANDOM() LIMIT ?2"#,
            value,
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
