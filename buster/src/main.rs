use std::fs::File;
use std::io::{self, BufRead, BufReader};
use reqwest;
use tokio;
use std::env;
use std::sync::{Arc, Mutex};

fn get_args() -> Result<(String, String, usize), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not enough arguments"));
    }
    let ip = args[1].clone();
    let wordlist = args[2].clone();

    let workers = args[3].clone().parse::<usize>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "The third argument must be an integer"))?;

    Ok((ip, wordlist, workers))
}


// fn read_wordlist() {
//     return
// }
#[tokio::main]
async fn main() -> io::Result<()> {
    let (ip, wordlist, workers) = get_args()?;

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

    let semaphore = Arc::new(tokio::sync::Semaphore::new(workers));
    let mut handles = Vec::new();
    let urls_tested = Arc::new(Mutex::new(0));

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