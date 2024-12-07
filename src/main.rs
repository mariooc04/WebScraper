use std::vec;
use clap::Parser;
use std::net::IpAddr;

mod scrapping;
mod port_scanner;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    url: String,
    addr: IpAddr,

    // For first and last port to scan
    #[arg(long, default_value_t = 1)]
    first_port: u16,
    #[arg(long, default_value_t = 1024)]
    last_port: u16,
}


fn main() {
    let args = Args::parse();

    // Making sure that port values are greater than 0
    assert!(args.first_port > 0);
    assert!(args.last_port >= args.first_port);
    
    let _ = scrapping::scrap_web(&args.url);

    let dirs = vec!["src", "main", "test"];
    let files = vec!["index.html", "main.rs", "test.rs"];

    let _ = scrapping::search_dir_files(&args.url, dirs, files);

    let _ = port_scanner::port_scan(args.addr, args.first_port, args.last_port);
}
