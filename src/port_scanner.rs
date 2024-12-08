/*
 * This module contains the functions used to scan ports to a URL
*/

use tokio::{net::TcpStream, runtime::Runtime, sync::mpsc::{self}};
use std::{net::IpAddr, task};

pub fn port_scan(ip: IpAddr, first_port: u16, last_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;

    let (tx, mut rx) = mpsc::channel(10);

    rt.block_on(async {
        for port in first_port..=last_port {
            println!("Trying with port: {port}");
            let tx = tx.clone();
            let task = tokio::spawn(async move {
                if let Err(err) = scan(ip, port, tx).await {
                    eprintln!("error: {}", err);
                }
            });

            task.await.unwrap();
        }
    });

    while let Ok((ip, port)) = rx.try_recv() {
        println!("Founded {} port as open to {} address", port, ip);
    }

    
    Ok(())
}

async fn scan (ip: IpAddr, trying_port: u16, results_tx: mpsc::Sender<(IpAddr, u16)>) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    if let Ok(_ping) = TcpStream::connect((ip, trying_port)).await {
        results_tx.send((ip, trying_port)).await?;
    }

    Ok(())
}