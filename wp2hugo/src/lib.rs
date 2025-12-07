#![allow(warnings)]

use std::collections::HashMap;
use chrono::{DateTime, NaiveDateTime, Utc};
use mysql::{OptsBuilder, Pool};
use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono_tz::America::Chicago;
use mysql::prelude::ToValue;
use tera;

#[derive(Debug, Serialize, Default, Clone, Deserialize)]
pub struct Post {
    pub id: u32,
    pub status: String,
    pub name: String,
    pub title: String,
    pub content: String,
    pub feature_image: Option<String>,
    pub post_date_gmt: Option<NaiveDateTime>,
    pub last_modified_gmt: Option<NaiveDateTime>,
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

pub fn date_format_filter(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let parse_format = "%Y-%m-%dT%H:%M:%S";
    let naive_dt = NaiveDateTime::parse_from_str(value.as_str().unwrap(), parse_format).unwrap();
    let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
    let chicago_dt = utc_dt.with_timezone(&Chicago);
    let output_format = "%Y-%m-%dT%H:%M:%S%:z";
    let formatted_string = chicago_dt.format(output_format).to_string();

    Ok(tera::to_value(formatted_string)?)
}

pub fn content_transform_filter(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let brStr = value.to_string().replace("\\r\\n", "<br/>\r\n");
    Ok(brStr.as_str().into())
}