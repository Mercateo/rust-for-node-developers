use hyper::rt::{run, Future, Stream};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use std::str::from_utf8;

fn main() {
    run(get());
}

fn get() -> impl Future<Item = (), Error = ()> {
    // 4 is number of blocking DNS threads
    let https = HttpsConnector::new(4).unwrap();

    let client = Client::builder().build(https);

    let req = Request::get("https://api.github.com/users/donaldpipowitch")
        .header("User-Agent", "Mercateo/rust-for-node-developers")
        .body(hyper::Body::empty())
        .unwrap();

    client
        .request(req)
        .and_then(|res| {
            let status = res.status();

            let buf = res.into_body().concat2().wait().unwrap();
            println!("Response: {}", from_utf8(&buf).unwrap());

            if status.is_client_error() {
                panic!("Got client error: {}", status.as_u16());
            }
            if status.is_server_error() {
                panic!("Got server error: {}", status.as_u16());
            }

            Ok(())
        })
        .map_err(|_err| panic!("Couldn't send request."))
}
