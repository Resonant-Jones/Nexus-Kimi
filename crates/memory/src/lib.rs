use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub id: String,
    pub data: String,
}

pub struct Memory {
    conn: Connection,
}

impl Memory {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memory (
                id   TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn put(&self, rec: &Record) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memory(id, data) VALUES (?1, ?2)",
            params![rec.id, rec.data],
        )?;
        Ok(())
    }

    pub fn get(&self, id: &str) -> anyhow::Result<Option<Record>> {
        let mut stmt = self.conn.prepare("SELECT id, data FROM memory WHERE id = ?1")?;
        let mut rows = stmt.query_map([id], |row| {
            Ok(Record {
                id: row.get(0)?,
                data: row.get(1)?,
            })
        })?;
        Ok(rows.next().transpose()?)
    }
}