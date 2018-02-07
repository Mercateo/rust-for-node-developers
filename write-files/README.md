# Write files

## Node

In our [last example](../read-files/README.md) we used `readFile` to read the content of a file and switched to a combination of `openSync` and `readSync` to achieve the same goal in a low-level manner and synchronous way to make it easier comparable to Rust. Now we switch to `readFileSync` to keep things easier. We reuse our basic project structure with typings for this.

In this example we want to read two files: `hello.txt` and `world.txt`. They contain just a single word - `Hello` and `world`. We concatenate the content of both files and write it into a new file `hello-world.txt`. Its contain should be `Hello world!` at the end. (The ` ` and the `!` will be added by us.)

The program could look like this:

```typescript
import { readFileSync, writeFileSync } from 'fs';

let hello;
try {
  hello = readFileSync('hello.txt', 'utf8');
} catch (err) {
  throw `Couldn't read 'hello.txt'.`;
}

let world;
try {
  world = readFileSync('world.txt', 'utf8');
} catch (err) {
  throw `Couldn't read 'world.txt'.`;
}

let helloWorld = `${hello} ${world}!`;

try {
  writeFileSync('hello-world.txt', helloWorld);
} catch (err) {
  throw `Couldn't write 'hello-world.txt'.`;
}

console.log(`Wrote file 'hello-world.txt' with content: ${helloWorld}`);
```

You'll note that we switched our error handling style from something like that:

```typescript
let foo;
try {
  foo = doFooSync();
} catch (err) {
  console.log(`Couldn't foo.`);
  process.exit(1);
}
```

To something like that:

```typescript
let foo;
try {
  foo = doFooSync();
} catch (err) {
  throw `Couldn't foo.`;
}
```

It does _roughly_ the same. We throw a new error with a custom error message and let Node handle the exiting of our process. This code isn't very pretty - normally you would do everything asynchronous and maybe you would use a `Promise` based API with proper `then`/`catch` callbacks. My point here is that different patterns of error handling exist and I want to show you a couple of them in Rust. Talking about error handling was where we stopped in our last example. Now is a good time to focus on error handling, because adding logic to write a file doesn't add a lot of complexity to our program.

Before looking into Rust we should test our program:

```bash
$ npm start -s
Wrote file in hello-world.txt with content: Hello world!
```

Charming.

## Rust

Our _"read files"_ example looked like this:

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = match File::open("hello.txt") {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => println!("Content is: {}", data),
    };
}
```

We want to read to files now, not just one. That is a good time to extract a function which simplifies this. We do this with the `fn` keyword and call our function `read_file` and it will pretty much work like `readFileSync` in our Node example.

```diff
use std::error::Error;
use std::fs::File;
use std::io::Read;

-fn main() {
-    let mut file = match File::open("hello.txt") {
+fn read_file(path: &str) -> String {
+    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(_) => println!("Content is: {}", data),
+        Ok(_) => (),
    };
+    return data;
}

+fn main() {
+    let hello = read_file("hello.txt");
+    let world = read_file("world.txt");

+    println!("Content is: {}", hello);
+    println!("Content is: {}", world);
+}
```

As you can see `read_file` accepts a param called `path` which is a `&str` and it will return (`->`) a value from the type `String`.

The remaining code should be relative self-explanatory. Only returning the actual value is new, but it looks just like JavaScript (`return data;`).

_But_... Rust allows us to write our code like this, too:

```diff
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = match File::open("hello.txt") {
fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => println!("Content is: {}", data),
        Ok(_) => (),
    };
-    return data;
+    data
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
```

