use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_yaml::Value;
use tera::{Context, Tera};
use wp2hugo::{date_format_filter, Post};

fn main() -> std::io::Result<()> {
    let posts_str = fs::read_to_string(&Path::new("temp/all_posts.json"))?;
    let post_vec: Vec<Post> = serde_json::from_str(&posts_str)?;

    let posts: HashMap<&str, &Post> = post_vec.iter()
        .map(|post| ( post.name.as_str() , post)).collect();

    let path = Path::new("temp/hugo-export/posts"); // specify the folder path here

    let mut processed = Vec::new();
    let mut unprocessed = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.display().to_string();
            if !filename.ends_with("-.md") {
                let md_file_str = fs::read_to_string(&filename)?;
                let splits: Vec<&str> = md_file_str.splitn(3, "---").collect();
                let front_matter_str = splits[1];
                let post_content = splits[2];

                println!("processing: {}", &filename);
                let front_matter: Value = serde_yaml::from_str(front_matter_str).unwrap();
                if let Some(url) = front_matter["url"].as_str() {
                    let url = url.trim_start_matches('/').trim_end_matches('/');

                    if let Some(post) = posts.get(url) {
                        generate_md_file(post, post_content)?;
                        processed.push(filename);
                    } else {
                        println!("Post not found with url: {url}, {filename}");
                        unprocessed.push(format!("{}, url: {}", filename, url));
                    }
                } else {
                    unprocessed.push(filename);
                }
            }
        }
    }

    println!("processed: {}, unprocessed: {}", processed.len(), unprocessed.len());

    fs::write("temp/processed.txt", processed.join("\n"));
    fs::write("temp/unprocessed.txt", unprocessed.join("\n"));

    Ok(())
}

fn generate_md_file(post: &Post, post_content: &str) -> std::io::Result<()> {
    let mut tera = Tera::new("templates/**/*").expect("Failed to parse templates");

    tera.register_filter("date_format", date_format_filter);

    // Create context and add data
    let mut context = Context::new();
    context.insert("post", post);
    context.insert("post_content", post_content);

    let html = tera.render("post-template.md", &context).expect("Failed to render");

    let folder_name = format!("temp/generated/{}", post.name.as_str());

    if fs::exists(folder_name.as_str())? {
        fs::remove_dir_all(folder_name.as_str())?;
    }

    fs::create_dir(folder_name.as_str())?;
    fs::write(format!("{}/index.md", folder_name), html)?;

    Ok(())
}