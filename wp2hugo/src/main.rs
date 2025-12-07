mod html2md;

use chrono::NaiveDateTime;
use mysql::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use tera::{Context, Tera};
use wp2hugo::{date_format_filter, get_db_conn_pool, Post};
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // let args: Vec<String> = env::args().collect();
    // let post_id: Option<u32> = args.get(1).map(|arg| arg.parse().unwrap());
    //
    // // let posts = get_all_posts(post_id)?;
    // // let post = posts.get(&post_id.unwrap()).unwrap();
    //
    // println!("{}", htmd::convert(post.html_content.as_str()).unwrap());
    //
    // let mut tera = Tera::new("templates/**/*").expect("Failed to parse templates");
    //
    // tera.register_filter("date_format", date_format_filter);
    //
    // // Create context and add data
    // let mut context = Context::new();
    // context.insert("post", post);
    //
    //
    // let html = tera.render("post-template.md", &context).expect("Failed to render");
    // println!("{}", html);

    Ok(())
}
