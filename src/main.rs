// use std::fs;
//
// use futures::future::join_all;
// use models::{AddRowRequest, Cell, SmartSheet};
// use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
// use serde_json::Value;
// use tracing::*;
//
// mod api;
// use api::*;
// mod models;
//
// const API: &str = "https://api.smartsheet.com/2.0";
//
// const SHEET_ID: u64 = 7091741517631364;
// const SHEET_URL: &str =
//     "https://app.smartsheet.com/sheets/wj7QmH2C5rGX9mfCRPFpgxf9mVJhHVV4FRRFcFH1";
// const ACCESS_TOKEN: &str = "h7pMJ8sdXVQApXlI5vZ17PuDqAYTPCHR2iZXY";
//
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let filter = tracing_subscriber::filter::EnvFilter::from_default_env()
//         .add_directive(format!("{}=debug", env!("CARGO_CRATE_NAME")).parse()?);
//     tracing_subscriber::fmt().with_env_filter(filter).init();
//
//     let time_start = std::time::Instant::now();
//     info!("Starting {}", env!("CARGO_CRATE_NAME"));
//
//     let mut client = reqwest::Client::new();
//     // let sheets = api::list_sheets(&mut client).await?;
//
//     // let futs = sheets
//     //     .into_iter()
//     //     .map(|(id, name)| async move {
//     //         let mut client = reqwest::Client::new();
//     //         info!("Downloading {name}");
//     //         let bytes = api::get_sheet_as_excel(&mut client, id)
//     //             .await
//     //             .expect("Getting Excel from API");
//     //         let path = format!("./responses/{name}.xlsx");
//     //         let _ = tokio::fs::write(path, bytes)
//     //             .await
//     //             .expect("Writing to disk");
//     //     })
//     //     .collect::<Vec<_>>();
//     //
//     // join_all(futs).await;
//
//     let sheet = api::get_sheet(&mut client, SHEET_ID).await?;
//     // debug!("{sheet:#?}");
//
//     let cells = sheet
//         .columns
//         .into_iter()
//         .map(|col| Cell {
//             column_id: col.id,
//             value: Some(Value::from(1234)),
//             formula: None,
//         })
//         .collect::<Vec<_>>();
//
//     let addrow = AddRowRequest {
//         to_top: true,
//         cells,
//     };
//
//     let res = client
//         .post(&format!("{API}/sheets/{SHEET_ID}/rows"))
//         .header(AUTHORIZATION, &format!("Bearer {ACCESS_TOKEN}"))
//         .header(CONTENT_TYPE, "application/json")
//         .json(&vec![
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//             &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow, &addrow,
//         ])
//         .send()
//         .await?
//         .text()
//         .await?;
//
//     debug!("res: {res:?}");
//
//     info!("Completed in {:?}s", (time_start.elapsed().as_secs()));
//
//     Ok(())
// }
