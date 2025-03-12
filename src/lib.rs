use prelude::*;
pub mod api;
pub mod error;
pub mod models;
pub mod prelude;

const API: &str = "https://api.smartsheet.com/2.0";

const SHEET_ID: u64 = 7091741517631364;
const SHEET_URL: &str =
    "https://app.smartsheet.com/sheets/wj7QmH2C5rGX9mfCRPFpgxf9mVJhHVV4FRRFcFH1";
const ACCESS_TOKEN: &str = "h7pMJ8sdXVQApXlI5vZ17PuDqAYTPCHR2iZXY";

#[cfg(test)]
mod tests {
    use api::Client;
    use chrono::NaiveDate;
    use models::Sheet;
    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn can_get_sheet() {
        let client = Client::from_api_key(ACCESS_TOKEN).await;
        let _sheet = client.sheet("Test Sheet").await.unwrap();
    }

    #[tokio::test]
    async fn one_client_many_sheets_open() {
        let client = Client::from_api_key(ACCESS_TOKEN).await;
        let _sheet = client.sheet("Test Sheet").await.unwrap();
        let _sheet2 = client.sheet("Test Sheet").await.unwrap();
    }

    #[derive(Debug, Default)]
    struct MyStruct {
        primary_column: String,
        column2: String,
        column3: String,
        column4: Option<String>,
        column5: Option<f64>,
        column6: i64,
        date: NaiveDate,
    }

    impl MyStruct {
        fn try_from_smartsheet(row: crate::models::Row) -> Result<Self> {
            let values: [Option<serde_json::Value>; 7] = row
                .cells
                .into_iter()
                .map(|r| r.value)
                .take(7)
                .collect::<Vec<Option<serde_json::Value>>>()
                .try_into()
                .map_err(|_| SmarterSheetError::RowToStructConversion(0))?;

            // Value::Null => todo!(),
            // Value::Bool(_) => todo!(),
            // Value::Number(number) => todo!(),
            // Value::String(_) => todo!(),
            // Value::Array(vec) => todo!(),
            // Value::Object(map) => todo!(),
            println!("Value is : {:#?}", &values[0]);
            let primary_column = match &values[0] {
                Some(Value::Number(n)) => n.to_string(),
                Some(Value::String(s)) => s.to_string(),
                _ => return Err(SmarterSheetError::RowToStructConversion(0)),
            };

            let column2 = match &values[1] {
                Some(Value::Number(n)) => n.to_string(),
                Some(Value::String(s)) => s.to_string(),
                _ => return Err(SmarterSheetError::RowToStructConversion(1)),
            };

            let column3 = match &values[2] {
                Some(Value::Number(n)) => n.to_string(),
                Some(Value::String(s)) => s.to_string(),
                _ => return Err(SmarterSheetError::RowToStructConversion(2)),
            };

            let column4 = match &values[3] {
                Some(Value::Null) => None,
                Some(Value::Number(n)) => Some(n.to_string()),
                Some(Value::String(s)) => Some(s.to_string()),
                None => None,
                _ => return Err(SmarterSheetError::RowToStructConversion(3)),
            };

            let column5 = match &values[4] {
                Some(Value::Null) => None,
                Some(Value::Number(n)) => n.as_f64(),
                Some(Value::String(s)) => Some(
                    s.parse::<f64>()
                        .map_err(|_| SmarterSheetError::RowToStructConversion(4))?,
                ),
                None => None,
                _ => return Err(SmarterSheetError::RowToStructConversion(4)),
            };

            let column6 = match &values[5] {
                Some(Value::Number(n)) => n
                    .as_f64()
                    .ok_or(SmarterSheetError::RowToStructConversion(5))?
                    .round() as i64,
                Some(Value::String(s)) => s
                    .parse::<i64>()
                    .map_err(|_| SmarterSheetError::RowToStructConversion(5))?,
                _ => return Err(SmarterSheetError::RowToStructConversion(5)),
            };

            // let date = values[6].clone().unwrap();

            let date = match &values[6] {
                Some(Value::String(s)) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .map_err(|_| SmarterSheetError::RowToStructConversion(6))?,
                _ => return Err(SmarterSheetError::RowToStructConversion(6)),
            };

            return Ok(Self {
                primary_column,
                column2,
                column3,
                column4,
                column5,
                column6,
                date,
            });
        }
    }

    #[tokio::test]
    async fn deserialize_cells() {
        let client = Client::from_api_key(ACCESS_TOKEN).await;
        let sheet: Sheet = client.sheet("Test Sheet").await.unwrap();

        let cells = sheet
            .rows
            .into_iter()
            .map(MyStruct::try_from_smartsheet)
            .take_while(|e| e.is_ok())
            .collect::<Result<Vec<MyStruct>>>();

        println!("cells: {cells:#?}");
    }

    #[tokio::test]
    async fn update_cells() {
        let client = Client::from_api_key(ACCESS_TOKEN).await;
        let sheet: Sheet = client.sheet("Test Sheet").await.unwrap();

        // let res = sheet.update()
        // println!("result: {res:#?}");
    }
}
