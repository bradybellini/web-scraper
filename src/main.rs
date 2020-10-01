use std::time::Duration;
use reqwest::Url;


fn build_client() -> &reqwest::blocking::Client() {
    
}

fn fetch_page(url: &str, origin_url: &str) {
    let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(10))
    .build();
    
}

fn main() {
    


    let resp = client.get("https://en.wikipedia.org/wiki/Heroes_of_the_Storm");
    
    println!("cool");
}
