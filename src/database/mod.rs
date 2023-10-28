// mod model;
//
// use anyhow::{anyhow, Result};
// // use sea_orm::{Database, DatabaseConnection};
//
// #[derive(Debug)]
// pub struct Db {
//     db_connection: DatabaseConnection,
// }
//
// impl Db {
//     pub async fn from(db_path: &str) -> Result<Self> {
//         let tmp_db_conn = Database::connect(std::format!("sqlite:{}?mode=rwc", db_path)).await;
//         if let Ok(tmp_db_conn) = tmp_db_conn {
//             Ok(Db {
//                 db_connection: tmp_db_conn,
//             })
//         } else {
//             Err(anyhow!(tmp_db_conn.unwrap_err()))
//         }
//     }
//
//     // pub fn init(&self) -> Result<()> {}
// }
