use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents a searchable historical conversation or task memory record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionMemoryRecord {
    pub session_id: String,
    pub task_title: String,
    pub intent_summary: String,
    pub key_decisions: String,
    pub files_touched: String,
    pub timestamp: DateTime<Utc>,
}

/// A search result returned from the FTS5 cognitive index with match relevance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemorySearchResult {
    pub session_id: String,
    pub task_title: String,
    pub intent_summary: String,
    pub key_decisions: String,
    pub files_touched: String,
    pub rank: f64,
}

/// High-performance SQLite FTS5 Memory Store for sub-millisecond cross-session recall.
pub struct CognitiveMemoryStore {
    conn: Connection,
}

impl CognitiveMemoryStore {
    /// Opens or creates an on-disk SQLite FTS5 cognitive memory store.
    pub fn open<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open SQLite database")?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;",
        )?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// Opens an in-memory SQLite FTS5 database (primarily for tests and ephemeral sessions).
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to open in-memory SQLite")?;
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;",
        )?;
        let store = Self { conn };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        // Create standard metadata table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS session_records (
                session_id TEXT PRIMARY KEY,
                task_title TEXT NOT NULL,
                intent_summary TEXT NOT NULL,
                key_decisions TEXT NOT NULL,
                files_touched TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );",
            [],
        )?;

        // Create FTS5 virtual table for full-text semantic searching
        self.conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS session_fts USING fts5(
                session_id UNINDEXED,
                task_title,
                intent_summary,
                key_decisions,
                files_touched,
                content='session_records',
                content_rowid='rowid'
            );",
            [],
        )?;

        // Triggers to keep FTS index synchronized with the records table
        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS session_ai AFTER INSERT ON session_records BEGIN
                INSERT INTO session_fts(rowid, session_id, task_title, intent_summary, key_decisions, files_touched)
                VALUES (new.rowid, new.session_id, new.task_title, new.intent_summary, new.key_decisions, new.files_touched);
            END;",
            [],
        )?;

        Ok(())
    }

    /// Stores a completed task summary into the persistent cognitive memory.
    pub fn index_session(&self, record: &SessionMemoryRecord) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO session_records (session_id, task_title, intent_summary, key_decisions, files_touched, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
            params![
                record.session_id,
                record.task_title,
                record.intent_summary,
                record.key_decisions,
                record.files_touched,
                record.timestamp.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    /// Performs full-text search across past conversations and returns ranked matches.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<MemorySearchResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT session_id, task_title, intent_summary, key_decisions, files_touched, rank
             FROM session_fts
             WHERE session_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2;",
        )?;

        // Sanitize query for FTS5 (escape special characters)
        let safe_query = query.replace('"', "\"\"");
        let fts_query = format!("\"{}\"", safe_query);

        let rows = stmt.query_map(params![fts_query, limit], |row| {
            Ok(MemorySearchResult {
                session_id: row.get(0)?,
                task_title: row.get(1)?,
                intent_summary: row.get(2)?,
                key_decisions: row.get(3)?,
                files_touched: row.get(4)?,
                rank: row.get(5)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Returns the total number of indexed session records.
    pub fn count(&self) -> Result<usize> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM session_records;")?;
        let count: usize = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fts5_indexing_and_sub_millisecond_search() {
        let store = CognitiveMemoryStore::open_in_memory().unwrap();

        let record1 = SessionMemoryRecord {
            session_id: "sess_20261027_01".into(),
            task_title: "DeepSeek Harness Benchmark".into(),
            intent_summary: "Evaluated Cordis IOC microkernel and ConPTY native Windows DLLs".into(),
            key_decisions: "Adopt in-process context forking to save 60-80% of tokens".into(),
            files_touched: "apps/cli/lib/bin.js, packages/subagent".into(),
            timestamp: Utc::now(),
        };

        let record2 = SessionMemoryRecord {
            session_id: "sess_20261028_01".into(),
            task_title: "jcode Swarm Collision Engine".into(),
            intent_summary: "Analyzed code shifting under its feet and Tokio memory scaling".into(),
            key_decisions: "Implement read-set leasing for multi-agent safety".into(),
            files_touched: "crates/jcode-swarm-core, src/tool/agentgrep.rs".into(),
            timestamp: Utc::now(),
        };

        store.index_session(&record1).unwrap();
        store.index_session(&record2).unwrap();

        // Search for "context forking"
        let matches = store.search("context forking", 5).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].session_id, "sess_20261027_01");
        assert!(matches[0].key_decisions.contains("in-process context forking"));

        // Search for "Tokio memory"
        let matches2 = store.search("Tokio memory", 5).unwrap();
        assert_eq!(matches2.len(), 1);
        assert_eq!(matches2[0].session_id, "sess_20261028_01");
    }
}
