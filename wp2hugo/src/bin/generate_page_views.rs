#![allow(warnings)]

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use wp2hugo::Post;

fn main() -> Result<()> {
    // The file is read from "temp/all_posts_from_mysql.json"
    let posts_str = fs::read_to_string(&Path::new("temp/all_posts_from_mysql.json"))?;
    let post_vec: Vec<Post> = serde_json::from_str(&posts_str)?;

    let page_view_file = File::create("../data/wpPageViews.toml")?;
    let mut writer = BufWriter::new(page_view_file);
    writeln!(writer, "# Page view in WordPress, wp_post_id -> page_view");
    writeln!(writer, "[page_views]")?;

    for post in post_vec {
        writeln!(writer, "{} = {}", post.id, post.views.unwrap_or(0))?;
    }

    Ok(())
}
