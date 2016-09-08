#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde_json;

use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, UserAgent};
use serde_json::from_str;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    description: String,
    fork: bool,
}

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch/repos";

    let mut headers = Headers::new();
    headers.set(UserAgent("Mercateo/rust-for-node-developers".to_string()));

    let client = Client::new();
    let mut res = client.get(url)
        .headers(headers)
        .send()
        .expect("Couldn't send request.");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");

    if res.status.is_client_error() {
        panic!("Got client error: {}", res.status);
    }
    if res.status.is_server_error() {
        panic!("Got server error: {}", res.status);
    }

    let repositories: Vec<Repository> = from_str(&buf).expect("Couldn't parse response.");
    println!("Result is:\n{:?}", repositories);
}
