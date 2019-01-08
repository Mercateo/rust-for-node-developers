# Read files

## Node

We'll start creating more complex examples now. We aim to read a [file called `hello.txt`](node/hello.txt) which is located in our project root. The file contains the text `Hello world!` and we log this content to the console.

Before we start with our Node example we'll install type declarations for Nodes built-in modules. Only then TypeScript knows about modules like `fs` which we'll need to read files. The declarations are available in a package called `@types/node`):

```bash
$ npm install --save-dev @types/node
```

Great! With that out of the way we can create our example.

To read a file and log its content I probably would write a program like this:

```ts
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

You import the `readFile` function, pass the file path (`'hello.txt'`) when you call this function, optionally pass an encoding (`'utf8'`) and pass a callback which is called when the file was read _or_ if an error appeared. By convention the first param of this callback is the error object (`err`) - _if_ an error appeared - and the second param is the content of the file (`data`) - if _no_ error appeared. We check both of these cases with an `if`/`else` statement. Note that in the case of an error we log its `message`, a human readable error description. It is not available on every `err` object, but for all typical errors associated with file reading (like `ENOENT: no such file or directory, open 'hello.txt'`, if the file couldn't be found). I also call `process.exit(1);` to stop the execution of the program and mark the `process` as a failure. This is a good style to stop your program and useful if this program is used by other scripts or inside continuous integration. If no error happened we just log our content: `` console.log(`Content is: ${data}`); ``.

Anyway... let's rewrite this example so it will be easier to compare to our Rust program. Have a look [at the new code](node/src/index.ts):

```ts
import { openSync, readSync, fstatSync, Stats } from 'fs';

let fileDescriptor: number;
try {
  fileDescriptor = openSync('hello.txt', 'r');
} catch (err) {
  console.log(`Couldn't open: ${err.message}`);
  process.exit(1);
}

let stat: Stats;
try {
  stat = fstatSync(fileDescriptor);
} catch (err) {
  console.log(`Couldn't get stat: ${err.message}`);
  process.exit(1);
}

const buffer = Buffer.alloc(stat.size);

try {
  readSync(fileDescriptor, buffer, 0, stat.size, null);
} catch (err) {
  console.log(`Couldn't read: ${err.message}`);
  process.exit(1);
}

let data: string;
try {
  data = buffer.toString();
} catch (err) {
  console.log(`Couldn't convert buffer to string: ${err.message}`);
  process.exit(1);
}

console.log(`Content is: ${data}`);
```

The first thing you'll notice is the usage of some `*Sync` functions instead of our asynchronous style we used earlier. Currently Rust only offers synchronous APIs to read and write files in the standard library. While asynchronous APIs _will be_ standardized really soon there are currently no plans to add asynchronous APIs for file operations like reading or writing to the standard library as far as I know. The second thing you'll notice is that we need to _open_ our file now (with `openSync`), before we can _read_ the content (with `readSync`). This is a lot more low-level than our `readFile` function which abstracts this away. But as you know... low-level functions are more powerful in general, too. If you need to read the content of a file in multiple steps or in slices it is better to open a file _once_ and perform all the read steps you need instead of opening the file for every read operation. Note that `openSync` returns a [file descriptor](https://en.wikipedia.org/wiki/File_descriptor) which is a reference to our file. The flag `'r'` tells `openSync` that we just want to read the file later on. In the next step we call `fstatSync` and pass the file descriptor to get the actual `size` of our file. This is needed to initialize our `buffer` which will store our file data when we call `readSync` and to tell `readSync` how _much_ to read. (Remember... with `readSync` we could also just read slices of a file, but in this case we want to read the whole file. That's why we pass `0`, `stat.size` and `null` as the last params. Have [a look at the docs](https://nodejs.org/docs/latest-v10.x/api/fs.html#fs_fs_readsync_fd_buffer_offset_length_position) to learn more about `readSync`.) As the final step we convert our `buffer` to a string. Note that we wrap every `*Sync` call and `buffer.toString` in a `try/catch`. This is analogous to our `if (err) {} else {}` logic in the asynchronous style and mirrors the following Rust example.

Let us test the program now:

```bash
$ npm -s start
Content is: Hello world!
```

Sweet. Now to Rust.

## Rust

Let us have a look at the whole Rust program:

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
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
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value,
    };

    println!("Content is: {}", data);
}
```

I _think_ you can read the code and grasp what it does. It _should_ look very similar to our Node example. Does it work?

```bash
$ cargo -q run
Content is: Hello world!
```

