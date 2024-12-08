/*
 * This module contains the functions used to scan ports to a URL
*/

use tokio::{net::TcpStream, runtime::Runtime, sync::mpsc::{self}, sync::Semaphore};
use std::{net::IpAddr, sync::Arc, task};
use tokio::time::{timeout, Duration};

pub fn port_scan(ip: IpAddr, first_port: u16, last_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;

    let mut tasks = vec![];
    let semaphore = Arc::new(Semaphore::new(500));

    let (tx, mut rx) = mpsc::channel(500);

    rt.block_on(async {
        for port in first_port..=last_port {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let tx = tx.clone();

            println!("Trying with port {port}");

            let task = tokio::spawn(async move {
                if let Err(err) = scan(ip, port, tx).await {
                    eprintln!("error: {}", err);
                }
                drop(permit);
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }
    });

    while let Ok((ip, port)) = rx.try_recv() {
        println!("Founded {} port as open to {} address", port, ip);
    }

    
    Ok(())
}

async fn scan (ip: IpAddr, trying_port: u16, results_tx: mpsc::Sender<(IpAddr, u16)>) -> Result<(), mpsc::error::SendError<(IpAddr, u16)>> {
    if let Ok(_ping) = timeout(Duration::from_secs(1), TcpStream::connect((ip, trying_port))).await {
        results_tx.send((ip, trying_port)).await?;
    }

    Ok(())
}