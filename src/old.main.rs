// use reqwest::Url;
// use select::document::Document;
// use select::predicate::Name;
// use select::predicate::Predicate;
// use std::collections::HashSet;
// use std::fs;
// use std::io::Read;
// use std::path::Path;
// use std::time::Instant;
// use std::{thread, time};

// fn has_extension(url: &&str) -> bool {
//     Path::new(url).extension().is_none()
// }

// fn get_links_from_html(html: &String) -> HashSet<String> {
//     Document::from(html.as_str())
//         .find(Name("a").or(Name("link")))
//         .filter_map(|n| n.attr("href"))
//         .filter(has_extension)
//         .filter_map(normalize_url)
//         .collect::<HashSet<String>>()
// }

// fn normalize_url(url: &str) -> Option<String> {
//     let new_url = Url::parse(url);
//     match new_url {
//         Ok(new_url) => {
//             if new_url.has_host()
//                 && new_url.host_str().unwrap() == "gosugamers.net/heroesofthestorm"
//             {
//                 Some(url.to_string())
//             } else {
//                 None
//             }
//         }
//         Err(_e) => {
//             if url.starts_with("/heroesofthestorm/")
//             /* || url.starts_with("/heroesofthestorm/matches/list") */
//             {
//                 Some(format!("https://www.gosugamers.net{}", url))
//             } else {
//                 None
//             }
//         }
//     }
// }

// fn fetch_url(client: &reqwest::blocking::Client, url: &str) -> String {
//     let mut res = client.get(url).send().unwrap();
//     println!("Status for {}: {}", url, res.status());

//     let mut body = String::new();
//     res.read_to_string(&mut body).unwrap();
//     body
// }

// fn write_file(path: &str, content: &str) {
//     fs::create_dir_all("scraped_html").unwrap();
//     fs::write(format!("scraped_html/{}.html", path), content);
// }

// fn main() {
//     let now = Instant::now();

//     let wait_time = time::Duration::from_secs(3);

//     let client = reqwest::blocking::Client::new();
//     let origin_url = "https://www.gosugamers.net/heroesofthestorm/matches/results";

//     let body = fetch_url(&client, origin_url);

//     write_file("0", &body);
//     let mut visited = HashSet::new();
//     visited.insert(origin_url.to_string());
//     let found_urls = get_links_from_html(&body);
//     let mut new_urls = found_urls
//         .difference(&visited)
//         .map(|x| x.to_string())
//         .collect::<HashSet<String>>();

//     while new_urls.len() > 0 {
//         // let i: i128 = 1;

//         let found_urls: HashSet<String> = new_urls
//             .iter()
//             .map(|url| {
//                 // let j: String = i.to_string();
//                 let body = fetch_url(&client, url);
//                 write_file(&found_urls.len().to_string(), &body);
//                 let links = get_links_from_html(&body);
//                 println!("Visited: {} found {} links", url, links.len());
//                 // let i = i + 1;
//                 thread::sleep(wait_time);
//                 links
//             })
//             .fold(HashSet::new(), |mut acc, x| {
//                 acc.extend(x);
//                 acc
//             });

//         visited.extend(new_urls);
//         new_urls = found_urls
//             .difference(&visited)
//             .map(|x| x.to_string())
//             .collect::<HashSet<String>>();
//         println!("New urls: {}", new_urls.len());
//     }

//     println!("URLs: {:#?}", found_urls);
//     println!("{}", now.elapsed().as_secs())

//     // let mut res = client.get(original_url).send().unwrap();
//     // println!("Status for {}: {}", original_url, res.status());

//     // let mut body = String::new();
//     // res.read_to_string(&mut body).unwrap();

//     // let found_urls = get_links_from_html(&body);
//     // println!("URLs: {:#?}", found_urls);
// }
