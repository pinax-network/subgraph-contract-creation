use std::collections::HashMap;

use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::pb::contract_creation::v1::Events;

#[substreams::handlers::map]
pub fn db_out(map: Events) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for event in map.data {
        let pk = HashMap::from([("block_hash".to_string(), event.block_hash.clone()), ("contract_address".to_string(), event.contract_address.clone())]);
        db_out
            .push_change_composite("contract_creation", pk, 0, table_change::Operation::Create)
            // block
            .change("block_hash", ("", event.block_hash.to_string().as_str()))
            .change("block_number", ("", event.block_number.to_string().as_str()))
            .change("block_time", ("", event.block_time.to_string().as_str()))
            .change("block_date", ("", event.block_date.to_string().as_str()))
            // contract creation
            .change("contract_address", ("", event.contract_address.to_string().as_str()))
            .change("creator_address", ("", event.creator_address.to_string().as_str()))
            .change("creator_factory", ("", event.creator_factory.to_string().as_str()))
            .change("creator_tx", ("", event.creator_tx.to_string().as_str()))
            // .change("init", ("", event.init.to_string().as_str())); // NOT IMPLEMENTED;
            .change("code", ("", event.code.unwrap().to_string().as_str()));
    }

    Ok(db_out)
}

#[substreams::handlers::map]
pub fn graph_out(map: Events) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in map.data {
        let pk = format!("{}-{}", event.block_hash, event.contract_address);
        let row = tables
            .create_row("ContractCreation", pk.as_str())
            // block
            .set("block_hash", event.block_hash.to_string())
            .set_bigint("block_number", &event.block_number.to_string())
            .set_bigint("block_time", &event.block_time.to_string())
            .set("block_date", event.block_date.to_string())
            // contract creation
            .set("contract_address", event.contract_address.to_string())
            .set("creator_address", event.creator_address.to_string())
            .set("creator_factory", event.creator_factory.to_string())
            .set("creator_tx", event.creator_tx.to_string());

        if !event.code.is_some() {
            row.set("code", &event.code.unwrap().to_string());
            // .set("init", event.init.to_string()) // NOT IMPLEMENTED
        }
    }

    Ok(tables.to_entity_changes())
}
