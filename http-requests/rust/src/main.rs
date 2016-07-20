extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, UserAgent};

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch";

    let mut headers = Headers::new();
    headers.set(UserAgent("Mercateo/rust-for-node-developers".to_string()));

    let client = Client::new();
    let mut res = client.get(url)
        .headers(headers)
        .send()
        .expect("Couldn't send request.");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");
    println!("Response: {}", buf);

    if res.status.is_client_error() {
        panic!("Got client error: {}", res.status);
    }
    if res.status.is_server_error() {
        panic!("Got server error: {}", res.status);
    }
}

// ---

// extern crate hyper;

// use std::io::Read;
// use hyper::Client;

// fn main() {
//     let url = "https://api.github.com/users/donaldpipowitch";

//     let client = Client::new();
//     let mut res = client.get(url).send().expect("Couldn't send request.");

//     let mut buf = String::new();
//     res.read_to_string(&mut buf).expect("Couldn't read response.");
//     println!("Response: {}", buf);

//     if res.status.is_client_error() {
//         panic!("Got client error: {}", res.status);
//     }
//     if res.status.is_server_error() {
//         panic!("Got server error: {}", res.status);
//     }
// }

// ---

// extern crate hyper;

// use std::io::Read;
// use hyper::Client;

// fn main() {
//     let url = "https://api.github.com/users/donaldpipowitch";

//     let client = Client::new();
//     let mut res = client.get(url).send().expect("Couldn't send request.");

//     let mut buf = String::new();
//     res.read_to_string(&mut buf).expect("Couldn't read response.");
//     println!("Response: {}", buf);
// }
