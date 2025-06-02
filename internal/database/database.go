package database

import (
    "database/sql"
    "fmt"
    "os"
    "path/filepath"

    _ "github.com/mattn/go-sqlite3"
)

// DB represents the database connection
type DB struct {
    conn *sql.DB
}

// New creates a new database connection
func New(dbPath string) (*DB, error) {
    // Ensure directory exists
    if err := ensureDir(filepath.Dir(dbPath)); err != nil {
        return nil, fmt.Errorf("failed to create database directory: %w", err)
    }

    // Open database connection
    conn, err := sql.Open("sqlite3", dbPath)
    if err != nil {
        return nil, fmt.Errorf("failed to open database: %w", err)
    }

    // Test connection
    if err = conn.Ping(); err != nil {
        return nil, fmt.Errorf("failed to ping database: %w", err)
    }

    db := &DB{conn: conn}

    // Initialize schema
    if err = db.initSchema(); err != nil {
        conn.Close()
        return nil, fmt.Errorf("failed to initialize schema: %w", err)
    }

    return db, nil
}

// Close closes the database connection
func (db *DB) Close() error {
    return db.conn.Close()
}

// GetDB returns the raw database connection
func (db *DB) GetDB() *sql.DB {
    return db.conn
}

// Helper to ensure directory exists
func ensureDir(path string) error {
    return os.MkdirAll(path, 0755)
}

// initSchema creates the necessary tables
func (db *DB) initSchema() error {
    schemas := []string{
        `CREATE TABLE IF NOT EXISTS tracked_products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL UNIQUE,
            current_price REAL,
            original_price REAL,
            lowest_price REAL,
            last_updated TIMESTAMP
        )`,
        `CREATE TABLE IF NOT EXISTS price_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER,
            price REAL,
            recorded_at TIMESTAMP,
            FOREIGN KEY (product_id) REFERENCES tracked_products(id)
        )`,
        `CREATE TABLE IF NOT EXISTS validations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entity_type TEXT NOT NULL,
            entity_id TEXT NOT NULL,
            status TEXT NOT NULL,
            message TEXT,
            checked_at TIMESTAMP
        )`,
    }

    for _, schema := range schemas {
        if _, err := db.conn.Exec(schema); err != nil {
            return fmt.Errorf("failed to create schema: %w", err)
        }
    }

    return nil
}
