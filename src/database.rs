use anyhow::Result;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool};
use sqlx::Row;

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

#[derive(Clone, Copy, Debug)]
pub enum Column {
    Character,
    Nation,
    Bending,
    Episode,
    Book,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_lifetime(None)
            .idle_timeout(None)
            .connect("sqlite::memory:")
            .await?;
        let mut rdr = ReaderBuilder::new().delimiter(b'|').from_path(path)?;
        sqlx::query(
            r#"CREATE TABLE quotes(
              quote TEXT NOT NULL,
              character TEXT NOT NULL,
              nation TEXT NOT NULL,
              bending TEXT NOT NULL,
              episode TEXT NOT NULL,
              book TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        for result in rdr.records() {
            let record = result?;
            sqlx::query(
                "INSERT INTO quotes (quote, character, nation, bending, episode, book) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .bind(&record[0])
            .bind(&record[1])
            .bind(&record[2])
            .bind(&record[3])
            .bind(&record[4])
            .bind(&record[5])
            .execute(&pool)
            .await?;
        }

        Ok(Database { pool })
    }

    pub async fn random(&self, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes ORDER BY RANDOM() LIMIT ?1"#)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn character(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes WHERE character = ?1 ORDER BY RANDOM() LIMIT ?2"#)
            .bind(value)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn nation(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes WHERE nation = ?1 ORDER BY RANDOM() LIMIT ?2"#)
            .bind(value)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn bending(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes WHERE bending = ?1 ORDER BY RANDOM() LIMIT ?2"#)
            .bind(value)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn episode(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes WHERE episode = ?1 ORDER BY RANDOM() LIMIT ?2"#)
            .bind(value)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn book(&self, value: &str, num: u8) -> Result<Vec<Quote>> {
        let quotes = sqlx::query(r#"SELECT * FROM quotes WHERE book = ?1 ORDER BY RANDOM() LIMIT ?2"#)
            .bind(value)
            .bind(num)
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes
            .into_iter()
            .map(|q| Quote {
                quote: q.get_unchecked(0),
                character: q.get_unchecked(1),
                nation: q.get_unchecked(2),
                bending: q.get_unchecked(3),
                episode: q.get_unchecked(4),
                book: q.get_unchecked(5),
            })
            .collect())
    }

    pub async fn get_all(&self, column: Column) -> Result<Vec<String>> {
        let col = match column {
            Column::Character => "character",
            Column::Nation => "nation",
            Column::Bending => "bending",
            Column::Episode => "episode",
            Column::Book => "book",
        };
        let quotes = sqlx::query(&format!("SELECT DISTINCT {} FROM quotes", col))
            .fetch_all(&self.pool)
            .await?;

        Ok(quotes.into_iter().map(|q| q.get(0)).collect())
    }
}
