use hyper::rt::{run, Future, Stream};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::str::from_utf8;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    description: Option<String>,
    fork: bool,
}

fn main() {
    run(get());
}

fn get() -> impl Future<Item = (), Error = ()> {
    // 4 is number of blocking DNS threads
    let https = HttpsConnector::new(4).unwrap();

    let client = Client::builder().build(https);

    let req = Request::get("https://api.github.com/users/donaldpipowitch/repos")
        .header("User-Agent", "Mercateo/rust-for-node-developers")
        .body(hyper::Body::empty())
        .unwrap();

    client
        .request(req)
        .and_then(|res| {
            let status = res.status();

            if status.is_client_error() {
                panic!("Got client error: {}", status.as_u16());
            }
            if status.is_server_error() {
                panic!("Got server error: {}", status.as_u16());
            }

            let buf = res.into_body().concat2().wait().unwrap();
            let json = from_utf8(&buf).unwrap();
            let repositories: Vec<Repository> = serde_json::from_str(&json).unwrap();
            println!("Result is:\n{:#?}", repositories);

            Ok(())
        })
        .map_err(|_err| panic!("Couldn't send request."))
}
