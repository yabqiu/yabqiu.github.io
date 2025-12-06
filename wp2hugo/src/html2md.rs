use std::fs;
use htmd::element_handler::Handlers;
use htmd::{Element, HtmlToMarkdown};

pub fn html2md(html_content: String) -> String {
    todo!()
}

#[test]
fn test_html2md() {
    let html_content = fs::read_to_string("example.html").unwrap();
    let convert = HtmlToMarkdown::builder()
        .scripting_enabled(false)
        .add_handler(vec!["pre"], |_handlers: &dyn Handlers, element: Element| {
            // Skip the img tag when converting.
            println!("{}", element.tag);
            None
        })
        .build();
    let md_content = convert.convert(html_content.as_str()).unwrap();
    fs::write("example.md", md_content);
    // println!("{}", htmd::convert(html_content.as_str()).unwrap());
}

#[test]
fn test_pre_markup() {
    let html_content = r#"
        <pre class="lang:java decode:true>
        public class Test {
            public static void main(String[] args) {
                System.out.println("Hello World!");
            }
        }
    "#;
    let convert = HtmlToMarkdown::builder()
        .scripting_enabled(false)
        // .add_handler(vec!["pre"], |_handlers: &dyn Handlers, element: Element| {
        //     // Skip the img tag when converting.
        //     println!("{}", element.tag);
        //     None
        // })
        .build();
    let md_content = convert.convert(html_content).unwrap();

    // fs::write("test.md", md_content);
    println!("{}", md_content);
}