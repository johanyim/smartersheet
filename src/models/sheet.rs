use crate::prelude::*;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::api::Client;

#[derive(Debug)]
pub struct Sheet<'a: 'b, 'b> {
    pub client: &'b Client<'a>,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

impl<'a, 'b> Sheet<'a, 'b> {
    fn col_by_index(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }

    fn col_by_title(&self, title: String) -> Result<&Column> {
        let mut titles = self.columns.iter().filter(|col| col.title == title).take(2);

        match (titles.next(), titles.next()) {
            (Some(t), None) => Ok(t),
            (None, None) => Err(SmarterSheetError::CouldNotFindColumn(title)),
            (Some(_), Some(_)) => Err(SmarterSheetError::AmbiguousColumnTitle(title)),
            (None, Some(_)) => unreachable!(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Row {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    pub id: u64,
    pub index: u64,
    pub primary: Option<bool>,
    pub title: String,
    // might be an enum
    // r#type: CellType,
    pub r#type: String,
    pub validation: bool,
    pub version: u64,
    pub width: u64,
}

// impl<'a, 'b> Sheet<'a, 'b> {
//     async fn get_rows(&mut self) {
//         let mut client = self.client.lock().await;
//         client.sheets();
//     }
// }

// impl Sheet {
//
//
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    #[serde(rename = "columnId")]
    pub column_id: u64,
    #[serde(rename = "displayValue")]
    pub display_value: Option<Value>,
    pub value: Option<Value>,
}

/*
[
    {
        "toTop":true,
        "cells": [
            {
                "columnId": 7960873114331012,
                "value": true
            },
            {
                "columnId": 642523719853956,
                "value": "New status",
                "strict": false
            }
        ]

    },
    {
        "toTop":true,
        "cells": [
            {
                "columnId": 7960873114331012,
                "value": true
            },
            {
                "columnId": 642523719853956,
                "value": "New status",
                "strict": false
            }
        ]
    }
]
*/
