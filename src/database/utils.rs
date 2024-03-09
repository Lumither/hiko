use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde_json::{json, Value};
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::query::Query;
use sqlx::{MySql, TypeInfo};
use sqlx_core::column::Column;
use sqlx_core::row::Row;

use crate::database::utils::ParseError::UnexpectedType;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedType(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            UnexpectedType(type_name) => {
                write!(f, " Unexpected Type: {}", type_name)
            }
        }
    }
}

impl Error for ParseError {}

pub async fn query_as_json<'a>(
    handle: &sqlx_core::pool::Pool<MySql>,
    query: Query<'a, MySql, MySqlArguments>,
) -> Result<Vec<Result<Value, Box<dyn Error>>>, Box<dyn Error>> {
    match query.fetch_all(handle).await {
        Ok(rows) => Ok(rows
            .iter()
            .map(row_to_json)
            .collect::<Vec<Result<Value, Box<dyn Error>>>>()),
        Err(e) => Err(Box::new(e)),
    }
}

fn row_to_json<'a>(row: &'a MySqlRow) -> Result<Value, Box<dyn Error>> {
    let mut result: HashMap<&'a str, Value> = Default::default();
    for col in row.columns() {
        let ord = col.ordinal();
        let col_type_ame = col.type_info().name();
        let tmp_value = match col_type_ame {
            "VARCHAR" => json!(row.try_get::<String, _>(ord)?),
            "INT" => json!(row.try_get::<i64, _>(ord)?),
            "JSON" | "TEXT" | "LONGTEXT" => json!(row.try_get::<Value, _>(ord)?),
            type_name => {
                return Err(Box::new(UnexpectedType(type_name.to_string())));
            } // todo: add more type parse
        };
        result.insert(col.name(), tmp_value);
    }
    Ok(json!(result))
}

#[cfg(test)]
mod tests {}
