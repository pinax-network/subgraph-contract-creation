use std::collections::HashMap;

use substreams::errors::Error;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};

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
            // .change("init", ("", event.init.to_string().as_str())); // NOT IMPLEMENTED
            .change("code", ("", event.code.to_string().as_str()));
    }

    Ok(db_out)
}

#[substreams::handlers::map]
pub fn graph_out(map: Events) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for event in map.data {
        let pk = format!("{}-{}", event.block_hash, event.contract_address);
        entity_changes
            .push_change("contract_creation", pk.as_str(), 0, entity_change::Operation::Create)
            // block
            .change("block_hash", event.block_hash.to_string())
            .change("block_number", event.block_number.to_string())
            .change("block_time", event.block_time.to_string())
            .change("block_date", event.block_date.to_string())
            // contract creation
            .change("contract_address", event.contract_address.to_string())
            .change("creator_address", event.creator_address.to_string())
            .change("creator_factory", event.creator_factory.to_string())
            .change("creator_tx", event.creator_tx.to_string())
            // .change("init", event.init.to_string()) // NOT IMPLEMENTED
            .change("code", &event.code);
    }

    Ok(entity_changes)
}
