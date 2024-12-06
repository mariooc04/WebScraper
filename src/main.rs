mod scrapping;
use scrapping::scrap_web;
use scrapping::search_dir_files;


#[tokio::main]
async fn main() {
    let result = tokio::spawn(async {
        let url = "https://www.github.com/";

        match scrap_web(url).await {
            Ok(data) => println!("Scrapping result: {:?}", data),
            Err(err) => eprintln!("Error during scraping: {}", err),
        }

        // Otras operaciones asincrónicas
    })
    .await;

    if let Err(e) = result {
        eprintln!("Error in spawned task: {:?}", e);
    }



    // Test of search_dir_files function (future test with a dictionary)
    /* let dirs = vec!["src", "main", "test"];
    let files = vec!["index.html", "main.rs", "test.rs"];
    if let Err(e) = search_dir_files(url, dirs, files) {
        eprintln!("Error: {}", e);
    }
    else {
        println!("Searching done!");
    } */

}
