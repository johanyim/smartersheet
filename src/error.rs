use thiserror::Error;

use crate::models::Row;

#[derive(Debug, Error)]
pub enum SmarterSheetError {
    #[error("No Sheets found")]
    NoSheets,
    #[error(transparent)]
    RequestFailed(#[from] reqwest::Error),
    #[error("Could not convert the row index {0} to into a struct")]
    RowToStructConversion(usize),
    #[error("The column titled {0} could not be found")]
    CouldNotFindColumn(String),
    #[error("There are multiple columns titled {0}")]
    AmbiguousColumnTitle(String),
}
