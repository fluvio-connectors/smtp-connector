use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize)]
pub struct Address {
    name: String,
    address: String,
}

#[derive(Debug, Deserialize)]
pub struct SmtpRecord {
    subject: String,
    body: String,
    from: Address,
    to: Address,
}
