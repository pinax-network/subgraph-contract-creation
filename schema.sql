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
    block_hash              String,
    block_number            UInt64,
    block_timestamp_seconds Uint64,
    contract_address        String,
    creator_address         String,
    creator_tx              String,
    creation_bytecode       String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (block_hash, contract_address)
        ORDER BY (block_hash, contract_address);