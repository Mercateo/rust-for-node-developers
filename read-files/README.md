# Read files

## Node

We'll start creating more complex examples now. We aim to read a file called `hello.txt` which is located in our project root. The file contains the text `Hello world!` and we log this content to the console.

Before we start with our Node example we'll install type declarations for Nodes built-in modules. Only then TypeScript knows about modules like `fs` which we'll need to read files. You can install declarations for Node modules which aren't written in TypeScript with a package called [`typings`](https://github.com/typings/typings). Just install it with npm:

```bash
$ npm install -g typings
```

After that you can install the declarations for Node like this:

```bash
$ typings install --save --global dt~node
```

You'll see that a `typings.json` is generated as well as a `typings` directory containing the Node declarations. Oh yeah, and we modify our `start` script in our `package.json` so it behaves more like `$ cargo run`:

```json
{
  "scripts": {
    "start": "npm run build && node dist",
    "build": "tsc"
  }
}
```

If you run `$ npm start` now it will compile your program and run it, much like `$ cargo run`.

Great! With that out of the way we can create our example.

To read a file and log its content I probably would write a program like this:

```typescript
import { readFile } from 'fs';

readFile('hello.txt', 'utf8', (err, data) => {
  if (err) {
    console.log(`Couldn't read: ${err.message}`);
    process.exit(1);
  } else {
    console.log(`Content is: ${data}`);
  }
});
```

You import the `readFile` function, pass the file path (`'hello.txt`), optionally pass an encoding (`'utf8'`) and pass a callback which is called when the file was read _or_ if an error appeared. By convention the first param of this callback is the error object (`err`) - _if_ an error appeared - and the second param is the content of the file (`data`) - if _no_ error appeared. We check both of these cases with an `if`/`else` statement. Note that in the case of an error we log its `message`, a human readable error description. It is not available on every `err` object, but for all typical errors associated with file reading (like `ENOENT: no such file or directory, open 'hello.txt'`, if the file couldn't be found). I also call `process.exit(1);` to stop the execution of the program and mark the `process` as a failure. This is a good style to stop your program and useful if this program is used by other scripts or in continuous integration. If no error happened we just log our content: ``console.log(`Content is: ${data}`);``.

Anyway... let's rewrite this example so it is easier comparable to our Rust program. Have a look at this new program:

```typescript
import { openSync, readSync, fstatSync } from 'fs';

let file;
try {
  file = openSync('hello.txt', 'r');
} catch (err) {
  console.log(`Couldn't open: ${err.message}`);
  process.exit(1);
}

let stat;
try {
  stat = fstatSync(file);
} catch (err) {
  console.log(`Couldn't get stat: ${err.message}`);
  process.exit(1);
}

const buffer = new Buffer(stat.size);

try {
  readSync(file, buffer, 0, stat.size, null);
} catch (err) {
  console.log(`Couldn't read: ${err.message}`);
  process.exit(1);
}

let data;
try {
  data = buffer.toString();
} catch (err) {
  console.log(`Couldn't convert buffer to string: ${err.message}`);
  process.exit(1);
}

console.log(`Content is: ${data}`);
```

The first thing you'll notice is the usage of some `*Sync` functions instead of our asynchronous style we used earlier. Currently Rust only offers synchronous APIs to read and write files in the standard library. It is [still open to discussion](https://github.com/rust-lang/rfcs/issues/1081) if asynchronous APIs should be included in the standard library or not. The second thing you'll notice is that we need to _open_ our file now (with `openSync`), before we can _read_ the content (with `readSync`). This is a lot more low-level than our `readFile` function which abstracts this away. But as you know... low-level functions are more powerful in general, too. If you need to read the content of a file in multiple steps or in slices it is better to open a file _once_ and perform all the read steps you need instead of opening the file for every read operation. Note that `openSync` returns a [file descriptor](https://en.wikipedia.org/wiki/File_descriptor) which is a reference to our file. The flag `'r'` tells `openSync` that we just want to read the file later on. In the next step we call `fstatSync` and pass the file descriptor to get the actual `size` of our file. This is needed to initialize our `buffer` which will store our file data when we call `readSync` and to tell `readSync` how _much_ to read. (Remember... with `readSync` we could also just read slices of a file, but in this case we want to read the whole file. That's why we pass `0`, `stat.size` and `null` as the last params.) As the final step we convert our `buffer` to a string. Note that we wrap every `*Sync` call and `buffer.toString` in a `try/catch`. This is analogous to our `if (err) {} else {}` logic in the asynchronous style and mirrors the following Rust example.

Let us test the program now:

```bash
$ npm start -s
Content is: Hello world!
```

Sweet. Now to Rust.

## Rust

Let us have a look at the whole Rust program:

```rust
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::str::from_utf8;

