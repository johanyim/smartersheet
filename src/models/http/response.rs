use serde::Deserialize;
use serde_json::Value;

use crate::models::{Cell, Column, Row};

#[derive(Debug, Deserialize)]
pub struct SheetsResponse {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}
