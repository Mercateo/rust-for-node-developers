# Rust for Node developers

> An introduction to the Rust programming language for Node developers.

Hi there, [I'm a JavaScript developer](https://github.com/donaldpipowitch) who wants to learn Rust and as a part of this process I'll write here about my learnings. So what is Rust actually and **why should you learn it**? It is a systems programming language like C or C++, but with influences from functional programming languages and even scripting languages like JavaScript. It _feels_ very modern - which is no surprise, because it is a relatively young language. [It went 1.0 in 2015!](http://blog.rust-lang.org/2015/05/15/Rust-1.0.html) That doesn't only mean it is _fun to write_, because it has less clutter to carry around, it is also _fun to use_, because it has a modern toolchain with a great package manager. Rusts most unique feature is probably the compile-time safety check: it catches errors like segfaults without introducing a garbage collector. Or to phrase it differently: maximum safety with maximum performance.

Probably even more important than its features is the ecosystem and the community behind a language. Rust really shines here - especially for people who love the web. It is **baked by Mozilla** and the **biggest real world project** written in Rust is probably [servo](https://github.com/servo/servo), a modern browser engine. servo is actually very modular - e.g. you can require its [HTML parser as a standalone module](https://github.com/servo/html5ever). Can you do that with any other browser engine? As far as I know... _no_. [Chances are pretty high](http://blog.rust-lang.org/2016/05/13/rustup.html) that Rust will become a good host platform for [wasm](https://github.com/webassembly) - the future binary format for the web.

Before we dive into our setup we want to look at least once into a real Rust file:

```rust
fn main() {
    println!("Hello World!");
}
```

The JavaScript equivalent _could roughly_ look like this:

```javascript
function main() {
  console.log('Hello World!');
}
```

Nothing too scary, right? The `!` behind `println` could be a bit confusing, but don't mind it for now. Just think of it as a special function.

How do we move on from here? First I'll guide you how my current setup looks like to use Node and Rust. After that I'll create several kitchen sink like examples - first with Node and then with Rust. I'll try to explain them as best as I can, but _don't_ expect in-depth explanations in every case. Don't forget that I'm trying to learn Rust - _just like you_. Probably you need to explain _me_ some things, too! Oh, and before I forget it: my Node examples will be written in [TypeScript](https://www.typescriptlang.org/) actually! I think it makes some examples easier to compare to Rust and if you experience sweet code completion for Rust you want a little bit of that in your Node projects anyway ;)

# Table of contents

0. [Setup](setup/README.md)
0. [Hello World](hello-world/README.md)
0. [Package Manager](package-manager/README.md)

Thank you for reading this article. ♥