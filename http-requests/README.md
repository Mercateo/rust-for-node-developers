# HTTP requests

## Node

This time we'll learn how we can send HTTP requests with Node and Rust. To show this we'll just make a `GET` on the GitHub API to get a user. For our Node example we start with our usual TypeScript setup known from the other examples.

Because GitHubs API is served over `https` we'll use Node's `https` module. We could have used a 3rd party lib like [`superagent`](https://github.com/visionmedia/superagent), because for Rust we'll actually use a 3rd party lib called [`hyper`](http://hyper.rs/), but after trying `hyper` I think it is better to just use Node's built-in `https` for comparision, because they are equally low level.

So this is our basic example:

```ts
import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

get({ host, path }, (res) => {
  let buf = '';
  res.on('data', (chunk) => (buf = buf + chunk));
  res.on('end', () => console.log(`Response: ${buf}`));
}).on('error', (err) => {
  throw `Couldn't send request.`;
});
```

We import the `get` function from `https`. We declare a `host` and `path` (no need to set the protocol, when we already use the `https` module). After that we call `get` and pass an options object (containing our `host` and `path`) and callback which accepts a response object (`res`) as the _first_ parameter. Yes, `get` doesn't follow the usual callback style pattern of Node where the first param is an error and the second param is a result. It is more low level than that. Instead we have an request object (the return value of `get`) and an response object (`res`) which are both event emitters. We listen for `error` events on the request object and in case of an error we just `throw` `Couldn't send request.` to exit our program.

We listen for `data` events on the response object and collect every new `chunk` of data in a string called `buf`. In the case of an `end` event on the response object we know we have our whole response body and log `buf`.

Let us test this program:

```
$ npm run -s start
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.
```

What's going on here? It turns out that we need to set a _user agent_ in our HTTP headers so GitHub allows us to a `GET` on its API. If you follow the link in the error you can read that the user agent should be the name of our GitHub account or the name of our app.

But there's more. This response is actually an _error_, because its status code is `403`. But it was not catched by our `error` event listener. Why? The `error` event listener is only called when our request couldn't be made. In this case we actually _made_ a request. The request worked correctly in technical terms. But the _server said_ that our request contains errors (the missing user agent). We need to manually throw client or server errors, if we are interested in them. Let's do that before we add a user agent. To do so I introduce two small helper functions:

