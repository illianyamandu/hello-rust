use std::io;
use reqwest::blocking::{Client, ClientBuilder};

fn main() {
    let http_client = Client::new();
    let result = http_client.get("http://localhost/tikmais-pdv-web/public/api/app/evento")
        .send();

    if result.is_ok() {
        println!("Body: {:#?}", result.ok().unwrap().text().unwrap());
        // let response = result.unwrap();
        // let status = response.status();
        // let body = response.text().unwrap();
        // println!("Status: {}", status);
        // println!("Body: {}", body);
    } else {
        println!("Error: {:?}", result.err());
    }
}
