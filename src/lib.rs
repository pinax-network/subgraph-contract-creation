mod pb;
use std::collections::HashMap;

use pb::evm_contract_creation::v1::{ContractCreationInfo, EvmContractCreations};

use substreams::{errors::Error, log, Hex};
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    if bytes.is_empty() {
        return "".to_string();
    } else {
        format! {"0x{}", Hex::encode(bytes)}.to_string()
    }
}

#[substreams::handlers::map]
fn map_contract_creation(blk: Block) -> EvmContractCreations {
    let mut contract_creation = EvmContractCreations::default();
    let block_hash = Hex(&blk.hash).to_string();
    let block_number = blk.number.to_u64().unwrap();
    let block_timestamp = blk.timestamp_seconds();

    for trace in blk.transaction_traces {
        for call in trace.calls {
            if call.call_type() == CallType::Create {
                for code in call.code_changes {
                    let mut info = ContractCreationInfo::default();

                    info.block_hash = ("0x".to_owned() + &block_hash).to_owned();
                    info.block_number = block_number;
                    info.block_timestamp_seconds = block_timestamp;
                    info.contract_address = bytes_to_hex(&code.address);
                    info.creator_address = bytes_to_hex(&trace.from);
                    info.creator_factory = if trace.to == code.address {
                        "".to_string()
                    } else {
                        bytes_to_hex(&trace.to)
                    };
                    info.creator_tx = bytes_to_hex(&trace.hash);
                    info.contract_bytecode = code.new_code;

                    contract_creation.data.push(info);
                }
            }
        }
    }

    contract_creation
}

#[substreams::handlers::map]
pub fn db_out(map: EvmContractCreations) -> Result<DatabaseChanges, Error> {
    let mut db_out = DatabaseChanges::default();

    for event in map.data {
        let pk = HashMap::from([
            ("block_hash".to_string(), event.block_hash.clone()),
            (
                "contract_address".to_string(),
                event.contract_address.clone(),
            ),
        ]);
        db_out
            .push_change_composite("contract_creation", pk, 0, table_change::Operation::Create)
            .change("block_hash", ("", event.block_hash.to_string().as_str()))
            .change(
                "block_number",
                ("", event.block_number.to_string().as_str()),
            )
            .change(
                "block_timestamp_seconds",
                ("", event.block_timestamp_seconds.to_string().as_str()),
            )
            .change(
                "contract_address",
                ("", event.contract_address.to_string().as_str()),
            )
            .change(
                "creator_address",
                ("", event.creator_address.to_string().as_str()),
            )
            .change(
                "creator_factory",
                ("", event.creator_factory.to_string().as_str()),
            )
            .change("creator_tx", ("", event.creator_tx.to_string().as_str()))
            .change(
                "contract_init_bytecode",
                ("", bytes_to_hex(&event.contract_init_bytecode).as_str()),
            )
            .change(
                "contract_bytecode",
                ("", bytes_to_hex(&event.contract_bytecode).as_str()),
            );
    }

    Ok(db_out)
}

#[substreams::handlers::map]
pub fn graph_out(map: EvmContractCreations) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for event in map.data {
        let pk = format!("{}-{}", event.block_hash, event.contract_address);
        entity_changes
            .push_change(
                "contract_creation",
                pk.as_str(),
                0,
                entity_change::Operation::Create,
            )
            .change("block_hash", event.block_hash.to_string())
            .change("block_number", event.block_number.to_string())
            .change(
                "block_timestamp_seconds",
                event.block_timestamp_seconds.to_string(),
            )
            .change("contract_address", event.contract_address.to_string())
            .change("creator_address", event.creator_address.to_string())
            .change("creator_factory", event.creator_factory.to_string())
            .change("creator_tx", event.creator_tx.to_string())
            .change(
                "contract_init_bytecode",
                bytes_to_hex(&event.contract_init_bytecode),
            )
            .change("contract_bytecode", bytes_to_hex(&event.contract_bytecode));
    }

    Ok(entity_changes)
}
