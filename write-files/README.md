# Write files

## Node

In our [last example](../read-files/README.md) we used `readFile` to read the content of a file and switched to a combination of `openSync` and `readSync` to achieve the same goal in a low-level manner and synchronous way to make it easier to compare to Rust. Now we switch to `readFileSync` to keep things a little bit easier. We reuse our basic project structure for the next example as well.

In this example we want to read two files: `hello.txt` and `world.txt`. They contain just a single word - `Hello` and `world`. We concatenate the content of both files and write it into a new file `hello-world.txt`. It should contain `Hello world!` at the end. (The space and the `!` will be added by us.)

The program could look like this:

```ts
import { readFileSync, writeFileSync } from 'fs';

let hello: string;
try {
  hello = readFileSync('hello.txt', 'utf8');
} catch (err) {
  throw `Couldn't read 'hello.txt'.`;
}

let world: string;
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

```ts
let someValue;
try {
  someValue = getSomeValue();
} catch (err) {
  console.log(`Couldn't get value.`);
  process.exit(1);
}
```

To something like that:

```ts
let someValue;
try {
  someValue = getSomeValue();
} catch (err) {
  throw `Couldn't get value.`;
}
```

It does _roughly_ the same. We throw an exeption with a custom error message and let Node handle the exiting of our process. This might be not very pretty, but my point here is that different patterns of error handling exist. We already learned about [Nodes callback style](../read-files/README.md) where the first param might be an error or if you think about Promises you probably already know about the [`catch()` handler](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/catch). Rust has different styles of handling errors as well and I want to introduce you to some of them.

Before we jump into Rust we should test our program:

```bash
$ npm -s start
Wrote file in hello-world.txt with content: Hello world!
```

Charming.

## Rust

Our simplified _"read files"_ example looked like this:

```rust
use std::error::Error;
use std::fs::read_to_string;

fn main() {
    match read_to_string("hello.txt") {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(data) => println!("Content is: {}", data),
    };
}
```

Let us read and print the content of both files now:

```diff
use std::error::Error;
use std::fs::read_to_string;

fn main() {
-    match read_to_string("hello.txt") {
+    let hello = match read_to_string("hello.txt") {
        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(data) => println!("Content is: {}", data),
+        Ok(data) => data,
    };

+    let world = match read_to_string("world.txt") {
+        Err(err) => panic!("Couldn't read: {}", err.description()),
+        Ok(data) => data,
+    };

+    println!("Content is: {} and {}", hello, world);
}
```

You shouldn't see anything new here (- besides using two values in `println!()` maybe). Let us remove the duplicated error handling logic now by creating a new function:

```diff
use std::error::Error;
use std::fs::read_to_string;

+fn read_file(path: &str) -> String {
+   let data = match read_to_string(path) {
+        Err(err) => panic!("Couldn't read: {}", err.description()),
+        Ok(data) => data,
+   };
+   return data;
+}

fn main() {
-    let hello = match read_to_string("hello.txt") {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(data) => data,
-   };
-
-    let world = match read_to_string("world.txt") {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(data) => data,
-    };
+    let hello = read_file("hello.txt");
+    let world = read_file("world.txt");
    println!("Content is: {} and {}", hello, world);
}
```

As you can see `read_file` accepts a param called `path` which is a `&str` and it will return (`->`) a value from the type `String`. In contrast to TypeScript we _always_ need to specify the return type.

The remaining code should be relative self-explanatory. Only returning the actual value is new, but it looks just like JavaScript and uses the `return` keyword: `return data;`.

_But_... Rust allows us to write our code like this, too:

```diff
fn read_file(path: &str) -> String {
    let data = match read_to_string(path) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(data) => data,
    };
-    return data;
+    data
}
```

What is that? `data` is an _implicit return_ while `return data;` is an _explicit return_. This is possible, because Rust is primarily an _expression-based_ language. In that case `data` is an _expression_ while `return data;` is a _statement_. An expression automatically returns a value and a statement does not. The `;` basically turns our expression into a statement. Therefore the `;` is _not_ meaningless as it is 99% of the time in JavaScript. It is really a little bit similar to arrow functions in JavaScript which can return values automatically (e.g. `[ 1 ].map(n => n + 1) // [ 2 ]`). Using expressions like that allows nice things. For example Rust has _no_ ternary operator (e.g. `let foo = x ? y : z;` in JavaScript), because we can do the same with our normal `if`/`else` keywords and expressions (e.g. `let foo = if x { y } else { z };` in Rust).

