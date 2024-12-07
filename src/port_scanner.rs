/*
 * This module contains the functions used to scan ports to a URL
*/

use tokio::{net::TcpStream, runtime::Runtime};
use std::net::IpAddr;

pub fn port_scan(ip: IpAddr, first_port: u16, last_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        for port in first_port..=last_port {
            tokio::spawn(async move {
                let stream_attempt = TcpStream::connect((ip, port)).await;
                if let Ok(_open) = stream_attempt {
                    println!("port {} is open", port);
                };
            });
        }
    });

    
    Ok(())
}