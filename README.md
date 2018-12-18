# Rust for Node developers

> An introduction to the Rust programming language for Node developers.

> 💡 **2nd edition.** I initially wrote this tutorial in the summer of 2016. Rust 1.0 was roughly a year old back than. This tutorial stayed quite popular over time even though I haven't added new chapters. As years passed by the Rust (and Node) ecosystem evolved further and this tutorial wasn't up-to-date with all changes. With the recent release of [_"Rust 2018"_](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html) (which I'll explain later in more depth) I took the opportunity to update this tutorial as well. Enjoy the read! 🎉

Hi there, I'm Donald. [I'm a JavaScript developer](https://github.com/donaldpipowitch) who wants to learn Rust and as a part of this process I'll write here about my learnings. But what is Rust and **why do I want learn it**? Rust is a systems programming language like C or C++, but with influences from functional programming languages and even scripting languages like JavaScript. It _feels_ very modern - which is no surprise, because it is a relatively young language. [It went 1.0 in 2015!](http://blog.rust-lang.org/2015/05/15/Rust-1.0.html) That doesn't only mean it is _fun to write_, because it has less clutter to carry around, it is also _fun to use_, because it has a modern toolchain with a great package manager. Rust's most unique feature is probably the compile-time safety check: it catches errors like segfaults without introducing a garbage collector. Or to phrase it differently: maximum safety with maximum performance.

Probably even more important than its features is the ecosystem and the community behind the language. Rust really shines here - especially for people who love the web. The language was (and still is) heavily influenced by developers from Mozilla. They have written [`servo`](https://github.com/servo/servo), a modern browser engine, in Rust and [parts of Firefox are now running Rust code](https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/). [Rust is also great for authoring WebAssembly code](https://www.rust-lang.org/what/wasm) (short: _Wasm_), a [binary format for the web](https://webassembly.org/), which is [supported in Firefox, Edge, Chrome and Safari](https://caniuse.com/#feat=wasm).

To summarize: Rust is a young modern language with a great tooling and ecosystem, good safety and performance promises and which can be used for a lot of different projects - from low level tasks to command line tools and even web projects.

Before we dive into our tutorial we want to look at least once into a real Rust file:

```rust
fn main() {
    println!("Hello World!");
}
```

The JavaScript equivalent _could roughly_ look like this:

```js
function main() {
  console.log('Hello World!');
}
```

Nothing too scary, right? The `!` behind `println` could be a bit confusing, but don't mind it for now. Just think of it as a special function.

How do we move on from here? First I'll guide you how my current setup looks like to use Node and Rust. Many people seemed to like that I introduce some convenient tooling and IDE configurations _before_ actually speaking about Rust itself. But you can skip this chapter, if you want. After the setup step I'll create several kitchen sink like examples - first with Node and then with Rust. I'll try to explain them as best as I can, but _don't_ expect in-depth explanations in every case. Don't forget that I'm trying to learn Rust - _just like you_. Probably you need to explain _me_ some things, too! And before I forget it: my Node examples will be written in [TypeScript](https://www.typescriptlang.org/)! Writing them in TypeScript will make it a little bit easier to compare some examples to Rust.

One word about the structure of this tutorial before we start. Every chapter has its own directory. If a chapter has sub-chapters they also have sub-directories. And if a (subd-)chapter contains code examples, you'll find a `node` and a `rust` directory which contain all the code of this (sub-)chapter. (One example: The chapter [_"Package Manager"_](package-manager/README.md) can be found inside [`package-manager/`](package-manager). It has the sub-chapter [_"Publishing"_](package-manager/README.md#publishing) and the corresponding code examples can be found in [`package-manager/publishing/node/`](package-manager/publishing/node) and [`package-manager/publishing/rust/`](package-manager/publishing/rust).)

# Table of contents

1. [Setup](setup/README.md)
2. [Hello World](hello-world/README.md)
3. [Package Manager](package-manager/README.md)
4. [Read files](read-files/README.md)
5. [Write files](write-files/README.md)
6. [HTTP requests](http-requests/README.md)
7. [Parse JSON](parse-json/README.md)

Thank you for reading this tutorial. ♥

I highly appreciate pull requests for grammar and spelling fixes as I'm not a native speaker. Thank you!