What is that? `data` is an _implicit return_ while `return data;` is an _explicit return_. This is possible, because Rust is primarily an _expression-based_ language. In that case `data` is an _expression_ while `return data;` is a _statement_. An expression automatically returns a value and a statement does not. The `;` basically turns our expression into a statement. Therefore the `;` is _not_ meaningless as it is 99% of the time in JavaScript. It is really a little bit similar to arrow functions in JavaScript which can return values automatically (e.g. `[ 1 ].map(n => n + 1) // [ 2 ]`). Using expressions like that allows nice things. For example Rust has _no_ ternary operator (e.g. `let foo = x ? y : z;` in JavaScript), because we can do the same with our normal `if`/`else` keywords and expressions (e.g. `let foo = if x { y } else { z };` in Rust).

I definitely need to get used to writing _implicit returns_ like that even if I use arrow functions very often in JavaScript. The single `data` on the last line looks a little bit weird to me. But to be honest... writing _implicit returns_ is the [prefered style](https://users.rust-lang.org/t/implicit-or-explicit-return-in-non-closures-what-is-more-idiomatic/6357/4) in that case. It is recommended to use the `return` keyword only for _early returns_. We see an example of that in a minute.

We should check that everything works:

```bash
$ cargo run -q
Content is: Hello
Content is: world
```

Fine. Now we can look into different ways of handling errors in Rust before we'll add the actual part of writing files.

The patterns we used for now was using `panic!` which allows adding a custom error message and exits our program. This is okay, but a little bit verbose. The shortest way for error handling is calling `unwrap()` on functions returning a `Result`. (Remember from the [last example](../read-files/README.md) that the `Result` type has an `Err` case and an `Ok` case which we both _need_ to handle. This can be done with pattern matching like we did for now.) `unwrap` _just_ exits the program on `Err` or returns the result on `Ok`, but you cannot pass a custom error message. It looks like that.

```diff
-use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
-    let mut file = match File::open(path) {
-        Err(err) => panic!("Couldn't open: {}", err.description()),
-        Ok(file) => file,
-    };
+    let mut file = File::open(path).unwrap();

    let mut data = String::new();
-    match file.read_to_string(&mut data) {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(_) => (),
-    };
+    file.read_to_string(&mut data).unwrap();
    data
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
```

As you can see our example becomes very readable. That's why you can see this style very often in tutorials. But you rarely use `unwrap` in real code, because it lacks a custom error message.

You can achieve that with `expect` which works like `unwrap`, but accepts custom error messages.

```diff
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
-    let mut file = File::open(path).unwrap();
+    let mut file = File::open(path).expect("Couldn't open file.");

    let mut data = String::new();
-    file.read_to_string(&mut data).unwrap();
+    file.read_to_string(&mut data).expect("Couldn't read file.");
    data
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
```

Nice. Now assume we write a special `fs` crate which offers a similar API as Node to ease the transition for Node developers to use Rust. `read_file` (which we could rename to `read_file_sync` in that case) shouldn't exit the program on errors. It is just a lib - the program needs to decide whether it should exit or not. How do we do that? We need to return an `Result` on our own - and the `std::io::Error` struct in an `Err` case. This will change the return signature of `read_file` and because Rust forces us to handle all errors, the error handling will move up to our `main` function. Guess what... on every error we'll use an _early return_ to return the `err` as our `Err` case (`Err(err)`). Similar we don't just return `data`, but `data` as our `Ok` case (`Ok(data)`).

```diff
use std::fs::File;
-use std::io::Read;
+use std::io::{Read, Error};

-fn read_file(path: &str) -> String {
+fn read_file(path: &str) -> Result<String, Error> {
-    let mut file = File::open(path).expect("Couldn't open file.");
+    let mut file = match File::open(path) {
+        Err(err) => return Err(err),
+        Ok(file) => file,
+    };

    let mut data = String::new();
-    file.read_to_string(&mut data).expect("Couldn't read file.");
+    match file.read_to_string(&mut data) {
+        Err(err) => return Err(err),
+        Ok(_) => (),
+    };
-    data
+    Ok(data)
}

fn main() {
-    let hello = read_file("hello.txt");
+    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
-    let world = read_file("world.txt");
+    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
```

This is a very useful pattern, but it also added a bit of verbosity back. But there is a macro for that! (If you see repeated patterns in your code, chances are hight that you can eliminate them with a macro.) This macro is called `try!` and it will do the same thing we did manually for now: return errors early or accept the result.

```diff
use std::fs::File;
use std::io::{Read, Error};

fn read_file(path: &str) -> Result<String, Error> {
-    let mut file = match File::open(path) {
-        Err(err) => return Err(err),
-        Ok(file) => file,
-    };
+    let mut file = try!(File::open(path));

    let mut data = String::new();
-    match file.read_to_string(&mut data) {
-        Err(err) => return Err(err),
-        Ok(_) => (),
-    };
+    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

    println!("Content is: {}", hello);
    println!("Content is: {}", world);
}
```

Beautiful! Now you know the most common error handling patterns. We can now start to add our file writing logic. First we need to create `String` called `hello_world` which will represent our file content. We can create an empty `String` and push values into it (very much like an `[].push` in JavaScript).

```diff
use std::fs::File;
use std::io::{Read, Error};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

-    println!("Content is: {}", hello);
-    println!("Content is: {}", world);

+    let mut hello_world = String::new();
+    hello_world.push_str(&hello);
+    hello_world.push_str(" ");
+    hello_world.push_str(&world);
+    hello_world.push_str("!");

+    println!("Content is: {}", hello_world);
}
```

`hello_world` needs to be `mut`, because we change its value by pushing new values into it. Pushing is done with the `push_str` method which accepts `&str`'s, but _not_ `String`'s. That's why we pass `&hello` instead of `hello`, because the `&` coerces a `String` into a `&str`.

This example can be simplified a little bit, because we can append `&str`'s with the `+` operator to a `String`.

```diff
use std::fs::File;
use std::io::{Read, Error};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

-    let mut hello_world = String::new();
-    hello_world.push_str(&hello);
-    hello_world.push_str(" ");
-    hello_world.push_str(&world);
-    hello_world.push_str("!");
+    let hello_world = &hello + " " + &world + "!";

    println!("Content is: {}", hello_world);
}
```

But... there is also a macro for that! `format!` which makes it more readable in my opinion. You use it like `println!`, but it just returns a `String`.

```diff
use std::fs::File;
use std::io::{Read, Error};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

-    let hello_world = hello + " " + &world + "!";
+    let hello_world = format!("{} {}!", hello, world);

    println!("Content is: {}", hello_world);
}
```

Now we have our file content and can write it into a file.

```diff
use std::fs::File;
-use std::io::{Read, Error};
+use std::io::{Read, Write, Error};

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = try!(File::open(path));

    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

+fn write_file(path: &str, data: &str) -> Result<(), Error> {
+    let mut file = try!(File::create(path));

+    try!(file.write_all(data.as_bytes()));
+    Ok(())
+}

fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

    let hello_world = format!("{} {}!", hello, world);

-    println!("Content is: {}", hello_world);
+    write_file("hello-world.txt", &hello_world).expect("Couldn't write 'hello-world.txt'.");
+    println!("Wrote file 'hello-world.txt' with content: {}", hello_world);
}
```

We create a new function with `fn` called `write_file`. `write_file` accepts two params `path` and `data` which are both from the type `&str`. It returns a `Result` which is either an `Error` or an "empty" (`()`) `Ok` case. The file is created with `Fill:create` (this _opens_ a file in write-only mode) which is available, because we added `Write` to `use std::io::{}`. We actually write our content into the file with `write_all` which accepts bytes (`&[u8]`). A `&str` can be converted into a `&[u8]` with the `as_bytes` method. After that we return our "empty" `Ok` case `Ok(())`.

Let's try our program:

```bash
$ cargo run -q
Wrote file 'hello-world.txt' with content: Hello world!
```

Awesome! In this example your learned different error handling patterns, different ways to concatenate a string and how to write a file.

______

← [prev](../read-files/README.md) | [next](../http-requests/README.md) →
