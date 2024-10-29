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

// Function to search directories or files in a website
// You have to pass the directories and files to search for (maybe from a dictionary)
fn search_dir_files(url: &str, dirs: Vec<&str>, files: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    for dir in dirs {
        let url = format!("{}/{}", url, dir);
        let response = client.get(&url).send();

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Directory found: {}", url);
                }
            }
            Err(_) => {
                println!("Directory not found: {}", url);
            }
        }
    }

    for file in files {
        let url = format!("{}/{}", url, file);
        let response = client.get(&url).send();

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    println!("File found: {}", url);
                }
            }
            Err(_) => {
                println!("File not found: {}", url);
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

    // Test of search_dir_files function (future test with a dictionary)
    let dirs = vec!["src", "main", "test"];
    let files = vec!["index.html", "main.rs", "test.rs"];
    if let Err(e) = search_dir_files(url, dirs, files) {
        eprintln!("Error: {}", e);
    }
    else {
        println!("Searching done!");
    }

}
