//! SQLite storage for XP and growth data.

use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

use crate::stats::StatName;

pub struct Database {
    conn: Connection,
}

/// XP totals for all stats.
#[derive(Debug, Default)]
pub struct XpTotals {
    pub debugging: u32,
    pub patience: u32,
    pub chaos: u32,
    pub wisdom: u32,
    pub snark: u32,
    pub total_events: u32,
}

impl XpTotals {
    pub fn get(&self, stat: StatName) -> u32 {
        match stat {
            StatName::Debugging => self.debugging,
            StatName::Patience => self.patience,
            StatName::Chaos => self.chaos,
            StatName::Wisdom => self.wisdom,
            StatName::Snark => self.snark,
        }
    }
}

impl Database {
    pub fn open() -> Result<Self> {
        let path = db_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS xp (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                seed TEXT NOT NULL,
                stat TEXT NOT NULL,
                amount INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_xp_seed ON xp(seed)",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_xp(&self, seed: &str, stat: &StatName, amount: u32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO xp (seed, stat, amount) VALUES (?1, ?2, ?3)",
            params![seed, stat.name(), amount],
        )?;
        Ok(())
    }

    pub fn get_xp(&self, seed: &str) -> Result<XpTotals> {
        let mut stmt = self.conn.prepare(
            "SELECT stat, SUM(amount), COUNT(*) FROM xp WHERE seed = ?1 GROUP BY stat",
        )?;

        let mut totals = XpTotals::default();

        let rows = stmt.query_map([seed], |row| {
            let stat: String = row.get(0)?;
            let amount: u32 = row.get(1)?;
            let count: u32 = row.get(2)?;
            Ok((stat, amount, count))
        })?;

        for row in rows {
            let (stat, amount, count) = row?;
            totals.total_events += count;
            match stat.as_str() {
                "DEBUGGING" => totals.debugging = amount,
                "PATIENCE" => totals.patience = amount,
                "CHAOS" => totals.chaos = amount,
                "WISDOM" => totals.wisdom = amount,
                "SNARK" => totals.snark = amount,
                _ => {}
            }
        }

        Ok(totals)
    }

    pub fn reset_xp(&self, seed: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM xp WHERE seed = ?1", [seed])?;
        Ok(())
    }
}

fn db_path() -> Result<PathBuf> {
    let base = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
    Ok(base.join("terminalgotchi").join("growth.db"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_db() -> Database {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.db");
        let conn = Connection::open(&path).unwrap();
        conn.execute(
            "CREATE TABLE xp (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                seed TEXT NOT NULL,
                stat TEXT NOT NULL,
                amount INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )
        .unwrap();
        // Leak TempDir so it doesn't get cleaned up during test
        std::mem::forget(dir);
        Database { conn }
    }

    #[test]
    fn add_and_get_xp() {
        let db = test_db();
        db.add_xp("alice", &StatName::Debugging, 10).unwrap();
        db.add_xp("alice", &StatName::Debugging, 5).unwrap();
        db.add_xp("alice", &StatName::Chaos, 3).unwrap();

        let xp = db.get_xp("alice").unwrap();
        assert_eq!(xp.debugging, 15);
        assert_eq!(xp.chaos, 3);
        assert_eq!(xp.patience, 0);
        assert_eq!(xp.total_events, 3);
    }

    #[test]
    fn reset_xp_clears() {
        let db = test_db();
        db.add_xp("bob", &StatName::Wisdom, 20).unwrap();
        db.reset_xp("bob").unwrap();

        let xp = db.get_xp("bob").unwrap();
        assert_eq!(xp.wisdom, 0);
        assert_eq!(xp.total_events, 0);
    }

    #[test]
    fn separate_seeds() {
        let db = test_db();
        db.add_xp("alice", &StatName::Snark, 10).unwrap();
        db.add_xp("bob", &StatName::Snark, 20).unwrap();

        assert_eq!(db.get_xp("alice").unwrap().snark, 10);
        assert_eq!(db.get_xp("bob").unwrap().snark, 20);
    }
}
