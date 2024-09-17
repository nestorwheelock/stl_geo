use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::error::Error;
use url::Url;

// Function to perform the scraping
fn scrape_page() -> Result<(), Box<dyn Error>> {
    // Define the base URL for the website
    let base_url = "https://dynamic.stlouis-mo.gov/citydata/newdesign/";

    // Create a cookie store to hold session cookies
    let cookie_store = Arc::new(Jar::default());

    // Create an HTTP client with cookie support
    let client = Client::builder()
        .cookie_provider(Arc::clone(&cookie_store))
        .build()?;

    // Step 1: Press the "Agree" button by submitting the form
    let agree_url = format!("{}index.cfm", base_url);
    let form_data = [("Agree", "Agree")];
    let response = client.post(&agree_url).form(&form_data).send()?;

    // Step 2: Print response headers to see if any "set-cookie" header is present
    if let Some(cookies) = response.headers().get_all("set-cookie").iter().next() {
        println!("Cookies after pressing Agree: {:?}", cookies);
    }

    // Step 3: Request the target page (now that we've accepted the disclaimer)
    let page_url = format!("{}addressparser.cfm", base_url);
    let response = client.get(&page_url).send()?;
    let content = response.text()?;

    // Step 4: Parse the HTML content and extract the data
    let document = Html::parse_document(&content);
    // Adjust the selector based on the actual HTML structure of the page
    let result_selector = Selector::parse("div.result").unwrap();  // Example selector

    // Iterate over the selected elements and extract the desired data
    for element in document.select(&result_selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("Extracted Data: {}", text);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Call the scrape function
    scrape_page()?;

    Ok(())
}

