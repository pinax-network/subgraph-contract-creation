use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::{
    pb::contract_creation::v1::{ContractCreation, Events},
    utils::bytes_to_hex,
};

#[substreams::handlers::map]
fn map_contract_creation(clock: Clock, block: Block) -> Events {
    let mut events = Events::default();
    let block_hash = format!("0x{}", clock.id);
    let block_number = clock.number;
    let timestamp = clock.timestamp.expect("missing block timestamp");
    let block_time = timestamp.seconds;
    let block_date = timestamp.to_string().split("T").next().unwrap().to_string();

    for trace in block.transaction_traces {
        for call in trace.calls {
            if call.call_type() == CallType::Create {
                for code in call.code_changes {
                    let creator_factory = if trace.to == code.address { "".to_string() } else { bytes_to_hex(&trace.to) };

                    events.data.push(ContractCreation {
                        block_hash: block_hash.to_string(),
                        block_number,
                        block_time,
                        block_date: block_date.to_string(),
                        contract_address: bytes_to_hex(&code.address),
                        creator_address: bytes_to_hex(&trace.from),
                        creator_factory,
                        creator_tx: bytes_to_hex(&trace.hash),
                        code: Some(bytes_to_hex(&code.new_code)),
                        // init // NOT IMPLEMENTED
                    });
                }
            }
        }
    }
    events
}

// used for ./block-index substreams
// this function will filter out all the transaction traces that are not contract creations
// improves Substreams caching performance by reducing the amount of bytes that needs to be read
#[substreams::handlers::map]
fn map_block_index(block: Block) -> Block {
    let mut indexed_block = Block::default();
    indexed_block.code_changes = block.code_changes;

    for trace in block.transaction_traces.into_iter() {
        for call in trace.clone().calls {
            if call.call_type() == CallType::Create {
                indexed_block.transaction_traces.push(trace);
                break;
            }
        }
    }
    indexed_block
}
