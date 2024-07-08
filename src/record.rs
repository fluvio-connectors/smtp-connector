use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Address {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct MailAttachment {
    pub content_type: String,
    pub file_name: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct SmtpRecord {
    pub subject: String,
    pub body: String,
    pub from: Address,
    pub to: Address,
    pub attachments: Option<Vec<MailAttachment>>,
}
