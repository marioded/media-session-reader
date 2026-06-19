use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Cover {
    pub data: Vec<u8>,

    pub mime: String,
}
