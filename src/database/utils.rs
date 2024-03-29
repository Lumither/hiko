use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::process::exit;

use serde_json::{json, Value};
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::query::Query;
use sqlx::{MySql, MySqlPool, TypeInfo};
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
        let col_type_name = col.type_info().name();
        let tmp_value = match col_type_name {
            "VARCHAR" | "TEXT" => json!(match row.try_get::<Value, _>(ord) {
                // `json` was not supported in mariadb and will be stored as TEXT, this seg is used to differentiate json and text
                Ok(value) => {
                    value
                }
                Err(_) => {
                    json!(row.try_get::<String, _>(ord)?)
                }
            }),
            "INT" => json!(row.try_get::<i64, _>(ord)?),
            "BOOLEAN" => json!(row.try_get::<bool, _>(ord)?),
            "JSON" | "LONGTEXT" => json!(row.try_get::<Value, _>(ord)?),
            "DATETIME" => json!(row.try_get::<NaiveDateTime, _>(ord)?),
            type_name => {
                return Err(Box::new(UnexpectedType(type_name.to_string())));
            } // todo: add more type parse
        };
        result.insert(col.name(), tmp_value);
    }
    Ok(json!(result))
}

pub async fn get_db_handler(
    url: &str,
    usr: &str,
    passwd: &str,
) -> Result<MySqlPool, Box<dyn Error>> {
    let conn =
        match MySqlPool::connect(format!("mysql://{}:{}@{}", usr, passwd, url).as_str()).await {
            Ok(conn) => conn,
            Err(e) => {
                log::error!("{}", e);
                exit(1)
            }
        };
    Ok(conn)
}

#[cfg(test)]
mod tests {}
