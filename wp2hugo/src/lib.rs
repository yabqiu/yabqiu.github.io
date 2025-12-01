use chrono::{DateTime, Local, NaiveDateTime, Utc};
use mysql::{OptsBuilder, Pool};
use std::error::Error;
use std::fs;
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Post {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub content: String,
    pub feature_image: Option<String>,
    pub post_date_gmt: NaiveDateTime,
    pub last_modified_gmt: NaiveDateTime,
    pub views: Option<u32>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

pub fn get_db_conn_pool() -> Result<Pool, Box<dyn Error>> {
    let config_str = fs::read_to_string(".db_config.json")?;
    let pool_config = serde_json::from_str(config_str.as_str())?;
    let opts = OptsBuilder::new().from_hash_map(&pool_config)?;

    Ok(Pool::new(opts)?)
}

