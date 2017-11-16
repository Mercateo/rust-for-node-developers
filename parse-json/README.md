# Parse JSON

## Node

In our last example we made an HTTPS request against the GitHub API with Node and Rust. That worked fine, but we got back a raw string as JSON. Not very usefull. This time we'll learn how to parse the JSON and extract the information we need. For this example we'll request the repositories of a specific GitHub user and we'll log the name and description of the repository and if it was forked.

Our [Node example](../http-requests/README.md) doesn't need to change much to achieve that:

```diff
import { get } from 'https';

const host = 'api.github.com';
-const path = '/users/donaldpipowitch';
+const path = '/users/donaldpipowitch/repos';

function isClientError(statusCode) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode) {
  return statusCode >= 500;
}

const headers = {
  'user-agent': 'Mercateo/rust-for-node-developers'
};

+interface Repository {
+  name: string;
+  description: string;
+  fork: boolean;
+}

get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => buf = buf + chunk);

  res.on('end', () => {
-    console.log(`Response: ${buf}`);

    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`
    }

+    const repositories: Array<Repository> = JSON.parse(buf)
+      .map(({ name, description, fork }) => ({ name, description, fork }));
+    console.log(`Result is:\n`, repositories);
  });
}).on('error', (err) => { throw `Couldn't send request.` });

