-- Migration: 20250529000001_create_coupons_table
-- Description: Creates the coupons table for storing coupon information
-- Author: RinKokonoe

-- Up Migration
CREATE TABLE IF NOT EXISTS coupons (
    -- Primary key and identifier
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Coupon details
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    discount_percentage REAL,  -- NULL if unknown
    code TEXT NOT NULL,
    url TEXT NOT NULL,
    source TEXT NOT NULL,      -- The source where the coupon was found
    
    -- Timing information
    expiry TEXT,               -- ISO 8601 / RFC 3339 timestamp, NULL if no expiration
    created_at TEXT NOT NULL DEFAULT (datetime('now')),  -- When the coupon was added to the database
    validated_at TEXT,         -- When the coupon was last validated
    
    -- Status flags
    is_valid INTEGER NOT NULL DEFAULT 0,    -- 0 = false, 1 = true
    is_posted INTEGER NOT NULL DEFAULT 0,   -- 0 = false, 1 = true
    
    -- Deduplication
    hash TEXT NOT NULL UNIQUE  -- Hash for detecting duplicate coupons
);

-- Create indexes for performance
-- Index for source to quickly filter by coupon source
CREATE INDEX IF NOT EXISTS idx_coupons_source ON coupons(source);

-- Index for validation status to quickly find valid coupons
CREATE INDEX IF NOT EXISTS idx_coupons_valid ON coupons(is_valid);

-- Index for posting status to quickly find unposted coupons
CREATE INDEX IF NOT EXISTS idx_coupons_posted ON coupons(is_posted);

-- Index for expiry to quickly find and clean up expired coupons
CREATE INDEX IF NOT EXISTS idx_coupons_expiry ON coupons(expiry);

-- Down Migration
-- In case we need to roll back the migration
DROP INDEX IF EXISTS idx_coupons_expiry;
DROP INDEX IF EXISTS idx_coupons_posted;
DROP INDEX IF EXISTS idx_coupons_valid;
DROP INDEX IF EXISTS idx_coupons_source;
DROP TABLE IF EXISTS coupons;

