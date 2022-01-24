# Hello world!

Ah, yes! A classic _"Hello world!"_ example. This one will be really quick, but I'll show you an important difference between Node and Rust.

Create a [`hello-world.js`](hello-world/node/hello-world.js) containing nothing more than this line:

```js
console.log('Hello world!');
```

And now run this file with Node:

```bash
$ node hello-word.js
Hello world!
```

_Yeah!_ You'll see `Hello world!` in your console.

Now we'll do the same for Rust. Create a [`hello_world.rs`](hello-world/rust/hello_world.rs) with the following content:

```rust
fn main() {
    println!("Hello world!");
}
```

Rust actually needs a special entry point to execute code. This is our `main` function and as you can see a function in Rust is declared like in JavaScript, but with the `fn` keyword instead of `function`. It is important to call the function `main` or the Rust compiler will throw an error. In Rust we typically use 4 spaces to indent code inside a function, while 2 spaces are more common for JavaScript projects. (But thankfully this is covered by prettier anyway and it could be covered by `rustfmt` when [my feature request will be solved](https://github.com/rust-lang/rls/issues/1198).) Rust also recommends `snake_case` naming style for directories and files while I think `kebab-case` is most common in JavaScript projects. `println` isn't a simple function call, but a macro - which is indicated by the `!`. For now think of a macro as some code which is transformed into other code at compile time. Last but not least you _need_ to wrap your string inside `"`, not `'`. In JavaScript there is no difference between `"` and `'` to create strings (and [many](https://github.com/feross/standard) prefer to use `'`, even though it's not prettiers default). In Rust a `"` creates a _string literal_ while `'` creates a _character literal_ which means it only accepts a _single character_. You could write `println!("H");` or `println!('H');` and both would compile, but `println!('Hello World!');` throws an error.

Now compile our code with the following command:

```bash
$ rustc hello_world.rs
```

You'll see... nothing on your console. Instead a new file called `hello_world` was created next to `hello_world.rs`. This file contains our compiled code. You can run it like this:

```bash
$ ./hello_world
Hello world!
```

Now you'll see `Hello world!` in your console. This shows a fundamental difference between JavaScript/Node and Rust. Rust needs to be compiled before our program can be executed. This extra step is not needed for JavaScript which makes the development cycle with JavaScript sometimes faster. However the compilation step catches a ton of bugs _before_ even executing your program. This can be _so_ useful that you probably want to introduce a similar sanity check to JavaScript - for example by using TypeScript. There is another big benefit: we can easily share our compiled `hello_world` program with other developers _without_ the need for them to have Rust installed. This is not possible with Node scripts. Everyone who wants to run our `hello-world.js` needs to have Node installed and in a version which is supported by our script.

In the next chapter I'll introduce you to package managing. I choose to talk about this topic, because it is the last topic related to "project configuration" before we can actually focus on small programming examples to learn the language itself.

---

← [prev _"Setup"_](../setup/README.md) | [next _"Package Manager"_](../package-manager/README.md) →
