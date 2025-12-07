#![allow(warnings)]

use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::Path;
use tera::{Context, Tera};
use wp2hugo::{Post, content_transform_filter, date_format_filter};

fn main() -> Result<()> {
    // The file is read from "temp/all_posts_from_mysql.json"
    let posts_str = fs::read_to_string(&Path::new("temp/all_posts_from_mysql.json"))?;
    let post_vec: Vec<Post> = serde_json::from_str(&posts_str)?;

    let processed_ids: Vec<u32> = get_processed_ids()?;
    println!("processed ids: {}", processed_ids.len());

    // generate_from_wordpress_hugo_exports(&post_vec);
    // generate_from_mysql_exports(&processed_ids, &post_vec);

    Ok(())
}

fn get_processed_ids() -> Result<Vec<u32>> {
    let mut processed_id : Vec<u32>= Vec::new();
    for entry in fs::read_dir("../content/post")? {
        let path = entry?.path();
        if path.is_dir() {
            let page_file_path = path.join("page.html");
            if fs::exists(&page_file_path)? {
                let page_file_str = fs::read_to_string(&page_file_path)?;
                let splits: Vec<&str> = page_file_str.splitn(3, "---").collect();
                let front_matter: Value = serde_yaml::from_str(splits[1]).unwrap();
                if let Some(wp_post_id) = front_matter.get("wpPostId") {
                     processed_id.push(wp_post_id.as_u64().unwrap() as u32)
                }
            }
        }
    }
    Ok(processed_id)
}

fn generate_from_mysql_exports(processed_ids: &Vec<u32>, post_vec: &Vec<Post>) -> Result<()> {
    let mut count = 0;
    for post in post_vec {
        if post.status == "publish" && !processed_ids.contains(&post.id) {
            let content = clean_post_content(post.content.as_str())?;
            count += 1;
            println!("#{count}, processing: {}, id: {}", post.name, post.id);
            generate_md_file(post, content.as_str(), "page.html")?;
        }
    }

    Ok(())
}

fn clean_post_content(post_content: &str) -> Result<String> {
    let mut content = post_content
        .replace("\n\n", "<br/>\r\n")
        .replace("\r\n", "<br/>\r\n");

    // replace </xx><br/> with </xx>\r\n
    for el in vec![
        "table",
        "tbody",
        "tr",
        "td",
        "div",
        "pre",
        "p",
        "blockquote",
        "li",
        "ol",
    ] {
        let re = regex::Regex::new(format!(r"(</{}>)<br/>", el).as_str()).unwrap();
        content = re.replace_all(content.as_str(), "$1\r\n").to_string();
    }

    // replace <xx something><br/> with <xx something>\r\n
    for el in vec![
        "table", "tbody", "tr", "td", "div", "ol",
    ] {
        let re = regex::Regex::new(format!(r"(<{}[^>]*>)<br/>", el).as_str()).unwrap();
        content = re.replace_all(content.as_str(), "$1\r\n").to_string();
    }

    Ok(content)
}

fn generate_from_wordpress_hugo_exports(post_vec: &Vec<Post>) -> Result<()> {
    let path = Path::new("temp/hugo-export/posts"); // specify the folder path here

    let posts: HashMap<&str, &Post> = post_vec
        .iter()
        .map(|post| (post.name.as_str(), post))
        .collect();

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
                        generate_md_file(post, post_content, "index.md")?;
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

    println!(
        "processed: {}, unprocessed: {}",
        processed.len(),
        unprocessed.len()
    );

    fs::write("temp/processed.txt", processed.join("\n"))?;
    fs::write("temp/unprocessed.txt", unprocessed.join("\n"))?;

    Ok(())
}

fn generate_md_file(post: &Post, post_content: &str, target_file: &str) -> Result<()> {
    let mut tera = Tera::new("templates/**/*").expect("Failed to parse templates");

    tera.register_filter("date_format", date_format_filter);
    tera.register_filter("transform", content_transform_filter);

    // Create context and add data
    let mut context = Context::new();
    context.insert("post", post);
    context.insert("post_content", post_content);

    let html = tera
        .render("post-template.md", &context)
        .expect("Failed to render");

    let folder_name = format!("temp/generated/{}", post.name.as_str());

    if fs::exists(folder_name.as_str())? {
        fs::remove_dir_all(folder_name.as_str())?;
    }

    fs::create_dir_all(folder_name.as_str())?;
    fs::write(format!("{}/{}", folder_name, target_file), html)?;

    Ok(())
}
