use std::fs::File;
use std::io::{self, BufRead, BufReader};
use reqwest;
use tokio;
use std::env;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ip = String::from(args[1].clone());
    let wordlist = String::from(args[2].clone());
    let file = File::open(wordlist)?;
    let reader = BufReader::new(file);
    let mut lines = vec![];

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

    let client = reqwest::Client::new();

    for line in &lines {
        let full_url = format!("{}{}", ip, line);
        let response = client.get(&full_url).send().await;

        match response {
            Ok(r) => {
                if r.status().is_success(){
                    println!("Url: {} Status: {}", full_url, r.status());
                }
            }
            Err(e) => {
                println!("Error with request {}", e);
            }
        }
    }
    Ok(())
}