use std::fs::OpenOptions;
use std::io::Write;
use crate::features::Page;
use crate::helpers::helpers::{get_project_name_from_rustyroad_toml};
use futures_util::TryFutureExt;
use tera::Tera;
pub fn render_index_page() -> Result<String, tera::Error> {
    // ensure this is a rustyroad project
    get_project_name_from_rustyroad_toml().unwrap_or_else(|err| {
        println!("This is not a rustyroad project.");
        panic!("Error: {}", err);
    });

    let tera = match Tera::new("src/views/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut templates = Vec::new();

    tera.get_template_names().into_iter().for_each(|name| {
        templates.push(name.to_string());
    });
    let context = tera::Context::new();
    let rendered = tera.render("pages/index.html.tera", &context)?;


    Ok(rendered)
}


// store the index page in a page struct and save it to the database
pub async fn save_index_page() {
    // ensure this is a rustyroad project
    get_project_name_from_rustyroad_toml().unwrap_or_else(|err| {
        println!("This is not a rustyroad project.");
        panic!("Error: {}", err);
    });

    let html = render_index_page().unwrap_or_else(|err| {
        println!("Error rendering index page: {}", err);
        panic!("Error: {}", err);
    });

    let mut page = Page::new();

    page.html_content = html;
    page.title = "index".to_string();
    page.slug = Some("index".to_string());
    page.associated_user_id = 1;
    page.page_status = Some("published".to_string());
    page.seo_no_index = Some(false);
    page.seo_no_follow = Some(false);
    page.schema_type = Some("WebPage".to_string());
    page.schema_page_type = Some("WebPage".to_string());
    page.schema_article_type = Some("WebPage".to_string());
    page.schema_description = Some("".to_string());
    page.schema_author = Some("".to_string());
    page.schema_publisher = Some("".to_string());
    page.schema_image = Some("".to_string());
    page.schema_url = Some("".to_string());
    page.schema_name = Some("".to_string());
    page.schema_headline = Some("".to_string());
    page.schema_date_published = Some(chrono::Utc::now().naive_utc());
    page.schema_date_modified = Some(chrono::Utc::now().naive_utc());
    page.seo_keywords = Some("".to_string());
    page.seo_focus_keyphrase = Some("".to_string());
    page.seo_canonical_url = Some("".to_string());
    page.seo_og_title = Some("".to_string());
    page.seo_og_locale = Some("".to_string());
    page.seo_og_type = Some("".to_string());
    page.seo_og_description = Some("".to_string());
    page.seo_og_image = Some("".to_string());
    page.seo_og_image_width = Some(0);
    page.seo_og_image_height = Some(0);
    page.seo_twitter_title = Some("".to_string());
    page.seo_twitter_description = Some("".to_string());
    page.seo_twitter_image = Some("".to_string());
    page.seo_twitter_image_alt = Some("".to_string());
    page.seo_twitter_card = Some("".to_string());
    page.featured_image = Some("".to_string());
    page.featured_image_thumbnail = Some("".to_string());
    page.seo_title = Some("".to_string());
    page.seo_description = Some("".to_string());
    page.author = Some("".to_string());
    page.author_image = Some("".to_string());
    page.author_thumbnail = Some("".to_string());
    page.author_url = Some("".to_string());
    page.excerpt = Some("".to_string());
    page.summary = Some("".to_string());
    page.is_secure = false;
    page.is_published = true;

    Page::create_page(page)
        .unwrap_or_else(|err| {
            println!("Error saving index page: {}", err);
            panic!("Error: {}", err);
        })
        .await;
}




/// # Name: update_index_controller
/// # Description: Parses the index controller and replaces the template with the html in the database
/// # Arguments:
/// ## * `index_controller` - The index controller -```String```
/// ## * `html` - The html to be inserted into the index controller - ```String```
pub async fn update_index_controller() -> Result<String, tera::Error> {
    // ensure this is a rustyroad project
    get_project_name_from_rustyroad_toml().unwrap_or_else(|err| {
        println!("This is not a rustyroad project.");
        panic!("Error: {}", err);
    });

    //

 // read the index controller
    let mut index_controller = std::fs::read_to_string("./src/controllers/index.rs")
        .unwrap_or_else(|err| {
            println!("Error reading index controller: {}", err);
            panic!("Error: {}", err);
        });


   save_index_page().await;

    let new_index_code = format!(r#"
      let result = Page::get_page_by_slug("index".to_string()).await;
      match result {{
            Ok(page) => {{
                context.insert("title", "Create Page");
                context.insert("route_name", "create_page");
                context.insert("html_content", &page.html_content);
                context.insert("page_id", &page.id);
                let s = tmpl.render("pages/page.html.tera", &context).unwrap();
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s)
            }}
            Err(e) => {{
                context.insert("error", &e.to_string());
                let s = tmpl.render("pages/404.html.tera", &context).unwrap();
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s)
            }}
        }}
    "#);

    index_controller = index_controller.replace(
        "let rendered = tmpl.render(\"pages/index.html.tera\", &context).unwrap();
    HttpResponse::Ok().body(rendered)",
        &new_index_code,
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./src/controllers/index.rs")
        .unwrap();
    // Write the updated contents to the file
    writeln!(file, "{}", index_controller).unwrap();

    file.flush()?;

    Ok("".to_string())
}


mod tests {

    #[test]
    fn test_render_index_page() {
        use color_eyre::owo_colors::OwoColorize;
        
        // change directory to example-grapesjs
        std::env::set_current_dir("../test42").unwrap();
        // print the working directory
        println!("Current directory: {:?}", std::env::current_dir());

        let result = crate::features::render_index_page();

        match result {
            Ok(rendered) => println!("Rendered: {}", rendered.bright_cyan()),
            Err(e) => eprintln!("Error: {}", e),
        }
    }


    #[tokio::test]
    async fn test_save_index_page() {
        // change directory to example-grapesjs
        std::env::set_current_dir("../test42").unwrap();
        // print the working directory
        println!("Current directory: {:?}", std::env::current_dir());

        crate::features::save_index_page().await;

        let page = crate::features::Page::get_page_by_slug("index".to_string()).await.unwrap();

        println!("Page: {:?}", page);
    }

    #[tokio::test]
    async fn test_update_index_controller() {
        // change directory to example-grapesjs
        std::env::set_current_dir("../test42").unwrap();
        // print the working directory
        println!("Current directory: {:?}", std::env::current_dir());

        crate::features::update_index_controller().await.expect("TODO: panic message");

        println!("Updated index controller")
    }
}
