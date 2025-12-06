use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use mysql::prelude::*;
use wp2hugo::{Post, get_db_conn_pool};

fn main() -> Result<(), Box<dyn Error>> {
    let all_posts = get_all_posts()?;
    fs::write("temp/all_posts_from_mysql.json", serde_json::to_string_pretty(&all_posts)?)?;
    Ok(())
}

fn get_all_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let db_conn_pool = get_db_conn_pool()?;
    let mut conn = db_conn_pool.get_conn()?;

    let all_post_query = r#"
     select a.id, a.post_status, nullif(a.post_date_gmt, '0000-00-00 00:00:00'),
       a.post_title, a.post_name, a.post_content, nullif(a.post_modified_gmt, '0000-00-00 00:00:00'),
       b.meta_value as views, d.guid as feature_image
             from wp_posts a
             left join wp_postmeta b on a.id=b.post_id and b.meta_key='views'
             left join wp_postmeta c on a.id=c.post_id and c.meta_key ='_thumbnail_id'
             left join wp_posts d on d.id=c.meta_value
       where a.post_type in ('post') and a.post_status in ('publish', 'draft', 'pending', 'private')
    "#;

    let rows: Vec<(
        u32,
        String,
        Option<NaiveDateTime>,
        String,
        String,
        String,
        Option<NaiveDateTime>,
        Option<String>,
        Option<String>,
    )> = conn.query(all_post_query)?;

    println!("found all required posts: {}", rows.len());

    let mut posts: HashMap<u32, Post> = HashMap::new();
    let mut post_ids: Vec<u32> = Vec::new();

    let mut count = 0;

    for (id, post_status, post_date_gmt, title, name,
        content, last_modified_gmt, views,
        feature_image) in rows {
        count += 1;
        println!("process post {count}");

        let mut post = Post::default();
        post.id = id;
        post.status = post_status;
        post.post_date_gmt = post_date_gmt;
        post.title = title;
        post.name = name;
        post.content = content;
        post.last_modified_gmt = last_modified_gmt;
        post.views = match views {
            None => None,
            Some(x) => x.parse().ok(),
        };
        post.feature_image = feature_image;

        post.categories = Vec::new();
        post.tags = Vec::new();

        post_ids.push(post.id);
        posts.insert(post.id, post);
    }


    let joined_post_ids = post_ids
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .join(",");

    let category_tags_query = format!(
        r#"
        select a.object_id, b.taxonomy, c.name from wp_term_relationships a
            left join wp_term_taxonomy b on a.term_taxonomy_id = b.term_taxonomy_id
            left join wp_terms c on b.term_id =c.term_id
            where b.taxonomy in ('category', 'post_tag') and object_id in ({})
        "#,
        joined_post_ids
    );

    // let params = Params::Positional(post_ids.iter().map(|id| Value::from(*id)).collect());
    let rows: Vec<(u32, String, String)> = conn.query(category_tags_query)?;

    for (id, taxonomy, name) in rows {
        let post: &mut Post = posts.get_mut(&id).unwrap();
        if taxonomy == "category" {
            post.categories.push(name);
        } else {
            post.tags.push(name);
        }
    }

    Ok(posts.values().cloned().collect::<Vec<Post>>())
}