```diff
import { get } from 'https';

const host = 'api.github.com';
const path = '/users/donaldpipowitch';

+function isClientError(statusCode: number) {
+  return statusCode >= 400 && statusCode < 500;
+}

+function isServerError(statusCode: number) {
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
+      throw `Got client error: ${res.statusCode}`;
+    }
+    if (isServerError(res.statusCode)) {
+      throw `Got server error: ${res.statusCode}`;
+    }
+  });
}).on('error', (err) => {
  throw `Couldn't send request.`;
});
```

Test the program again:

```
$ npm run -s start
Response: Request forbidden by administrative rules. Please make sure your request has a User-Agent header (http://developer.github.com/v3/#user-agent-required). Check https://developer.github.com for other possible causes.


/Users/pipo/workspace/rust-for-node-developers/http-requests/node/dist/index.js:21
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

function isClientError(statusCode: number) {
  return statusCode >= 400 && statusCode < 500;
}

function isServerError(statusCode: number) {
  return statusCode >= 500;
}

+const headers = {
+  'User-Agent': 'Mercateo/rust-for-node-developers'
+};

-get({ host, path }, (res) => {
+get({ host, path, headers }, (res) => {
  let buf = '';
  res.on('data', (chunk) => (buf = buf + chunk));

  res.on('end', () => {
    console.log(`Response: ${buf}`);

    if (isClientError(res.statusCode)) {
      throw `Got client error: ${res.statusCode}`;
    }
    if (isServerError(res.statusCode)) {
      throw `Got server error: ${res.statusCode}`;
    }
  });
}).on('error', (err) => {
  throw `Couldn't send request.`;
});
```

Check the program:

```
$ npm run -s start
Response: {"login":"donaldpipowitch","id":1152805, ...
```

It works! You'll see a nice big blob of JSON as our response. This is fine for now. I'll write about handling JSON in the next example.

Time to move to Rust.

## Rust

As I said earlier we'll use a 3rd party lib called [`hyper`](http://hyper.rs/) for our Rust example. It is the _de facto_ standard for working with HTTP(S) in Rust. But I have to tell you something about asynchronous APIs (like doing network requests) in Rust.

As you may know JavaScript is a single-threaded lanugage and all asynchronous APIs are driven by [an event loop](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop). You probably also know about [Promises](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_promises) which allow us to model asynchronous control flow and that the `async`/`await` syntax for functions are build on top of Promises. (Disclaimer: There are more ways to handle asynchronity than by using Promises. In our own chapter from above we used callbacks for example.) You may also recall that it took a while until Promises and `async`/`await` were fully standardized and landed natively in JavaScript.

Rust is basically in the same progress. There will be an `async` keyword and an `await!` macro which will behave similar to JavaScripts `async`/`await`. There is also a concept similar to Promises which is called [Futures](https://github.com/rust-lang-nursery/futures-rs), but they aren't fully standardized yet. You can follow the whole progress on [_"Are we `async` yet?"_](https://areweasyncyet.rs/). But there are also key differences to JavaScript: Rust is multi-threaded and has no build-in event loop.

So everything you'll now see can and will probably change in the future. Take it with a grain of salt.

To use `hyper` we need to append this section to our `Cargo.toml`. This will also add [`hyper-tls`](https://github.com/hyperium/hyper-tls) which we need for HTTPS. It looks like this was kept modular to allow different resolution strategies to get certificates.

```toml
[dependencies]
hyper = "0.12.21"
hyper-tls = "0.3.1"
```

Let's start with an high level overview of our file this time:

```rust
use hyper::rt::{run, Future, Stream};
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use std::str::from_utf8;

fn main() {
    run(get());
}

fn get() -> impl Future<Item = (), Error = ()> {
    // more code here
}
```

`hyper`'s `run` function - as far as I understand - solves a similar problem as our event loop in JavaScript. We can pass a Future to `run`, so it knows when the asynchronous APIs have "finished" its job. Futures are currently bundled in `hyper`. (Think about all the 3rd party Promise libs like [Bluebird](https://github.com/petkaantonov/bluebird) we used - and sometimes still use - in the JavaScript world, before Promises were added to the language.)

As you can I create a custom function called `get` which returns "something" that `impl`'emented Future. Just like Promises which can resolve or reject our Future can represent a successful (`Item`) or erroneous (`Error`) outcome. But in both cases I'm not really interested in the result, so I'll just return `()`.

Now we only need to look into our `get` function.

```rust
fn get() -> impl Future<Item = (), Error = ()> {
    // 4 is number of blocking DNS threads
    let https = HttpsConnector::new(4).unwrap();

    let client = Client::builder().build(https);

    let req = Request::get("https://api.github.com/users/donaldpipowitch")
        .header("User-Agent", "Mercateo/rust-for-node-developers")
        .body(hyper::Body::empty())
        .unwrap();

    // more coded here
}
```

First we create our `HttpsConnector` which is allows up to 4 blocking DNS threads. (If you don't understand that, it's fine. We don't need to technically understand that part to understand the example in general. The `HttpsConnector` just enables us to make HTTPS requests.) As you can see I used `unwrap()` here instead of our `?` operator. Currently it is not possible out of the box to use `?` inside a function which returns a Future. Sadly I couldn't found an RFC or discussions about this topic and I'd like to see how to handle this case more gracefully.

The next thing we create is a `Client` which will make the actual requests. It is uses the [`builder` pattern](https://en.wikipedia.org/wiki/Builder_pattern) which is _really_ popular in the Rust ecosystem in my experience. In this case we just pass our `HttpsConnector` instance to the builder and get a client back.

Last but not least we configure our request. It will be a `GET` request (that's why we use `Request::get`), we pass an url, we set the `User-Agent` header and set an empty buddy.

Now we'll need to pass our configured request to the client so it actually executes the request and we can handle the response.

```rust
fn get() -> impl Future<Item = (), Error = ()> {
    // previous code

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
```

The client gets our request _and then_ we handle the response (by using `and_then` which is similar to `then` in a Promise) or handle the error (by using `map_err` which is similar to `catch` in a Promise) if the request couldn't been made at all. In the error case we just `panic!`. We'll do that for all error cases, that's why I just wrote `Error = ()` in the function signature as we don't return a useful error.

If a request could be made we'll get the response (`res`). The response has several useful methods to extract the status (`status()`) and body (`into_body().concat2().wait()`, because the body comes in chunks - similar to our Node example). With `status.is_client_error()` and `status.is_server_error()` we can easily check for 4xx and 5xx error codes. `status.as_u16()` returns the plain status code (e.g. `403`) without the canonical reason (e.g. `Forbidden`). Note that we return `Ok(())` instead of just `()`, [because `()` doesn't implement the Future trait, but `Result` does it](https://stackoverflow.com/questions/46625376/the-trait-bound-futuresfuture-is-not-satisfied-when-using-tcpconnectionn/49331886#49331886).

If you run the program now you should get the same output as we did in the Node example.

```
$ cargo -q run
Response: {"login":"donaldpipowitch","id":1152805, ...
```

This is great, but I actually hid a problem from you. In my original code I had written this:

```diff
-            let status = res.status();
-
-            let buf = res.into_body().concat2().wait().unwrap();
-            println!("Response: {}", from_utf8(&buf).unwrap());

+            let buf = res.into_body().concat2().wait().unwrap();
+            println!("Response: {}", from_utf8(&buf).unwrap());
+
+            let status = res.status();
```

This made more sense in my opinion as I used the `status` _after_ I used the `buf`. But this throws a compiler error:

```
26 |             let buf = res.into_body().concat2().wait().unwrap();
   |                        --- value moved here
...
29 |             let status = res.status();
   |                          ^^^ value borrowed here after move
```

This error occurs when an attempt is made to use a variable after its contents have been "moved" elsewhere. This is Rusts _ownership_ model which we already mentioned in a [previous chapter](../read-files/README.md) in action. For me it's by far the most complex new concept to understand in Rust. There can only be one "owner" of some content or data at a single point in time. Originally `res` holds the data corresponding to response body, but by calling `res.into_body()` the ownership is transferred and is given to our `buf` variable at the end. After this line no one is allowed to access `res` anymore. It wouldn't be a problem if we could create a _reference_ to the body by calling `res.body()` (similar to `res.status()` which gives us a reference to the status), but I'm not sure if it's possible to get the actual body content as a string from a _referenced_ body.

Nice. In the next example I'll show you how to actually handle a JSON response.

---

← [prev _"Write files"_](../write-files/README.md) | [next _"Parse JSON"_](../parse-json/README.md) →
