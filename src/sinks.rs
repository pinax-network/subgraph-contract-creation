use substreams::errors::Error;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use crate::pb::contract_creation::v1::Events;

#[substreams::handlers::map]
pub fn graph_out(map: Events) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in map.contract_creations {
        let pk = format!("{}-{}", event.block_hash, event.address);
        // TO-DO: should primary key be `event.code_hash`?
        let row = tables
            .create_row("ContractCreation", pk.as_str())
            // block
            .set("block_hash", event.block_hash.to_string())
            .set_bigint("block_number", &event.block_number.to_string())
            .set_bigint("block_time", &event.block_time.unwrap().seconds.to_string())
            .set("block_date", event.block_date.to_string())
            .set("block_month", event.block_month.to_string())
            // transaction
            .set("transaction_hash", event.transaction_hash.to_string())
            .set_bigint("transaction_index", &event.transaction_index.to_string())
            // contract creation
            .set_bigint("ordinal", &event.ordinal.to_string())
            .set("address", event.address.to_string())
            .set("from", event.from.to_string())
            .set("to", event.to.to_string())
            .set("deployer", event.deployer.to_string());

        // OPTIONAL contract creation
        event.factory.as_ref().map(|factory| row.set("factory", factory.to_string()));
        event.code.as_ref().map(|code| row.set("code", code.to_string()));
        event.code_hash.as_ref().map(|code_hash| row.set("code_hash", code_hash.to_string()));
        event.input.as_ref().map(|input| row.set("input", input.to_string()));
    }

    Ok(tables.to_entity_changes())
}