```

Our raw response which is stored in `buf` can be easily parsed with the global `JSON` object and its `parse` method. After that we `map` over the returned array to extract just the `name`, `description` and `fork` fields. (The actual response has much more data!) Note that we only _assume_ that `JSON.parse(buf)` returns an array. We are optimistic here, because we think we know the GitHub API, but to be really save, we should check if our parsed response actually is an array. We _assume_ that `name`, `description` and `fork` exist and are strings or booleans, too! Again this is somehow optimistic. GitHub could send us different data. It is up to you as a developer to decide how many safety checks you want to make here. Is this a critical part of your application? How much do you trust GitHub and there API contract?

We added an `interface` called `Repository` to describe our response format. The parsed response has the type `Array<Repository>` and is saved in `repositories`. It is _not mandatory_ to tell TypeScript the type of `repositories` in this case, but it would make further usage of `repositories` easier and safer, because TypeScript would check incorrect usage of `repositories`.

For this example it is sufficient not do more checks. Let us test our program:

```
$ npm run -s start
Result is:
 [ { name: 'ajv',
    description: 'The fastest JSON schema Validator. Supports v5 proposals',
    fork: true },
  { name: 'angular',
    description: 'Code to optimize AngularJS for complex pages',
    fork: true },
  ...
```

It works! Nothing complicated and you probably have done this a thousand time, if you're a JavaScript developer.

## Rust

Parsing JSON is a little bit trickier currently in Rust. The state of art way of deserializing a string to JSON is by using the [`serde`](https://github.com/serde-rs/serde) and [`serde_json`](https://github.com/serde-rs/json) crates. But to do that ergonomically we also need [`serde_macros`](https://github.com/serde-rs/serde/tree/master/serde_macros).

Add all three crates to your `Cargo.toml`:

```diff
[package]
-name = "http-requests"
+name = "parse-json"
version = "1.0.0"
publish = false

[dependencies]
hyper = "0.9"
+serde = "0.8"
+serde_macros = "0.8"
+serde_json = "0.8"
```

Sadly there is one more step involved to parse JSON with `serde`. We need to switch to a nightly build of Rust. At least for now. I'll tell you in a minute why. Just install nightly with [`rustup`](../setup/README.md).

```bash
$ rustup install nightly
...
  nightly-x86_64-apple-darwin installed - rustc 1.13.0-nightly (923bac459 2016-09-06)
```

If you want to you can make nightly your new default. It has some unstable features, but nicer error messages. You do it that way:

```bash
$ rustup default nightly
info: using existing install for 'nightly-x86_64-apple-darwin'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin unchanged - rustc 1.13.0-nightly (923bac459 2016-09-06)
```

So why do we need nightly here? We want to use a Rust feature called [_attributes_](https://doc.rust-lang.org/book/attributes.html). Attributes change the _meaning of an item_ to which they are applied. An item can be a struct declaration for example. They are written as `#[test]` or `#![test]`. `#[test]` would be applied to the _next_ item and `#![test]` would be applied to the _enclosing_ item. E.g.:

```rust
#[foo]
struct Foo;

mod bar {
    #![bar]
}
```

We can pass additional data to attributes (`#[inline(always)]`) or keys and values (`#[cfg(target_os = "macos")]`).

For now _every_ attribute is defined by the Rust compiler and we want to use an attribute called `derive` which is only available in nightlies. `derive` allows us to automatically implement certain traits for a custom struct. The trait we're interested in is [`Deserialize`](https://docs.serde.rs/serde/de/trait.Deserialize.html) from... you guessed it: `serde`!

Let us look at modified [Rust example](../http-requests/README.md) :

```diff
+#![feature(custom_derive, plugin)]
+#![plugin(serde_macros)]

extern crate hyper;
+extern crate serde_json;

use std::io::Read;
use hyper::Client;
use hyper::header::{Headers, UserAgent};
+use serde_json::from_str;

+#[derive(Deserialize, Debug)]
+struct Repository {
+    name: String,
+    description: Option<String>, // description is an Option<String> here, as it is optional field in github repos and might return null in JSON
+    fork: bool,
+}

fn main() {
-    let url = "https://api.github.com/users/donaldpipowitch";
+    let url = "https://api.github.com/users/donaldpipowitch/repos";

    let mut headers = Headers::new();
    headers.set(UserAgent("Mercateo/rust-for-node-developers".to_string()));

    let client = Client::new();
    let mut res = client.get(url)
        .headers(headers)
        .send()
        .expect("Couldn't send request.");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");
-    println!("Response: {}", buf);

    if res.status.is_client_error() {
        panic!("Got client error: {}", res.status);
    }
    if res.status.is_server_error() {
        panic!("Got server error: {}", res.status);
    }

+    let repositories: Vec<Repository> = from_str(&buf).expect("Couldn't parse response.");
+    println!("Result is:\n{:?}", repositories);
}
```

First we enable two compiler features (`custom_derive` and `plugin`) with `#![feature(custom_derive, plugin)]`. These are needed to use the compiler plugin of `serde_macros`. [Compiler plugins](https://doc.rust-lang.org/book/compiler-plugins.html) can extend Rusts compiler behavior with syntax extensions, linter checks and so on. The plugin is loaded in the next line with `#![plugin(serde_macros)]`.

We import `serde_json` and its `from_str` function, which will decode our JSON from our `buf` string.

We declare our `Repository` struct and its fields. The struct has the following attribute attached to it: `#[derive(Deserialize, Debug)]`. We derive `Deserialize` which will handle the deserialization of our JSON into our struct. But what is `Debug` and where does it come from? So far when we logged a value we used the `println!` macro like this: `println!("Log: {}", foo);`. To do that `foo` actually needs to implement the [`fmt::Display`](https://doc.rust-lang.org/stable/std/fmt/trait.Display.html). Coming from a JavaScript background you can think of implementing the `Display` trait as providing a nicely formatted `toString` on custom data structures. But what is `Debug` now? It is the [`fmt::Debug`](https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html) trait. `Debug` is more generic than `Format` and can be _automatically_ derived for a struct if all of its fields implement `Debug`. That's why we use it here. It is an easy we to log custom structs. The usage with `println!` is just a little bit different. You use `{:?}` instead of just `{}`. That's it!

Having our `Repository` struct and its attributes setup correctly everything boils down to these two lines:

```rust
let repositories: Vec<Repository> = from_str(&buf).expect("Couldn't parse response.");
println!("Result is:\n{:?}", repositories);
```

We call `from_str` and pass our `buf` as a reference. `from_str` will try to parse our response as `Vec<Repository>`. If everything works without an error, we log `repositories` with `println!("Result is:\n{:?}", repositories);`. (Note the usage of `{:?}`.)

Run it now and you should see this:

```bash
$ cargo run -q
Result is:
[Repository { name: "ajv", description: Some("The fastest JSON schema Validator. Supports v5 proposals"), fork: true }, Repository { name: "angular", description: Some("Code to optimize AngularJS for complex pages"), fork: true }, ...]
```

You see that our `Vec<Repository>` is printed with `{:?}` in the format `[Repository { name, description, fork }, ...]`. Thanks to `#[derive(Debug)]`! Notice that `description` has a form of `Some(...)` - in the corresponding struct it was declared as an `Option<String>`, because the description field is optional and non-required on every github repo.

But that is not all: The [string formatting](https://doc.rust-lang.org/nightly/std/fmt/index.html) in Rust allow you to do a bunch of really cool things, e.g., padding inputs to a certain length or choosing representations for numbers. One of the most amazing features is the "alternate mode", which you can trigger with an `#`. The alternate mode for `Debug` is to pretty-print the data, so it gets split up in lines and is nicely indented!

If we change our `println` to

```rust
println!("Result is:\n{:#?}", repositories);
//                      ^-- added a `#` here
```

it will now output our data like this:

```bash
$ cargo run -q
Result is:
[
    Repository {
        name: "ajv",
        description: Some(
          "The fastest JSON schema Validator. Supports v5 proposals"
        ),
        fork: true
    },
    Repository {
        name: "angular",
        description: Some(
          "Code to optimize AngularJS for complex pages"
        ),
        fork: true
    },
    ...
]
```

Phew. A lot of new concepts and unstable features to parse JSON in an ergonomic way. But it _is_ ergonomic and powerful, if you use these features.

I guess as a JavaScript developers you just need to get comfortable to move more work to the Rust compiler, so you can work more declarative using attributes in Rust.

______

← [prev](../http-requests/README.md)
