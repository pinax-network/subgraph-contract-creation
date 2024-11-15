-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-----------------------------------------------------------
-- Tables to store the raw events without any processing --
-----------------------------------------------------------

CREATE TABLE IF NOT EXISTS contract_creation
(
    -- block --
    block_hash              String,
    block_number            UInt64,
    block_time              DateTime,
    block_date              Date,

    -- contract creation --
    contract_address        String,
    creator_factory         String,
    creator_address         String,
    creator_tx              String,
    code                    String,
    -- init                    String, -- NOT IMPLEMENTED
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_hash, contract_address)
        ORDER BY (block_hash, contract_address);