I definitely need to get used to writing _implicit returns_ like that even if I use arrow functions very often in JavaScript. The single `data` on the last line looks a little bit weird to me. But to be honest... writing _implicit returns_ is the [prefered style](https://users.rust-lang.org/t/implicit-or-explicit-return-in-non-closures-what-is-more-idiomatic/6357/4) in that case. It is recommended to use the `return` keyword only for _early returns_.

In fact we can simplify the code even further:

```diff
fn read_file(path: &str) -> String {
-    let data = match read_to_string(path) {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(data) => data,
-    };
-    data
+    match read_to_string(path) {
+        Err(err) => panic!("Couldn't read: {}", err.description()),
+        Ok(data) => data,
+   }
}
```

We should check that everything works:

```bash
$ cargo -q run
Content is: Hello and world
```

Fine. Now we can look into different ways of handling errors in Rust before we'll add the actual part of writing files.

The patterns we used for now was using `panic!` which allows adding a custom error message and exits our program. This is okay, but a little bit verbose. The shortest way for error handling is calling `unwrap()` on functions returning a `Result`. (Remember from the [last example](../read-files/README.md) that the `Result` type has an `Err` case and an `Ok` case which we both _need_ to handle. This can be done with pattern matching like we did for now.) `unwrap` _just_ exits the program on `Err` or returns the result on `Ok`, but you cannot pass a custom error message. It looks like that.

```diff
fn read_file(path: &str) -> String {
-    match read_to_string(path) {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
-        Ok(data) => data,
-   }
+    read_to_string(path).unwrap()
}
```

As you can see our example becomes very readable. We could also remove `use std::error::Error;` now. That's why you can see this style very often in tutorials. But you rarely use `unwrap` in real code, because it lacks a custom error message.

You can achieve that with `expect` which works like `unwrap`, but accepts custom error messages.

```diff
fn read_file(path: &str) -> String {
-    read_to_string(path).unwrap()
+    read_to_string(path).expect("Couldn't read.")
}
```

A mix of both styles could look like this:

```diff
fn read_file(path: &str) -> String {
-    read_to_string(path).expect("Couldn't read.")
+    read_to_string(path).unwrap_or_else(|err| panic!("Couldn't read: {}", err.description()))
}
```

The `|err| panic!()` you see here is a [closure](https://doc.rust-lang.org/book/ch13-01-closures.html). You can think of them like arrow functions in JavaScript: `(err) => console.log()`. (Like JavaScript with `(err) => { console.log() }` you can use `{}`, if you need multiple lines, but they are optional for one-liners: `|err| { panic!() }`.)

Nice.

To showcase the next error handling pattern we will use the more verbose example from [the previous chapter](../read-files/README.md). I just extract it into our `read_file` function. The complete example would like this:

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.description()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => (),
    };

    match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value.to_string(),
    }
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("world.txt");
    println!("Content is: {} and {}", hello, world);
}
```

The only new part is the `to_string()` in the last line of the `read_file` function which convers our `&str` into a `String`.

Now assume we write a special `fs` crate which offers a similar API as Node to ease the transition for Node developers to use Rust. `read_file` shouldn't exit the program on errors. It is just a lib - the program needs to decide whether it should exit or not. How do we do that? We need to return an `Result` on our own. The `Result` will return a `String` in the `Ok` case and an error in the `Err` case.

The problem is the type of our error. `File::open` and the other file related methods can return a `std::io::Error` struct in an `Err` case. (Don't confuse `std::io::Error`, which is a struct for I/O related errors with our trait `std::error::Error` which was used so far.) But `from_utf8` can actually return a `std::str::Utf8Error`. Without `std::str::Utf8Error` our function could look like this:

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    // our code
}
```

But we need to be able to return _both_ errors. Sadly we can't write `Result<String, std::io::Error | std::str::Utf8Error>` like we can in TypeScript which would read as _"the `Err` case is either `std::io::Error` or `std::str::Utf8Error`"_. There are multiple ways to solve this:

1. Create a custom error type which takes care of this. But this can [be very verbose](https://twitter.com/PipoPeperoni/status/1084700498308022272).
2. Use [a third party lib like `error-chain`](https://github.com/rust-lang-nursery/error-chain) to get rid of the verbosity.
3. Use `Box<std::error::Error>` a more generic type we can cast every error to. Sadly we won't be able to statically determine the error type anymore.

I have the feeling that `1.` and `2.` are a little bit out of scope for this tutorial. In a real world library you would probably go with `2.`. I _really_ hope that Rust will someday handle this scenario more easily without introducing third party libs.

For the sake of this tutorial we go with solution `3.`. So what is `Box`? [`Box` is a built in struct](https://doc.rust-lang.org/std/boxed/struct.Box.html). It is basically a way to store a pointer to some value and with `Box<std::io::Error>` we can basically store and return _any_ error.

This will change the return signature of `read_file` and because Rust forces us to handle all errors, the error handling will move up to our `main` function. On every error we'll use an _early return_ to return the `err` as our `Err` case. This is written as `Err(err.into())` - the `.into()` will take care of converting the error into a `Box`. Similar we don't just return `data`, but `Ok(data)`.

```diff
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

-fn read_file(path: &str) -> String {
+fn read_file(path: &str) -> Result<String, Box<Error>> {
    let mut file = match File::open(path) {
-        Err(err) => panic!("Couldn't open: {}", err.description()),
+        Err(err) => return Err(err.into()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
-        Err(err) => panic!("Couldn't get stat: {}", err.description()),
+        Err(err) => return Err(err.into()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
-        Err(err) => panic!("Couldn't read: {}", err.description()),
+        Err(err) => return Err(err.into()),
        Ok(_) => (),
    };

    match from_utf8(&buffer) {
-        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
-        Ok(value) => value.to_string(),
+        Err(err) => return Err(err.into()),
+        Ok(value) => Ok(value.to_string()),
    }
}

fn main() {
-    let hello = read_file("hello.txt");
-    let world = read_file("world.txt");
+    let hello = read_file("hello.txt").unwrap();
+    let world = read_file("world.txt").unwrap();
    println!("Content is: {} and {}", hello, world);
}
```

This is a very useful pattern, but also quite verbose. Thankfully Rust has the `?` operator which will unwrap the value or return an error. This will reduce our code dramatically.

```diff
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn read_file(path: &str) -> Result<String, Box<Error>> {
-    let mut file = match File::open(path) {
-        Err(err) => return Err(err.into()),
-        Ok(value) => value,
-    };
-
-    let stat = match file.metadata() {
-        Err(err) => return Err(err.into()),
-        Ok(value) => value,
-    };
-
-    let mut buffer = vec![0; stat.len() as usize];
-
-    match file.read(&mut buffer) {
-        Err(err) => return Err(err.into()),
-        Ok(_) => (),
-    };
-
-    match from_utf8(&buffer) {
-        Err(err) => return Err(err.into()),
-        Ok(value) => Ok(value.to_string()),
-    }
+    let mut file = File::open(path)?;
+    let stat = file.metadata()?;
+
+    let mut buffer = vec![0; stat.len() as usize];
+    file.read(&mut buffer)?;
+    let value = from_utf8(&buffer)?.to_string();
+
+    Ok(value)
}

fn main() {
    let hello = read_file("hello.txt").unwrap();
    let world = read_file("world.txt").unwrap();
    println!("Content is: {} and {}", hello, world);
}
```

Beautiful! Now you know the most common error handling patterns. We can now start to add our file writing logic. First we need to create `String` called `hello_world` which will represent our file content. We can create an empty `String` and push values into it (very much like an `[].push` in JavaScript).

```diff
fn main() {
    let hello = read_file("hello.txt").unwrap();
    let world = read_file("world.txt").unwrap();
-    println!("Content is: {} and {}", hello, world);

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
fn main() {
    let hello = read_file("hello.txt").expect("Couldn't read 'hello.txt'.");
    let world = read_file("world.txt").expect("Couldn't read 'world.txt'.");

-    let mut hello_world = String::new();
-    hello_world.push_str(&hello);
-    hello_world.push_str(" ");
-    hello_world.push_str(&world);
-    hello_world.push_str("!");
+    let hello_world = hello + " " + &world + "!";
    println!("Content is: {}", hello_world);
}
```

But... there is also a macro for that! `format!` which makes it more readable in my opinion - especially if your string would be more complex. You use it like `println!`, but it just returns a `String`.

```diff
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
use std::error::Error;
use std::fs::File;
-use std::io::Read;
+use std::io::{Read,Write};
use std::str::from_utf8;

fn read_file(path: &str) -> Result<String, Box<Error>> {
    let mut file = File::open(path)?;
    let stat = file.metadata()?;

    let mut buffer = vec![0; stat.len() as usize];
    file.read(&mut buffer)?;
    let value = from_utf8(&buffer)?.to_string();

    Ok(value)
}

+fn write_file(path: &str, data: &str) -> Result<(), Box<Error>> {
+    File::create(path)?.write_all(data.as_bytes())?;
+    Ok(())
+}

fn main() {
    let hello = read_file("hello.txt").unwrap();
    let world = read_file("world.txt").unwrap();

    let hello_world = format!("{} {}!", hello, world);
-    println!("Content is: {}", hello_world);
+    write_file("hello-world.txt", &hello_world).unwrap();
+    println!("Wrote file 'hello-world.txt' with content: {}", hello_world);
}

```

We create a new function with `fn` called `write_file`. `write_file` accepts two params `path` and `data` which are both from the type `&str`. It returns a `Result` which is either a `Box<Error>` for the `Err` case or "empty" (`()`) for the `Ok` case. The file is created with `File:create` (this _opens_ a file in write-only mode) which is available, because we added `Write` to `use std::io::{}`. We actually write our content into the file with `write_all` which accepts bytes (`&[u8]`). A `&str` can be converted into a `&[u8]` with the `as_bytes` method. After that we return our "empty" `Ok` case with `Ok(())`.

Let's try our program:

```bash
$ cargo -q run
Wrote file 'hello-world.txt' with content: Hello world!
```

Awesome! In this example you learned different error handling patterns, different ways to concatenate a string and how to write a file.

---

← [prev _"Read files"_](../read-files/README.md) | [next _"HTTP requests"_](../http-requests/README.md) →
