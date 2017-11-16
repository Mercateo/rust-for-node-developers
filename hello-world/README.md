# Hello world!

Ah, yes! A classic _"Hello world!"_ example. This one will be really quick, but I'll show you an important difference between Node and Rust.

Create a `hello-world.js` containing nothing more than this line:

```javascript
console.log('Hello world!');
```

And now run this file with Node:

```bash
$ node hello-word.js
Hello world!
```

_Yeah!_ You'll see `Hello world!` in your console.

Now we'll do the same for Rust. Create a `hello_world.rs` with the followind content:

```rust
fn main() {
    println!("Hello world!");
}
```

Rust actually needs a special entry point to execute code. This is our `main` function and as you can see a function in Rust is declared like in JavaScript, just with the `fn` keyword instead of `function`. It is important to call the function `main` or the Rust compiler will throw an error. In Rust we typically use 4 spaces to indent code inside a function. In JavaScript most projects I know (and [the Standard Style](https://github.com/feross/standard)) use 2 spaces. Rust also recommends `snake_case` naming style for directories and files while I think `kebab-case` is most common in JavaScript projects. `println` isn't a simple function call, but a macro - which is indicated by the `!`. For now think of a macro as some code which is transformed into other code at compile time. Last but not least you _need_ to wrap your string inside `"`, not `'`. In JavaScript there is no difference between `"` and `'` to create strings and [many](https://github.com/feross/standard) prefer to use `'`. In Rust a `"` creates a _string literal_ while `'` creates a _character literal_ which means it only accepts a single character. You could write `println!("H");` or `println!('H');` and both would compile, but `println!('Hello World!');` throws an error.

Now compile our code with the following command:

```bash
$ rustc hello-word.rs
```

You'll see... nothing on your console. Instead a new file called `hello-world` was created next to `hello-world.rs`. This is our compiled code. You can run it like this:

```bash
$ ./hello-world
Hello world!
```

Now you'll see `Hello world!` in your console. This shows a fundamental difference between JavaScript/Node and Rust. Rust needs to be compiled before our program can be executed. This extra step is not needed for JavaScript which makes the development cycle with JavaScript sometimes faster. However the compilation step catches a ton of bugs _before_ even executing your program. This can be _so_ useful that you probably often want to introduce something similar to JavaScript - like TypeScript. There is another big benefit: we can easily share our compiled `hello-world` program with other developers _without_ the need for them to have Rust installed. This is not possible with Node scripts. Everyone who wants to run our `hello-world.js` needs to have Node installed.

______

← [prev](../setup/README.md) | [next](../package-manager/README.md) →
