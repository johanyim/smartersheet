use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct AddRowRequest {
    pub to_top: bool,
    pub cells: Vec<AddRowRequestCell>,
}

#[derive(Debug, Serialize)]
pub struct AddRowRequestCell {
    #[serde(rename = "columnId")]
    pub column_id: u64,
    pub value: Value,
    pub strict: Option<bool>,
}
