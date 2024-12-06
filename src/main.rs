mod scrapping;
use scrapping::scrap_web;
use scrapping::search_dir_files;


fn main() {
    let url = "https://github.com";

    let _ = scrapping::scrap_web(url);

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
