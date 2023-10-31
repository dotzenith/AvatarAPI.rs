use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub character: String,
    pub nation: String,
    pub bending: String,
    pub episode: String,
    pub book: String,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Database {
            connection: Connection::open(path)?
        })
    }

    pub fn random(&self, num: usize) -> Result<Vec<Quote>> {
        let mut statement = self.connection.prepare("SELECT * FROM Quotes ORDER BY RANDOM() LIMIT (?1)")?;
        let iter = statement.query_map([num], |row| {
            Ok(Quote {
                quote: row.get(0)?,
                character: row.get(1)?,
                nation: row.get(2)?,
                bending: row.get(3)?,
                episode: row.get(4)?,
                book: row.get(5)?,
            })
        })?;
        iter.collect()
    }
}
