// This a web crawler to download some bird images from Wikipedia.
//
// To run it:
//      cargo run
//
// References used to build it:
// https://www.youtube.com/watch?v=HCwMb0KslX8
// https://blog.logrocket.com/web-scraping-rust/
// https://rolisz.ro/2020/03/01/web-crawler-in-rust/
// https://hackernoon.com/parsing-html-with-rust-a-simple-tutorial-using-tokio-reqwest-and-scraper
use scraper::{Html, Selector};

use std::env;
use std::{thread, time};

// TODO: return error instead of panicking.
// TODO: run it async.
fn download_bird_image(image_page_path: &String) {
    let image_page_url = "https://en.wikipedia.org".to_owned() + &image_page_path;

    let client = reqwest::blocking::Client::new();
    println!("\n\nGetting image page url {}", image_page_url);

    let res = client.get(&image_page_url).send().unwrap();
    println!("Status for {}: {}", image_page_url, res.status());

    let text = res.text().expect("could not get request");

    let document = Html::parse_document(&text);
    let img_selector = Selector::parse(".fullImageLink a").expect("could not build parser");

    let link = document.select(&img_selector).next().unwrap();

    // Wikipedia respond with 403 status code sometimes,
    // so we need to retry the download for those cases.
    for attempt in 1..5 {
        let image_path = link
            .value()
            .attr("href")
            .expect("href attr not foud")
            .to_string();

        let image_url = "https:".to_owned() + &image_path;

        if attempt > 1 {
            let sleep_time = time::Duration::from_millis(500);
            thread::sleep(sleep_time);
        }

        println!("Found image url: {}", image_url);

        let file_name = image_page_path.replace("/wiki/File:", "");
        let file_path = "./static/".to_owned() + &file_name;
        println!("Creating file: {}", file_path);

        let mut file = std::fs::File::create(&file_path).unwrap();

        let mut res_img = reqwest::blocking::get(&image_url).unwrap();
        println!("Status for {}: {}", image_url, res_img.status());

        if res_img.status().is_success() {
            res_img.copy_to(&mut file).unwrap();
            println!("Downloaded image url: {}", image_url);
            break;
        } else if attempt == 5 {
            println!("Failed to download image url: {}", image_url);
        }
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let client = reqwest::blocking::Client::new();
    let origin_url = "https://en.wikipedia.org/wiki/Bird";

    let res = client.get(origin_url).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let text = res.text().expect("could not get request");

    let document = Html::parse_document(&text);
    let birds_selector =
        Selector::parse(".clade .clade-leaf .mw-file-description").expect("could not build parser");

    let mut total: i32 = 0;
    for title in document.select(&birds_selector) {
        total += 1;

        let image_page_path = title
            .value()
            .attr("href")
            .expect("href attr not foud")
            .to_string();

        download_bird_image(&image_page_path);

        if total == 3 {
            // Keep it simple and don't overload Wikipedia with
            // this demo.
            break;
        }
    }

    println!("Total: {:?}", total);
}
