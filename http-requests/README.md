# HTTP requests

## Node

This time we'll learn how we can send HTTP requests with Node and Rust. To show this we'll just make a `GET` on the GitHub API to get a user. For our Node example we start with our usual TypeScript setup known from the other examples.

Because GitHubs API is served over `https` we'll use Node's `https` module. We could have used a 3rd party lib like [`superagent`](https://github.com/visionmedia/superagent), because for Rust we'll actually use a 3rd party lib called [`hyper`](http://hyper.rs/), but after trying `hyper` I think it is better to just use Node's built-in `https` for comparision, because they are equally low level.

So this is our basic example:

```typescript
import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

get({ host, path }, (res) => {
  let buf = '';
  res.on('data', (chunk) => buf = buf + chunk);
  res.on('end', () => console.log(`Response: ${buf}`));
}).on('error', (err) => { throw `Couldn't send request.` });
```

We import the `get` function from `https`. We declare a `host` and `path` (no need to set the protocol, when we already use the `https` module). After that we call `get` and pass an options object (containing our `host` and `path`) and callback whic accepts a response object (`res`) as the _first_ parameter. Yes, `get` doesn't follow the usual callback style pattern of Node where the first param is an error and the second param is a result. It is more low level than that. Instead we have an request object (the return value of `get`) and an response object (`res`) which are both event emitters. We listen for `error` events on the request object and in case of an error we just `throw` `Couldn't send request.` to exit our program.

We listen for `data` events on the response object and collect every new `chunk` of data in a string called `buf`. In the case of an `end` event on the response object we know we have our whole response body and log `buf`.

Let us test this program:

```
$ npm run -s start
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.
```

What's going on here? It turns out that we need to set a _user agent_ in our HTTP headers so GitHub allows us to a `GET` on its API. If you follow the link in the error you can read that the user agent should be the name of our GitHub account or the name of our app.

But there's more. This response is actually an _error_, because its status code is `403`. But it was not catched by our `error` event listener. Why? The `error` event listener is only called when our request couldn't be made. In this case we _could_ make a request. The request worked correctly in technical terms. But the _server said_ that our request contains errors (the missing user agent). We need to manually throw client or server errors, if we are interested in them. Let's do that before we add a user agent. To do so I introduce two small helper functions:

```diff
import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

+function isClientError(statusCode) {
+  return statusCode >= 400 && statusCode < 500;
+}

+function isServerError(statusCode) {
+  return statusCode >= 500;
+}

get({ host, path }, (res) => {
  let buf = '';
  res.on('data', (chunk) => buf = buf + chunk);

-  res.on('end', () => console.log(`Response: ${buf}`));
+  res.on('end', () => {
+    console.log(`Response: ${buf}`);
+
+    if (isClientError(res.statusCode)) {
+      throw `Got client error: ${res.statusCode}`
+    }
+    if (isServerError(res.statusCode)) {
+      throw `Got server error: ${res.statusCode}`
+    }
+  });
}).on('error', (err) => { throw `Couldn't send request.` });
````

Test the program again:

```
$ npm run -s start
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.


/Users/donaldpipowitch/Workspace/rust-for-node-developers/http-requests/node/dist/index.js:42
            throw "Got client error: " + res.statusCode;
            ^
Got client error: 403
```

The program exits correctly and logs the status code. It is not beautiful, but it works.

Now we just need to add our headers. We use the name of this repository as our user agent: `Mercateo/rust-for-node-developers`.

```diff
import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

function isClientError(statusCode) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode) {
  return statusCode >= 500;
}

+const headers = {
+  'user-agent': 'Mercateo/rust-for-node-developers'
+};

-get({ host, path }, (res) => {
+get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => buf = buf + chunk);

  res.on('end', () => {
    console.log(`Response: ${buf}`);

    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`
    }
  });
}).on('error', (err) => { throw `Couldn't send request.` });
```

Check the program:

```
$ npm run -s start
Response: {"login":"donaldpipowitch","id":1152805, ...
```

It works! You'll see a nice big blob of JSON as our response. This is fine for now. I'll write about handling JSON in the next example.

Time to move to Rust.

## Rust

As said earlier we'll use a 3rd party lib called [`hyper`](http://hyper.rs/) for our Rust example. It is the _de facto_ standard for working with HTTP in Rust. Currently `hyper` only offers a synchronous API - something which Node doesn't even support, so both examples will work slightly different. However an asynchronous API is [in the work](https://github.com/hyperium/hyper/pull/778) and I'll update this example as soon as it becomes stable.

To use `hyper` we need to append this section to our `Cargo.toml`.

```toml
[dependencies]
hyper = "0.9.0"
```

To check if you can use `hyper` you can create a `src/main.rs` containing nothing but `fn main() {}` and calling `$ cargo run`.

Do you see an error like this one?:

```
$ cargo run
   Compiling url v1.1.1
   Compiling openssl v0.7.14
   Compiling openssl-sys-extras v0.7.14
   Compiling solicit v0.4.4
Build failed, waiting for other jobs to finish...
error: failed to run custom build command for `openssl v0.7.14`
Process didn't exit successfully: `/Users/donaldpipowitch/Workspace/rust-for-node-developers/http-requests/rust/target/debug/build/openssl-2fa77a207dd9f358/build-script-build` (exit code: 101)
--- stdout
TARGET = Some("x86_64-apple-darwin")
OPT_LEVEL = Some("0")
PROFILE = Some("debug")
TARGET = Some("x86_64-apple-darwin")
debug=true opt-level=0
HOST = Some("x86_64-apple-darwin")
TARGET = Some("x86_64-apple-darwin")
TARGET = Some("x86_64-apple-darwin")
HOST = Some("x86_64-apple-darwin")
CC_x86_64-apple-darwin = None
CC_x86_64_apple_darwin = None
HOST_CC = None
CC = None
HOST = Some("x86_64-apple-darwin")
TARGET = Some("x86_64-apple-darwin")
HOST = Some("x86_64-apple-darwin")
CFLAGS_x86_64-apple-darwin = None
CFLAGS_x86_64_apple_darwin = None
HOST_CFLAGS = None
CFLAGS = None
running: "cc" "-O0" "-ffunction-sections" "-fdata-sections" "-g" "-m64" "-fPIC" "-o" "/Users/donaldpipowitch/Workspace/rust-for-node-developers/http-requests/rust/target/debug/build/openssl-2fa77a207dd9f358/out/src/c_helpers.o" "-c" "src/c_helpers.c"
ExitStatus(ExitStatus(256))


command did not execute successfully, got: exit code: 1



--- stderr
src/c_helpers.c:1:10: fatal error: 'openssl/ssl.h' file not found
#include <openssl/ssl.h>
         ^
1 error generated.
thread '<main>' panicked at 'explicit panic', /Users/donaldpipowitch/.cargo/registry/src/github.com-88ac128001ac3a9a/gcc-0.3.31/src/lib.rs:840
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

If yes, you probably have an error with OpenSSL. Just follow [these steps](https://github.com/sfackler/rust-openssl#building) to solve the error.

On Mac I had to install `openssl` with `brew` like this:

```bash
brew install openssl
```

After that I had to add this to my `.bash_profile`:

```
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```

Now you can call `$ cargo clean` and run `$ cargo run` again without any errors.

With this out of the way we can write our Rust example. We'll start exactly like our Node example _without_ setting a user agent.

```rust
extern crate hyper;

use std::io::Read;
use hyper::Client;

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch";

    let client = Client::new();
    let mut res = client.get(url).send().expect("Couldn't send request.");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");
    println!("Response: {}", buf);
}
```

First we tell the compiler that we want to use an external crate called `hyper`. Than we import `std::io::Read` which you should know from our [_read files exampe_](../read-files/README.md) and we import `hyper::Client` which allows us to make requests.

In our `main` function we set our `url` containing the protocol, host and path for our request. After that we create a new instance of the [`Client` struct](http://hyper.rs/hyper/0.8.0/hyper/client/struct.Client.html) which we call `client`. `client` has a `get` method which we can pass our `url` into. It will _not_ start the request immediately, but return an instance of the [`RequestBuilder` struct](http://hyper.rs/hyper/v0.9.10/hyper/client/struct.RequestBuilder.html). This one has a `send` method which will start the request. `send` returns `Result<Response>`, so we call `expect` to handle the `Error` case and get an instance of [`Response` struct](http://hyper.rs/hyper/0.8.0/hyper/client/response/struct.Response.html) as the result in the `Ok` case.

After that we read our result into a buffer (`buf`) and print the response body. This should also be familiar from the [_read files exampe_](../read-files/README.md).

If you run our program now you see the same error as in our first Node example:

```
$ cargo run -q
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.
```

Again no error was reported in the case of a `403` response. Let's change that. Thankfully `hyper` can handle status codes easily.

```diff
extern crate hyper;

use std::io::Read;
use hyper::Client;

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch";

    let client = Client::new();
    let mut res = client.get(url).send().expect("Couldn't send request.");

    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Couldn't read response.");
    println!("Response: {}", buf);

+    if res.status.is_client_error() {
+        panic!("Got client error: {}", res.status);
+    }
+    if res.status.is_server_error() {
+        panic!("Got server error: {}", res.status);
+    }
}
```

Our `Response` struct (`res`) has a field `status` which is a [`StatusCode` enum](http://hyper.rs/hyper/0.8.0/hyper/status/enum.StatusCode.html). In Rust enums can have methods and with `is_client_error` and `is_server_error` we can easily check for the errors we are interested in.

Note that we use an `if` statement in Rust for the first time. Unlike JavaScript we write it without additional parantheses around the `if` condition. (We _could_ write it that way, but the compiler warns us about unnecessary parentheses.)

Test the program again:

```
$ cargo run -q
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.

thread '<main>' panicked at 'Got client error: 403 Forbidden', src/main.rs:50
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/http-requests` (exit code: 101)
```

Great. Our program reports the status code and exits. Again... this is not beautiful, but what we currently want.

Now let us add the user agent. A quote from the docs:

> Hyper's header representation is likely the most complex API exposed by Hyper.

The usage of headers is way more specific (but also safer) than Node's way of handling this. Let us look at the final example:

```diff
extern crate hyper;

use std::io::Read;
use hyper::Client;
+use hyper::header::{Headers, UserAgent};

fn main() {
    let url = "https://api.github.com/users/donaldpipowitch";

+    let mut headers = Headers::new();
+    headers.set(UserAgent("Mercateo/rust-for-node-developers".to_string()));

    let client = Client::new();
-    let mut res = client.get(url).send().expect("Couldn't send request.");
+    let mut res = client.get(url)
+        .headers(headers)
+        .send()
+        .expect("Couldn't send request.");

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
```

We import `Headers` and `UserAgent` from `hyper::header`. We create a new instance of the [`Headers` struct](http://hyper.rs/hyper/v0.9.10/hyper/header/struct.Headers.html) which we call `headers`. Now we can set different fields to `headers` with the `set` method. This function accepts a [`UserAgent` struct](http://hyper.rs/hyper/v0.9.10/hyper/header/struct.UserAgent.html) among other structs for other fields. We just need to pass `"Mercateo/rust-for-node-developers".to_string()` directly to `UserAgent`.

The last thing we need to do is passing our `headers` to our actually request with the `headers` method, just before we call `send`. Done!

```
$ cargo run -q
Response: {"login":"donaldpipowitch","id":1152805, ...
```

The Node and the Rust example both show the same result now. Nice. In the next example I'll show you how to actually handle a JSON response.

______

← [prev](../write-files/README.md) | [next](../parse-json/README.md) →