fn main() {
    let mut file = match File::open("hello.txt") {
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
    }

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value,
    };

    print!("Content is: {}", data);
}
```

I _think_ you can read the code and grasp what it does. It _should_ look very similar to our Node example. Does it work?

```bash
$ cargo run -q
Content is: Hello world!
```

Yes!

Let us step through all lines now. Even some of its imported modules are quite interesting!

```rust
use std::fs::File;
```

`std::fs` behaves much like `fs` in Node world. It contains core APIs for accessing the file system. In this case we only `use` `File` which has the `open` method to open our file (similar to Nodes `openSync`).

```rust
use std::io::Read;
```

This is surprising... if you look into our example we actually never use anything like `Read`. Try to remove this line and run `$ cargo run -q`. You'll get this error:

```
src/main.rs:23:16: 23:20 error: no method named `read` found for type `std::fs::File` in the current scope
src/main.rs:23     match file.read(&mut buffer) {
                              ^~~~
src/main.rs:23:16: 23:20 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
src/main.rs:23:16: 23:20 help: candidate #1: `use std::io::Read`
```

So there is _no method named `read`_ on _type `std::fs::File`_. But why does it work when I use `use std::io::Read;`? Does it add a method named `read` to `std::fs::File`? Yeah, at least you can think of it like that.

If you read the error message again you'll see that _items from traits can only be used if the trait is in scope_. Okay. Whatever a _trait_ is, it looks like `std::io::Read` actually _is_ a trait. The compiler even says it the _candidate #1_ for this use case. And if you `use` the trait it is added to the _current scope_.

So what is a trait? To quote [Rust by Example](http://rustbyexample.com/trait.html): _A `trait` is a collection of methods defined for an unknown type: `Self`._ A trait can specify method signatures (like an `interface` in other languages), but it can also provide fully implemented methods.

If I would need to compare to something in the Node world, I probably would compare it to function binding probably best shown with the [experimental `::` operator](https://github.com/zenparsing/es-function-bind) ([not implemented in TypeScript yet](https://github.com/Microsoft/TypeScript/issues/3508)):

```typescript
import { someFoo } from 'cool-foos';
import { doBar } from 'foo-utils';

someFoo::doBar();
```

This calls `doBar` as if it would be a method from `someFoo`. This is nice so we don't need to add `doBar` to `someFoo`'s `prototype` (e.g. `SomeFoo.prototype.doBar`).

This is _not_ the same as a Rust trait, but it helps _me_ to understand them. If I don't `import { doBar } from 'foo-utils';` I can't call `someFoo::doBar();`. If I don't `use std::io::Read;` I can't call `file.read`.

Now to the last question for this line of code. If `std::io::Read` is a `trait`, what is `std::fs::File`? It is a `struct`! And a `struct` is like a data structure. For now this explanation should be enough. If we create own structs we'll learn more about them.

```rust
use std::error::Error;
use std::str;
```

`std::error::Error` is a trait, too. We need it to get human readable error messages (`err.description()`). `std::str::from_utf8` is just a function which we need to convert our `buffer` (actually a slice of bytes) to a string slice (`&str`).

Next follows our `main` function. The entry point of our program:

```rust
fn main() {
    // ...
}
```

The first thing we do in `main` is opening a file:

```rust
let mut file = match File::open("hello.txt") {
    Err(err) => panic!("Couldn't open: {}", err.description()),
    Ok(value) => value,
};
```

Wow! A lot to see here. We declare a variable with `let` called `file`. `file` is marked as `mut` which stands for mutability. If you worked with [React](https://facebook.github.io/react/) before you probably know the concept of mutability and immutability. _Every_ variable in Rust is _immutable by default_. When we read the `file` later on this will change the _reading position_ of `file` internally, so it needs to be `mut`.
Next is `match File::open("hello.txt") {}`. Let me say this first: Rust has _no_ `try`/`catch` keywords! The possibility of an error is expressed by _types_ instead. `File::open` actually returns the `Result<File>` type. The `Result` type represents either success (`Ok`) or failure (`Err`). For now you can think of it as a JavaScript `Promise` - if this helps you to understand `Result`. The last part to understand this code snippet is the `match` keyword used for pattern matching. You can think of `match` as a super powerful `switch`/`case` (which isn't available in Rust at all, because it uses the more powerful `match`). What makes it so powerful? It enforces you to cover _every_ case. It is not possible to forget one.

If `Result` is either `Ok` or `Err`, you need to handle both cases. So we cover both cases. Every case can give us a variable. `Err` can contain an `err` (in JavaScript we would say that this is our `err` object) and `Ok` can contain the actual result (`value`). In the case of `Ok` we just _return_ `value` so it saved in `file`. (Yes, we can return values in pattern matching and save them directly to a variable. You don't see a `return` keyword here, so think of `Ok(value) => value` as `(value) => value` in JavaScript for now.) In the case of `Err` we call `panic!`. Remember that `!` marks a macro - which I introduced as _some code which is transformed into other code at compile time_ earlier. This explanation should still be enough to understand macros for now. `panic!` will log our error message and exit the program. So `panic!("Couldn't open: {}", err.description())` really works very much like ``console.log(`Couldn't get stat: ${err.message}`); process.exit(1);`` in our Node example.

Now move on to the next piece of code:

```rust
let stat = match file.metadata() {
    Err(err) => panic!("Couldn't get stat: {}", err.description()),
    Ok(value) => value,
};
```

Nothing completely new here. `file.metadata` returns a `Result` type like `File::open` so we use pattern matching again. If `file.metadata` is succesful we get (_surprise_) metadata for our file very much like `fstatSync(file)` in JavaScript. `stat` is not marked as `mut`, because we don't change its values and just read them. (`stat.len()` will give us the size of our file like `stat.size` in our JavaScript example.)

```rust
let mut buffer = vec![0; stat.len() as usize];
```

Here we create a `Vec` (pronounced as _vector_) called `buffer`. `vec!` is a macro to create a `Vec` more easily with an `array`-like syntax. An alternative would be to use `Vec::new()`. I say `array`-like, because Rust actually has `array`'s, too, and they look a lot like JavaScripts arrays (e.g. `[1, 2]`). However they don't behave like JavaScripts arrays. A JavaScript array is much more similar to Rusts `Vec`. `Vec` and `array` can be compared to `String` and `&str` in this regard. A `Vec` and a `String` can have a dynamic size and behave similar to JavaScripts arrays and strings while Rusts `array` and `&str` have a fixed size.

So we create a `Vec` and use a _repeat expression_ to do so with using `;` in `vec![x; N]`. That means our `Vec` is filled with `0`'s `stat.len()` times. (You can also create `array`'s or vectors with a `,` like `[1, 2, 3]` as you would in JavaScript.) We need to cast `stat.len()` (which is the type `u64`) to `usize`, because `N` needs to be `usize`. This is done with the `as` keyword and really works just [like TypeScript](https://www.typescriptlang.org/docs/handbook/basic-types.html#type-assertions).

Finally our `buffer` is flagged as `mut`, because it will change its values when we read our file. This is done in the next step:

```rust
match file.read(&mut buffer) {
    Err(err) => panic!("Couldn't read: {}", err.description()),
    Ok(_) => (),
}
```

We pass `buffer` to `file.read` with `&mut`. That means that `buffer` is passed to `file.read` as a _mutable reference_. This is needed to _allow_ `file.read` to change `buffer`. (It is not enough to flag `buffer` as `mut` in general, we need to allow this to other functions or method in every case, where it is intended.) _Allowing_ this is actually a core feature of Rust called _ownership_. I mentioned that [earlier](../package-manager) and we'll see it in a lot of examples, because it is such an essential feature to Rust. `file.read` _borrows_ `buffer` for as long as `file.read` _runs_. If it quits our `main` function becomes the owner of `buffer` again. Doing so ensures that only _one_ function is the owner of a piece of memory at a time and prevents data races. This makes Rust so safe.

`file.read` has no return value which we are interested in, so we do nothing in the `Ok` case (just `Ok(_) => ()` which is kind of a [_noop_](https://en.wikipedia.org/wiki/NOP)).

Now the last snippet:

```rust
let data = match from_utf8(&buffer) {
    Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
    Ok(value) => value,
};

print!("Content is: {}", data);
```

Nothing fancy here. We just convert our `buffer` to a `&str` with `from_utf8`. Note that we pass `&buffer` to `from_utf8` which means that `from_utf8` gets a _reference_ (with `&`) of `buffer`. So `&` is a _reference_ to a resource and `&mut` is a _mutable reference_ to a resource. `from_utf8` doesn't need to change `buffer`'s values so the reference doesn't need to be mutable.

At the end we just print out our file content.

Nice. I hope you could follow the example. Are we done? Well... we could be done. But as we moved our Node example from a higher level `readFile` to some lower level functions, we can actually simplify our Rust example, too. This is of course possible, because we read our complete file at once instead of in several steps.

This is our simplified example:

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
        Ok(_) => print!("Content is: {}", data),
    }
}
```

The example should be self explanatory. We skip the part where we read the file content into a byte buffer and convert it to a string completely. Now we write the file content directly into a `String`.

Question: Why do we use `let mut data = String::new();` instead of `let mut data = "";`? Because a string slice (`&str`) is static and has a fixed size! We can only push new values (e.g. from reading a file) to `String`.

We could simplify our example even more with less verbose error handling, but I'll leave this up to a future example.

______

<!-- ← [prev](../setup) | [next](../?) → -->
← [prev](../package-manager)