Yes!

Let us step through all lines now. Even some of its imported modules are quite interesting!

```rust
use std::error::Error;
```

This is surprising... if you look into our example we actually never use anything like `Error`. Try to remove this line and run `$ cargo -q run`. You'll get this error:

```
error[E0599]: no method named `description` found for type `std::io::Error` in the current scope
 --> src/main.rs:7:53
  |
7 |         Err(err) => panic!("Couldn't open: {}", err.description()),
  |                                                     ^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use std::error::Error;
  |
```

Our `err` is an instance of a so called `struct`, which is a _data structure_. For now you can think of it like an object in JavaScript, but you'll learn more about `struct`'s later. By default our `err` has no method called `description`. It is only available when we add `use std::error::Error;`. Why?

If you read the error message again you'll see that _items from traits can only be used if the trait is in scope_. Okay. Whatever a _`trait`_ is, it looks like `std::error::Error` actually _is_ a `trait`. The compiler even recommends it for this use case. And if you `use` the `trait` it is added to the _current scope_.

So what is a trait? To quote [Rust by Example](http://rustbyexample.com/trait.html): _A `trait` is a collection of methods defined for an unknown type: `Self`._ A trait can specify method signatures (like an `interface` in other languages), but it can also provide fully implemented methods, so they become available for that type.

If I would need to compare this to something in the Node world, I probably would think of manipulating the `prototype`. Think about this example:

```ts
import './get-second-item';

const two = [1, 2, 3].getSecondItem();
console.log(two); // logs `2`
```

With `./get-second-item` being:

```ts
Array.prototype.getSecondItem = function getSecondItem() {
  return this[1];
};
```

This is _not_ the same as a Rust trait, but it helps _me_ to understand them. If I don't `import './get-second-item';` I can't call `getSecondItem`. If I don't `use std::error::Error;` I can't call `description`. But while manipulating the `prototype` is a bad practice in JavaScript using `traits` in Rust is very idiomatic. Thanks to features like _scoping_ we don't inherit the problems of manipulating the `prototype`.

```rust
use std::fs::File;
```

`std::fs` behaves much like `fs` in Node world. It contains core `struct`'s and `trait`'s for accessing the file system. In this case we only `use` `File` which is a `struct` and has the `open` method to open our file (similar to Nodes `openSync`) on its instances.

```rust
use std::io::Read;
```

`Read` is a `trait`, too. It allows us to call `read` on our `File` instance.

```rust
use std::str::from_utf8;
```

`std::str::from_utf8` is just a function which we need to convert our `buffer` (actually a slice of bytes) to a string slice (`&str`).

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

Wow! A lot to see here. We declare a variable with `let` called `file`. `file` is marked as `mut` which stands for mutability. You probably know the concept of mutability and immutability already - it _basically_ means that we can change the value of a variable (it is mutable) or not (it is immutable). _Every_ variable in Rust is _immutable by default_. When we read the `file` later on this will change the _reading position_ of `file` internally, so it needs to be `mut`.

Next is `match File::open("hello.txt") {}`. Let me say this first: Rust has _no_ `try`/`catch` keywords, because you can't `throw` an exception! The possibility of an error is expressed by _types_ instead. `File::open` actually returns the `Result<File>` type. The `Result` type represents either success (`Ok`) or failure (`Err`). For now you can think of it as a synchronous variant of a JavaScript `Promise` which either _fulfills_ (similar to `Ok`) or _rejects_ (similer to `Err`). The last part to understand this code snippet is the `match` keyword used for pattern matching. You can think of `match` as a super powerful `switch`/`case` (which isn't available in Rust at all, because it uses the more powerful `match`). What makes it so powerful? It enforces you to cover _every_ case. It is not possible to forget one. (If you're curious `match` has [even more features](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html).)

If `Result` is either `Ok` or `Err`, you need to handle both cases. So we cover both cases. Every case can give us a variable. `Err` can contain an `err` and `Ok` can contain the actual result (`value`). In the case of `Ok` we just _return_ `value` so it saved in `file`. (Yes, we can return values in pattern matching and save them directly to a variable. You don't see a `return` keyword here, so think of `Ok(value) => value` as an automatically invoked `(value) => value` in JavaScript for now.) In the case of `Err` we call `panic!`. Remember that `!` marks a macro - which I introduced as _some code which is transformed into other code at compile time_ earlier. This explanation should still be enough to understand macros for now. `panic!` will log our error message and exit the program. So `panic!("Couldn't open: {}", err.description())` really works very much like `` console.log(`Couldn't get stat: ${err.message}`); process.exit(1); `` in our Node example.

Now move on to the next piece of code:

```rust
let stat = match file.metadata() {
    Err(err) => panic!("Couldn't get stat: {}", err.description()),
    Ok(value) => value,
};
```

Nothing completely new here. `file.metadata` returns a `Result` type like `File::open` so we use pattern matching again. If `file.metadata` is succesful we get metadata for our file very much like `fstatSync(file)` in JavaScript. `stat` is not marked as `mut`, because we don't change its values and just read them. (`stat.len()` will give us the size of our file like `stat.size` in our JavaScript example.)

```rust
let mut buffer = vec![0; stat.len() as usize];
```

Here we create a `Vec` (pronounced as _vector_) called `buffer`. `vec!` is a macro to create a `Vec` more easily with an `array`-like syntax. An alternative would be to use `Vec::new()`. I say `array`-like, because Rust actually has `array`'s, too, and they look a lot like JavaScripts arrays (e.g. `[1, 2]`). However they don't behave like JavaScripts arrays. A JavaScript array is much more similar to Rusts `Vec`. `Vec` and `array` can be compared to `String` and `&str` in this regard. A `Vec` and a `String` can have a dynamic size and behave similar to JavaScripts arrays and strings while Rusts `array` and `&str` have a fixed size.

So we create a `Vec` and use a _repeat expression_ by using `;` in `vec![x; N]`. That means our `Vec` is filled with `0`'s `stat.len()` times. (You can also create `array`'s or vectors with a `,` like `[1, 2, 3]` as you would in JavaScript.) We need to cast `stat.len()` (which is the type `u64`) to `usize`, because `N` needs to be `usize`. This is done with the `as` keyword and really works just [like TypeScript](https://www.typescriptlang.org/docs/handbook/basic-types.html#type-assertions).

Finally our `buffer` is flagged as `mut`, because it will change its values when we read our file. This is done in the next step:

```rust
match file.read(&mut buffer) {
    Err(err) => panic!("Couldn't read: {}", err.description()),
    Ok(_) => (),
};
```

We pass `buffer` to `file.read` with `&mut`. That means that `buffer` is passed to `file.read` as a _mutable reference_ (the `&` marks a reference). This is needed to _allow_ `file.read` to change `buffer`. (It is not enough to flag `buffer` as `mut` in general, we need to allow this to other functions or method in every case, where it is intended.) _Allowing_ this is actually a core feature of Rust called [_ownership_](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html). `file.read` _borrows_ `buffer` for as long as `file.read` _runs_. If it quits our `main` function becomes the owner of `buffer` again. Doing so ensures that only _one_ function is the owner of a piece of memory at a time and prevents data races. This makes Rust so safe.

`file.read` has no return value which we are interested in, so we do nothing in the `Ok` case: just `Ok(_) => ()`. You can think of this as a [_noop_](https://en.wikipedia.org/wiki/NOP): the `_` in `Ok(_)` is just a placeholder and the last `()` is the so called _unit type_, which is used to mark a meaningless value. (Every function which doesn't return a meaningfull value implicitely returns `()`, just like JavaScript functions return `undefined` by default.)

Now the last snippet:

```rust
let data = match from_utf8(&buffer) {
    Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
    Ok(value) => value,
};

println!("Content is: {}", data);
```

Nothing fancy here. We just convert our `buffer` to a `&str` with `from_utf8`. Note that we pass `&buffer` to `from_utf8` which means that `from_utf8` gets a _reference_ (because we used `&`) of `buffer`. So `&` is a _reference_ to a resource and `&mut` is a _mutable reference_ to a resource. `from_utf8` doesn't need to change `buffer`'s values so the reference doesn't need to be mutable.

At the end we just print out our file content.

Nice. I hope you could follow the example. Are we done? Well... we could be done. But as we moved our Node example from a higher level `readFile` function to some lower level functions to make it a little bit easier to compare to Rust, we could do the opposite now and use some more higher level function in Rust as well. This is possible, because we read our complete file at once instead of in several steps.

This is our simplified example:

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

The example should be quite explanatory. You just pass the path to a file to `read_to_string` and it returns a `Result`. In the `Ok` case we print out the content of the file.

We could simplify our example even more with less verbose error handling as you'll see in [the next chapter](../write-files/README.md).

---

← [prev _"Package Manager"_](../package-manager/README.md) | [next _"Write files"_](../write-files/README.md) →
