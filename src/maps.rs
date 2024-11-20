use substreams::{errors::Error, pb::substreams::Clock};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::{
    pb::contract_creation::{ContractCreation, Events},
    utils::{bytes_to_hex, to_date, to_month},
};

#[substreams::handlers::map]
fn map_contract_creation(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let block_hash = format!("0x{}", clock.id);
    let block_number = clock.number;
    let timestamp = clock.timestamp.expect("missing block timestamp");
    let block_date = to_date(clock.clone());
    let block_month = to_month(clock);

    for trace in block.transaction_traces {
        for call in trace.calls {
            if call.call_type() == CallType::Create {
                for code in call.code_changes {
                    let from = bytes_to_hex(&trace.from);
                    let to = bytes_to_hex(&trace.to);
                    let factory = if trace.to == code.address { "".to_string() } else { to.clone() };
                    let deployer = if factory.is_empty() { from.clone() } else { to.clone() };
                    // TO-DO: should `code` be required to push event?

                    events.contract_creations.push(ContractCreation {
                        // block
                        block_hash: block_hash.to_string(),
                        block_number,
                        block_time: Some(timestamp),
                        block_date: block_date.to_string(),
                        block_month: block_month.to_string(),
                        // transaction
                        transaction_hash: bytes_to_hex(&trace.hash),
                        transaction_index: trace.index,
                        // contract creation
                        ordinal: code.ordinal,
                        address: bytes_to_hex(&code.address),
                        from,
                        to,
                        deployer,
                        factory: Some(factory),
                        code: Some(bytes_to_hex(&code.new_code)),
                        code_hash: Some(bytes_to_hex(&code.new_hash)),
                        input: Some(bytes_to_hex(&call.input)),
                    });
                }
            }
        }
    }
    Ok(events)
}

// used for ./block-index substreams
// this function will filter out all the transaction traces that are not contract creations
// improves Substreams caching performance by reducing the amount of bytes that needs to be read
#[substreams::handlers::map]
fn map_block_index(block: Block) -> Result<Block, Error> {
    let mut indexed_block = Block::default();
    indexed_block.header = block.header;
    indexed_block.code_changes = block.code_changes;

    for trace in block.transaction_traces.into_iter() {
        for call in trace.clone().calls {
            if call.call_type() == CallType::Create {
                indexed_block.transaction_traces.push(trace);
                break;
            }
        }
    }
    Ok(indexed_block)
}
