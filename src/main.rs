use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use url::Url;

// Function to do Scrapping of a known website
fn scrap_web(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send()?.text()?;

    // Parse the HTML document
    let document = Document::from(response.as_str());

    // Get all the links in the document
    for node in document.find(Name("a")) {
        if let Some(link) = node.attr("href") {
            let url = Url::parse(link);
            match url {
                Ok(url) => {
                    println!("Link: {}", url);
                }
                Err(_) => {
                    println!("Invalid URL: {}", link);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let url = "https://www.github.com/";

    if let Err(e) = scrap_web(url) {
        eprintln!("Error: {}", e);
    }
    else {
        println!("Scraping done!");
    }

}
