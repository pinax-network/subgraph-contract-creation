use substreams::{pb::substreams::Clock, Hex};

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    if bytes.is_empty() {
        return "".to_string();
    } else {
        format! {"0x{}", Hex::encode(bytes)}.to_string()
    }
}

pub fn to_date(clock: Clock) -> String {
    let timestamp = clock.timestamp.expect("missing clock");
    timestamp.to_string().split("T").next().expect("missing date").to_string()
}

// must return 2024-01-01
pub fn to_month(clock: Clock) -> String {
    let block_date = to_date(clock);
    let parts: Vec<&str> = block_date.split('-').take(2).collect();
    format!("{}-01", parts.join("-"))
}
