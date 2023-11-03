use std::{env, thread, time};

use reqwest::{StatusCode, Url};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("there are should be two args: interval and url");
        return;
    }
    let interval = &args[1].parse::<u64>();
    if interval.is_err() {
        println!("Interval parsing error");
        return;
    }
    let interval = interval.clone().unwrap();

    let url = &args[2];
    let url = Url::parse(url);
    if url.is_err() {
        println!("URL parsing error");
        return;
    }
    let url = url.unwrap();

    let delay = time::Duration::from_secs(interval);
    loop {
        let response = reqwest::blocking::get(url.clone());

        if let Ok(response) = response {
            let status = response.status();

            if status == StatusCode::OK {
                println!("Checking '{}'. Result: OK(200)", url);
            } else {
                println!("Checking '{}'. Result: Err({})", url, status.as_u16());
            }
        } else {
            let err = response.err();
            println!("Unknown error: {:?}", err);
        }

        thread::sleep(delay);
    }
}
