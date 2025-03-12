use crate::{models::response::SheetsResponse, prelude::*};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, OnceCell};

use futures::TryFutureExt;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use tokio::runtime::Runtime;
use tracing::warn;

use crate::error::SmarterSheetError;

use super::models::{Column, Sheet};

#[derive(Debug)]
pub struct Client<'a> {
    api_client: Arc<Mutex<reqwest::Client>>,
    access_token: &'a str,
    sheets: OnceCell<HashMap<String, u64>>,
}

impl<'a: 'b, 'b> Client<'a> {
    pub async fn from_api_key(api_key: &'a str) -> Client<'a> {
        Client {
            api_client: Arc::new(Mutex::new(reqwest::Client::new())),
            access_token: api_key,
            sheets: OnceCell::new(),
        }
    }
    pub async fn sheets(&self) -> HashMap<String, u64> {
        let ret = self.sheets.get_or_init(|| async {
            let client = self.api_client.lock().await;

            let res: Value = client
                .get(&format!("https://api.smartsheet.com/2.0/sheets"))
                .header(AUTHORIZATION, &format!("Bearer {}", self.access_token))
                .header(CONTENT_TYPE, "application/json")
                .send()
                .and_then(|r| r.json())
                .await
                .unwrap();

            let res = res
                .get("data")
                .and_then(|d| d.as_array())
                .unwrap()
                .into_iter()
                .map(|obj| {
                    (
                        obj.get("name")
                            .clone()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        obj.get("id").clone().unwrap().as_u64().clone().unwrap(),
                    )
                })
                .collect::<HashMap<String, u64>>();

            res
        });

        ret.await.clone()
    }

    pub async fn sheet(&'a self, sheet_name: &str) -> Option<Sheet<'a, 'b>> {
        let sheet_url = format!(
            "https://api.smartsheet.com/2.0/sheets/{}",
            self.sheets().await.get(sheet_name)?
        );

        let client = self.api_client.lock().await;
        let res: SheetsResponse = client
            .get(&sheet_url)
            .header(AUTHORIZATION, &format!("Bearer {}", self.access_token))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .ok()?
            .json()
            .await
            .unwrap();

        // println!("{:#?}", res);

        Some(Sheet {
            client: &self,
            columns: res.columns,
            rows: res.rows,
        })
    }
}
