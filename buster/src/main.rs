use std::fs::File;
use std::io::{self, BufRead, BufReader};
use reqwest;
use tokio;
use std::env;
use std::sync::{Arc, Mutex};
// fn build_url (domain: String, directory: String) -> String {
//     //Check for https/http and add if needed
//     let url = format!("{}{}", domain, directory);
//     url
// }

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ip = String::from(args[1].clone());
    let wordlist = String::from(args[2].clone());
    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);
    let mut lines = vec![];
    let semaphore = Arc::new(tokio::sync::Semaphore::new(1));
    let mut handles = Vec::new();
    let urls_tested = Arc::new(Mutex::new(0));

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.chars().next() != Some('#') {
                    lines.push(line.clone());
                }
            },
            Err(e) => println!("Error with line {}", e)
        }
    }

    for line in &lines {
        let full_url = format!("{}{}", ip, line);
        let sem_clone = semaphore.clone();
        let urls_total = Arc::clone(&urls_tested);

        let handle = tokio::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();

            let response = reqwest::get(&full_url).await;
            match response {
                Ok(r) => {
                        let mut num = urls_total.lock().unwrap();
                        *num += 1;
                        if r.status().is_success(){

                        println!("Url: {} Status: {}, urls_tested: {}", full_url, r.status(), *num);
                    }
                }
                Err(e) => {
                    println!("Error with request {}", e);
                }
            }

        });

        handles.push(handle)
    }

    for handle in handles {
        handle.await?
    }

    Ok(())
}