extern crate reqwest;
extern crate url;

mod webdriver;
mod config;

use url::Url;

fn main() -> Result<(), Box<std::error::Error>> {
    let config = config::load_config();
    let client = reqwest::Client::new();
    let mut context = webdriver::WebDriverContext { 
        client: client,
        config: config,
        session_id: None
    };
    
    // End-to-end demo!
    webdriver::create_session(&mut context);
    webdriver::navigate(&context, Url::parse("http://www.recurse.com")?);
    let html_element =
        webdriver::get_element(
            &context,
            webdriver::WebDriverLocatorStrategy::TagName,
            "html"
        );
    println!("{}", webdriver::get_text(&context, html_element));
    webdriver::delete_session(&context);

    Ok(())
}
