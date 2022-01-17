# Parse JSON

## Node

In our last example we made an HTTPS request against the GitHub API with Node and Rust. That worked fine, but we got back a raw string as JSON. Not very usefull. This time we'll learn how to parse the JSON and extract the information we need. For this example we'll request the repositories of a specific GitHub user and we'll log the name and description of the repository and if it was forked.

Our [Node example](../http-requests/README.md) doesn't need to change much to achieve that:

```diff
import { get } from 'https';

const host = 'api.github.com';
-const path = '/users/donaldpipowitch';
+const path = '/users/donaldpipowitch/repos';

function isClientError(statusCode: number) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode: number) {
  return statusCode >= 500;
}

const headers = {
  'User-Agent': 'Mercateo/rust-for-node-developers'
};

+type Repository = {
+  name: string;
+  description: string | null;
+  fork: boolean;
+};

get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => (buf = buf + chunk));

  res.on('end', () => {
-    console.log(`Response: ${buf}`);

    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`;
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`;
    }

+    const repositories: Repository[] = JSON.parse(buf).map(
+      ({ name, description, fork }) => ({ name, description, fork })
+    );
+    console.log('Result is:\n', repositories);
  });
}).on('error', (err) => {
  throw `Couldn't send request.`;
});
```

Our raw response which is stored in `buf` can be easily parsed with the global `JSON` object and its `parse` method. After that we `map` over the returned array to extract just the `name`, `description` and `fork` fields. (The actual response has much more data!) Note that we only _assume_ that `JSON.parse(buf)` returns an array. We are optimistic here, because we think we know the GitHub API, but to be really safe, we should check if our parsed response actually is an array. We _assume_ that `name`, `description` and `fork` exist and are strings, booleans or maybe `null` in case of the `description`, too! Again this is somehow optimistic. GitHub could send us different data. It is up to you as a developer to decide how many safety checks you want to make here. Is this a critical part of your application? How much do you trust GitHub and their API contract?

We also added a `type` called `Repository` to describe our response format. The parsed response has the type `Repository[]` (which means it is an array containing `Repository`'s and can also be written as `Array<Repository>`) and is saved in `repositories`. It is _not mandatory_ to tell TypeScript the type of `repositories` in this case, but it would make further usage of `repositories` easier and safer, because TypeScript would check incorrect usage of `repositories` now. Without adding the type explicitly TypeScript would default to treat `respositories` as [`any`](https://www.typescriptlang.org/docs/handbook/basic-types.html#any) which would result in doing no type checks at all when we use `repositories`.

For the scope of this example it is sufficient not do more runtime checks. Let us test our program:

```
$ npm run -s start
Result is:
 [ { name: 'afpre',
    description: ' CLI for the AWS Federation Proxy',
    fork: true },
  { name: 'ajv',
    description: 'The fastest JSON schema Validator. Supports v5 proposals',
    fork: true },
  ...
```

It works! Nothing complicated and you probably have done this a thousand times, if you use APIs regularly.

## Rust

The state of art way of deserializing a string to JSON is by using the [`serde`](https://github.com/serde-rs/serde) and [`serde_json`](https://github.com/serde-rs/json) crates.

Add all three crates to your `Cargo.toml`:

```diff
[package]
-name = "http-requests"
+name = "parse-json"
version = "1.0.0"
publish = false

[dependencies]
hyper = "0.12.21"
hyper-tls = "0.3.1"
+serde = { version = "1.0", features = ["derive"] }
+serde_json = "1.0"
```

What you see here is the possibility to configure a single crate within the `Cargo.toml`. In this case we enabled a feature called `derive` for `serde` which isn't enabled by default. This allows us to automatically deserialize a JSON string into a custom `struct`.

We do this with a language construct called [_attributes_](https://doc.rust-lang.org/reference/attributes.html). Attributes change the _meaning of an item_ to which they are applied. An item can be a struct declaration for example. They are written as `#[test]` or `#![test]`. `#[test]` would be applied to the _next_ item and `#![test]` would be applied to the _enclosing_ item. E.g.:

```rust
#[hello]
struct SomeStruct;

fn some_function() {
    #![world]
}
```

We can pass additional data to attributes (`#[inline(always)]`) or keys and values (`#[cfg(target_os = "macos")]`).

The attribute we are interested in is called `derive`. It automatically implements certain traits to a custom data structure (in this case a `struct`). The trait we want to derive is called `Deserialize` from the `serde` crate. We'll also derive the build-in `Debug` trait, so we can `println!` our `struct`. A custom `struct` can be created with the `struct` keyword. In our case it has three fields: `name` (which is a `string`), `fork` (which is a `bool`) and `description` (which _maybe_ is a `string`). To express a potentially unavailable value we can use `Option`. `Option` is a little bit like `Result` in the sense that it shows two possible outcomes: `Result` has the successful (`Ok`) and failured (`Err`) cases while `Option` either has _no_ value (the `None` case) or it _has_ a value (the `Some` case).

Having that said this is how we define our custom `struct` called `Repository`:

```rust
#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    description: Option<String>,
    fork: bool,
}
```

Let's add that to the example from the [previous chapter](../http-requests/README.md) and also parse our string:

```diff
use hyper::rt::{run, Future, Stream};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
+use serde::Deserialize;
use std::str::from_utf8;

+#[derive(Deserialize, Debug)]
+struct Repository {
+    name: String,
+    description: Option<String>,
+    fork: bool,
+}

fn main() {
    run(get());
}

fn get() -> impl Future<Item = (), Error = ()> {
    // 4 is number of blocking DNS threads
    let https = HttpsConnector::new(4).unwrap();

    let client = Client::builder().build(https);

-    let req = Request::get("https://api.github.com/users/donaldpipowitch")
+    let req = Request::get("https://api.github.com/users/donaldpipowitch/repos")
        .header("User-Agent", "Mercateo/rust-for-node-developers")
        .body(hyper::Body::empty())
        .unwrap();

    client
        .request(req)
        .and_then(|res| {
            let status = res.status();

-            let buf = res.into_body().concat2().wait().unwrap();
-            println!("Response: {}", from_utf8(&buf).unwrap());

            if status.is_client_error() {
                panic!("Got client error: {}", status.as_u16());
            }
            if status.is_server_error() {
                panic!("Got server error: {}", status.as_u16());
            }

+            let buf = res.into_body().concat2().wait().unwrap();
+            let json = from_utf8(&buf).unwrap();
+            let repositories: Vec<Repository> = serde_json::from_str(&json).unwrap();
+            println!("Result is:\n{:#?}", repositories);

            Ok(())
        })
        .map_err(|_err| panic!("Couldn't send request."))
}
```

Two new things can be seen here.

We used the `Vec` type here, because we get multiple `Repository`'s from the response. ([Remember](../read-files/README.md) that we already used the `vec!` macro in previos chapter, which created a `Vec`.)

The other new thing is the usage of `{:#?}` inside `println!`. So far when we logged a value we used the `println!` macro like this: `println!("Log: {}", some_value);`. To do that `some_value` actually needs to implement the [`Display`](https://doc.rust-lang.org/stable/std/fmt/trait.Display.html) trait. Coming from a JavaScript background you can think of implementing the `Display` trait as providing a nicely formatted `toString` on custom data structures. Sadly `Display` can't be derived automatically. But when all fields in a struct implement `Debug`, we can derive it automatically for custom structs. That's why we use it here. It is an easy way to log custom structs. The usage with `println!` is just a little bit different. You use `{:?}` instead of just `{}`. And if you use `{:#?}` the output will be _pretty printed_. (If you're curious the [string formatting](https://doc.rust-lang.org/std/fmt/index.html) in Rust allows you to do even more cool things, like printing numbers with leading zeros.)
Let us try our program:

```bash
$ cargo -q run
Result is:
Result is:
[
    Repository {
        name: "afpre",
        description: Some(
            " CLI for the AWS Federation Proxy"
        ),
        fork: true
    },
    Repository {
        name: "ajv",
        description: Some(
            "The fastest JSON schema Validator. Supports v5 proposals"
        ),
        fork: true
    },
    ...
]
```

Nice. Applaud yourself. You really learned a lot.

Thank you for reading my articles so far. If you liked them, please let me know. With a little bit of luck I'm able to add new chapters in the future. Maybe about generating WASM and using it in Node Modules? Would you like that? Until then, have a nice day! 👋

---

← [prev _"HTTP requests"_](../http-requests/README.md)
