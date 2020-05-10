# Package Manager

## Meta Data

Node and Rust are both installed together with a package manager. 

* Node's package manager is called _npm_, its packages are called _node modules_ and its official website is [npmjs.com](https://www.npmjs.com/). 
* Rust's package manager is called _Cargo_, its packages are called _crates_ and its official website is [crates.io](https://crates.io/).

![official npm website](./npm-site.png)
![official Cargo website](./cargo-site.png)

If you followed my [Setup](../setup/README.md) and you use Node v10.14.2 your npm version is probably 6.4.1. You can check this by running the following command:

```bash
$ npm --version
6.4.1
```

If you want to update npm you can run this:

```bash
$ npm install -g npm
```

Let us check our installed Cargo version. I have 1.31.0:

```bash
$ cargo --version
cargo 1.31.0 (339d9f9c8 2018-11-16)
```

As you can see it prints the same version as our installed Rust compiler. It's best practice to update both tools in tandem with `rustup`:

```bash
$ rustup update
```

The _manifest file_ - the file which contains meta-data of your project like its name, its version, its dependencies and so on - is called `package.json` in the Node world and `Cargo.toml` in Rust. We'll now add manifest files to our [_"Hello World"_ examples](../hello-world/README.md), we created earlier.

Lets have a look at a typical `package.json` without dependencies:

```json
{
  "name": "hello-world",
  "version": "0.1.0",
  "author": "John Doe <john.doe@email.com> (https://github.com/john.doe)",
  "contributors": [
    "Jane Doe <jane.doe@email.com> (https://github.com/jane.doe)"
  ],
  "private": true,
  "description": "This is just a demo.",
  "license": "MIT OR Apache-2.0",
  "keywords": ["demo", "test"],
  "homepage": "https://github.com/john.doe/hello-world",
  "repository": {
    "type": "git",
    "url": "https://github.com/john.doe/hello-world"
  },
  "bugs": "https://github.com/john.doe/hello-world/issues"
}
```

The `Cargo.toml` looks really similar (besides being a `.toml` and not `.json`):

```toml
[package]
name = "hello-world"
version = "0.1.0"
authors = ["John Doe <john.doe@email.com>",
           "Jane Doe <jane.doe@email.com>"]
publish = false
description = "This is just a demo."
license = "MIT OR Apache-2.0"
keywords = ["demo", "test"]
homepage = "https://github.com/john.doe/hello-world"
repository = "https://github.com/john.doe/hello-world"
documentation = "https://github.com/john.doe/hello-world"
```

So what have we here? Both manifest formats offer `name` and `version` fields which are **mandatory**. Adding the authors of a project is slightly different between the modules, but optional for both. npm assumes a main `author` for every package and multiple `contributors` while in Cargo you just fill an `authors` array. The `authors` field is actually **mandatory** for Cargo. As a value you use a string with the pattern `name <email> (url)` in npm and `name <email>` in Cargo. (Maybe `(url)` will be added in the future, but [currently it is not used](https://github.com/rust-lang/cargo/issues/2736) by anyone in Cargo.) Note that `<email>` and `(url)` are optional and that `name` doesn't have to be a person. You can just use you company name as well or something like `my cool team`.

If you don't accidentally want to publish a module to a public repository you can do that with either `"private": true` in npm or `publish = false` in Cargo. You can optionally add a `description` field to describe your project. (While you technically could use [MarkDown](https://commonmark.org/) in your descriptions, the support is spotty in both ecosystems and it isn't rendered properly most of the time.)

To add a single license you write `"license": "MIT"` in npm and `license = "MIT"` in Cargo. In both cases the value needs to be an [SPDX license identifier](https://spdx.org/licenses/). If you use multiple licences you can use an [SPDX license expression](https://spdx.org/spdx-specification-21-web-version#h.jxpfx0ykyb60) like `"license": "MIT OR Apache-2.0"` for npm or `license = "MIT OR Apache-2.0"` for Cargo.

You can also optionally add multiple `keywords`, so your package can be found more easily in the official repositories.

You can add a link to your `homepage` and `repository` in both files (with a slightly different format for `repository`). npm allows you to add a link to reports `bugs` while Cargo allows you to add a link to find `documentation`.

## Build tool

Cargo can be used to build your Rust project and you can add custom build scripts to npm as well. (Remember that you don't _need_ a build step in the Node ecosystem, but if you rely on something like TypeScript it is needed. I'll show this in more in-depth when I introduce TypeScript to our Node projects.)

For now I just added a `main` and `scripts.start` field to our [`package.json`](meta-data/node/package.json):

```json
{
  // ...your previous code
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js"
  }
}
```

A `main` field points to your packages entry file. This is the file that will be loaded, if someone requires _your_ package. `scripts.start` is a convention to point to the file which should be loaded, if you want to _run_ your package by calling `$ npm start`:

```bash
$ npm start

> hello-world@0.1.0 start /Users/pipo/workspace/rust-for-node-developers/package-manager/meta-data/node
> node src

Hello world!
```

To ignore the npm output use `-s` (for `--silent`):

```bash
$ npm -s start
Hello world!
```

In this case the entry file to our package specified in `main` and the file which should be run if you call `$ npm start` point to the same file, but this doesn't have to be the case. Additionally you could specify [multiple executable files in a field called `bin`](https://docs.npmjs.com/files/package.json#bin).

Cargo on the other hand will look for a `src/main.rs` file to build and/or run and if it finds a `src/lib.rs` file, it will build a library which than can be required by a different crate.

You run your Rust project with Cargo like this:

```bash
$ cargo run
   Compiling hello-world v0.1.0 (/Users/pipo/workspace/rust-for-node-developers/package-manager/meta-data/rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/hello-world`
Hello world!
```

To ignore the Cargo output use `-q` (for `--quiet`):

```bash
$ cargo -q run
Hello world!
```

You'll see that Cargo created a new file in your directory: a `Cargo.lock`. (It also placed your compiled program in a `target` directory.) The `Cargo.lock` file basically works like a `package-lock.json` in the Node world (or a [`yarn.lock`](https://yarnpkg.com/lang/en/docs/yarn-lock/), if you use yarn instead of npm), but is also generated during builds. Just to be complete let us generate a `package-lock.json` as well:

```bash
$ npm install
npm notice created a lockfile as package-lock.json. You should commit this file.
up to date in 0.924s
found 0 vulnerabilities
```

This should be your `package-lock.json`:

```json
{
  "name": "hello-world",
  "version": "0.1.0",
  "lockfileVersion": 1
}
```

This should be your `Cargo.lock`:

```toml
[[package]]
name = "hello-world"
version = "0.1.0"
```

Both files become more interesting if you use dependencies in your project to ensure everyone uses the same dependencies (and dependencies of dependencies) at any time.

Before we move on let us make a slight adjustment to our [`Cargo.toml`](meta-data/rust/Cargo.toml) by adding the line `edition = "2018"`. This will add support for [_Rust 2018_](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html) to our package. _Editions_ are a feature which allow us to make backwards incompatible changes in Rust without introducing new major versions. You basically opt-in into new language features per package and your dependencies can be a mix of _different_ editions. Currently there are two different editions available: _Rust 2015_ (which is the default) and _Rust 2018_ (which is the newest).

## Publishing

Before we learn how to install and use dependencies we will actually publish a package that we can require afterwards. It will just export a `Hello world!` string. I call both packages `rfnd-hello-world` (with `rfnd` as an abbreviation for _"Rust for Node developers"_). npm offers namespacing of modules called [_scoped packages_](https://docs.npmjs.com/misc/scope). If I'd have used that feature our module could have looked like this: `@rfnd/hello-world`. Cargo doesn't support namespacing and this is an [intended limitation](https://internals.rust-lang.org/t/crates-io-package-policies/1041). By the way... even if `snake_case` is preferred for files and directories in Rust the module names in Cargo should use `kebab-case` [by convention](https://users.rust-lang.org/t/is-it-good-practice-to-call-crates-hello-world-hello-world-or-does-it-not-matter/6114). This is probably used most of time in npm world, too.

I'll introduce TypeScript for our Node module in this step. It isn't _that_ necessary currently, but it'll simplify some comparisons between Node and Rust in the next chapters when I use types or modern language features like [ES2015 modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import). First we need to install TypeScript as a `devDependency`, which is a dependency we only need for development, but not for our package itself at runtime:

```bash
$ npm install --save-dev typescript
```

To build the project we need to call the TypeScript compiler (`tsc`) by adding a new `build` field in the `scripts` object of the `package.json`. We also add a `prepublishOnly` entry which _always_ runs the build process before we'll publish our module:

```json
{
  "scripts": {
    "build": "tsc --build src",
    "prepublishOnly": "npm run build"
  }
}
```

By using `--build src` the TypeScript will look for a [`tsconfig.json` in the `src/` directory](package-manager/publishing/node/src/tsconfig.json) which configures the actual output. It looks like this:

```json
{
  "compilerOptions": {
    "module": "commonjs",
    "declaration": true,
    "outDir": "../dist",
    "sourceMap": true,
    "declarationMap": true
  }
}
```

Note that we'll generate CommonJS modules (because this is a Node project), it will generate `declaration` files (so other TypeScript projects know our types and interfaces even when they use the generated JavaScript files), all JavaScript and declaration files will be placed in a `dist` folder and finally we generate Source Maps to map the generated JavaScript back to the original TypeScript code (useful for debugging).

This also means that the `main` field in our [`package.json`](package-manager/publishing/node/package.json) now points to `dist/index.js` - our compiled JavaScript code. And we also add a `typings` field which shows other modules where our generated declarations are stored.

```json
{
  "main": "dist/index.js",
  "typings": "dist/index.d.ts"
}
```

Note that we don't want to commit our `node_modules` and `dist` to our repository, because these directories contain external or generated code. But be **warned**! If you place these directories in a `.gitignore` npm will _not_ include them in our published package. This okay for `node_modules` (which are _never_ included anyway), but a package without `dist` is pointless. You'll actually need to add an empty `.npmignore`, so npm ignores the `.gitignore`. (A little bit tricky, I know.) You can use the `.npmignore` to ignore files and directories which are committed in your repository, but shouldn't be included in your published package. In our case it'd be fine to include everything. As an alternative you could also explicitly list all files which should be included in a [`files` field](https://docs.npmjs.com/files/package.json#files) in your `package.json`.

With this setup aside this is our actual package under `index.ts`:

```ts
export const HELLO_WORLD = 'Hello world!';
```

We `export` a `const` with the value `'Hello world!'`. This is ES2015 module syntax and we write our exported variable name in `UPPER_CASES` which is a common convention for constants. Call `$ npm run build` to build your project.

This is how our generated `dist/index.js` looks:

```js
'use strict';
exports.__esModule = true;
exports.HELLO_WORLD = 'Hello world!';
//# sourceMappingURL=index.js.map
```

Nothing fancy. Basically the same code in a different module syntax. The second line tells other tools that it was originaly an ES2015 module. The last line links our file to the corresponding Source Map.

The generated declaration file `dist/index.d.ts` is also worth a look:

```ts
export declare const HELLO_WORLD = 'Hello world!';
//# sourceMappingURL=index.d.ts.map
```

You see that TypeScript could infer the type of `HELLO_WORLD` as a `'Hello world!'`. This is a _value type_ which is in this case a special variant of the type `string` with the concrete value `'Hello world!'`.

We didn't need to tell TypeScript the type explicitly, but we _could_ have done that. It would have looked like that:

```ts
export const HELLO_WORLD: 'Hello world!' = 'Hello world!';
```

Or like this, if we'd just want to tell others that it is a string:

```ts
export const HELLO_WORLD: string = 'Hello world!';
```

Great. This is our module. Now it needs to be published. You need to create an account at [npmjs.com](https://www.npmjs.com/signup). If you have done that you'll get a profile like [this](https://www.npmjs.com/~donaldpipowitch). Now call `$ npm login` and enter your credentials from your new account. After that you can just call `$ npm publish`;

```bash
$ npm publish

> rfnd-hello-world@1.0.1 prepublishOnly .
> npm run build


> rfnd-hello-world@1.0.1 build /Users/pipo/workspace/rust-for-node-developers/package-manager/publishing/node
> tsc --build src

# some output from npm notice...

+ rfnd-hello-world@1.0.1
```

Congratulations! 🎉 You successfully created a package which can be seen [here](https://www.npmjs.com/package/rfnd-hello-world).

Time for Rust! We first create a [`.gitignore`](publishing/rust/.gitignore) with the following content:

```
Cargo.lock
target
```

As pointed out earlier `Cargo.lock` behaves similar to `package-lock.json`, but while the `package-lock.json` can always be committed into your version control the `Cargo.lock` should only be committed [for binary projects, not libraries](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries). Npm ignores the `package-lock.json` in libraries, but Cargo doesn't do the same for `Cargo.lock`.

The `target` directory will contain generated code, so it is also ignored.

Actually this is all the setup we need. Now dive into our package (living in [`src/lib.rs`](publishing/rust/src/lib.rs), because this will be a library):

```rust
pub const HELLO_WORLD: &str = "Hello world!";
```

As you can see this line of code in Rust is really similar to our TypeScript code (when we excplicitly set the type to `string`) which looked like this:

```ts
export const HELLO_WORLD: string = 'Hello world!';
```

Let's go through the Rust line of code word for word:

* `pub` makes our variable _public_ - very much like `export` in JavaScript, so it can be used by other packages. 
* `const` in Rust is different than `const` in JavaScript though. In Rust this is a real constant - a value which can't be changed. 
    * In JavaScript it is a constant _binding_ which means we can't assign another value to the same name (in this case our variable name is `HELLO_WORLD`). But the value itself can be changed, if it is a non-[primitive](https://developer.mozilla.org/en-US/docs/Glossary/Primitive) value. (E.g. `const a = { b: 1 }; a.b = 2;` is possible.) 
* Unlike TypeScript we _need_ to declare the type of `HELLO_WORLD` here by adding `&str` or we'll get compiler errors. Rust also supports type inferring, but `const` _always_ requires an [explicit type annotation](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html#constants). 
    * `&str` is pronounced as _string slice_ (and [as a reminder](../hello-world/README.md) `"Hello world!"` is pronounced as a _string literal_). 
    * Rust actually has another String type called just `String`. A `&str` has a fixed size and cannot be mutated while a `String` is heap-allocated and has a dynamic size. A `&str` can be easily converted to a `String` with the `to_string` method like this: `"Hello world!".to_string();`. We'll see more of that in later examples, but you can already see methods can be invoked in the same way as we do in JavaScript and that built-in types come with a couple of built-in methods (like `'hello'.toUpperCase()` in JavaScript for example).

We only need to publish our new crate now. You need to login on [crates.io/](https://crates.io/) with your GitHub account to do so. If you've done that visit your [account settings](https://crates.io/me) to get your API key. Than call `cargo login` and pass your API key:

```bash
$ cargo login <api-key>
```

You can inspect what will be published by packaging your Crate locally like this:

```bash
$ cargo package
```

Much like npm Cargo ignores all your directories and files in your `.gitignore`, too. That is fine. We don't need to ignore more files (or less) in this case. (If you _do_ need to change that, you can modify your `Cargo.toml` [as explained in the documentation](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields-optional).)

Now we only need to publish the crate like this:

```bash
$ cargo publish
    Updating crates.io index
   Packaging rfnd-hello-world v1.0.1 (/Users/pipo/workspace/rust-for-node-developers/package-manager/publishing/rust)
   Verifying rfnd-hello-world v1.0.1 (/Users/pipo/workspace/rust-for-node-developers/package-manager/publishing/rust)
   Compiling rfnd-hello-world v1.0.1 (/Users/pipo/workspace/rust-for-node-developers/package-manager/publishing/rust/target/package/rfnd-hello-world-1.0.1)
    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
   Uploading rfnd-hello-world v1.0.1 (/Users/pipo/workspace/rust-for-node-developers/package-manager/publishing/rust)
```

Awesome! Your crate is now published and can be seen [here](https://crates.io/crates/rfnd-hello-world).

Remember that you can publish your package in the same version only _once_. This is true for Cargo and npm as well. To publish your package again with changes you need to change the version as well. The quickest way which doesn't introduce additional tooling is by just changing the value of `version` in `package.json` or `Cargo.toml` manually. Both communities follow [SemVer-style versioning](https://semver.org/) (more or less).

This is probably the minimum you need to know to get started in publishing your own packages, but I only scratched the surface. Have a look at the [npm documentation](https://docs.npmjs.com/) and [Cargo documentation](https://doc.rust-lang.org/cargo/index.html) to learn more.

Now that we published two packages we can try to require them in other projects as dependencies.

## Dependencies

Let us start with Node again to show you how using dependencies work. To be honest... we already used a dependency, right? TypeScript. We added it to the `devDependencies` and use it in every example now:

```bash
$ npm install --save-dev typescript
```

`devDependencies` are only needed when we develop our Node application, but not at runtime. We use our recently published package as a real `dependency`. Install it like this:

```bash
$ npm install --save rfnd-hello-world
```

You should see the following dependencies in your `package.json`:

```json
{
  "devDependencies": {
    "typescript": "^3.2.2"
  },
  "dependencies": {
    "rfnd-hello-world": "^1.0.1"
  }
}
```

We should also change our `start` script so it behaves similar to `$ cargo run` - build the project and run it:

```json
{
  "scripts": {
    "start": "npm run build && node dist",
    "build": "tsc --build src"
  }
}
```

The final `package.json` looks pretty much like our previous example, just with less meta data. I'll show it to you, so we are on the same page:

```json
{
  "name": "use-hello-world",
  "version": "0.1.0",
  "private": true,
  "main": "dist/index.js",
  "typings": "dist/index.d.ts",
  "scripts": {
    "start": "npm run build && node dist",
    "build": "tsc --build src"
  },
  "devDependencies": {
    "typescript": "^3.2.2"
  },
  "dependencies": {
    "rfnd-hello-world": "^1.0.1"
  }
}
```

The `tsconfig.json` is just copy and pasted without modifications.

We installed our dependencies, now we can use them like this:

```ts
import { HELLO_WORLD } from 'rfnd-hello-world';

console.log(`Required "${HELLO_WORLD}".`);
```

Let's run our example:

```bash
$ npm start

> use-hello-world@0.1.0 start /Users/pipo/workspace/rust-for-node-developers/package-manager/dependencies/node
> npm run build && node dist


> use-hello-world@0.1.0 build /Users/pipo/workspace/rust-for-node-developers/package-manager/dependencies/node
> tsc --build src

Required "Hello world!".
```

Good. Now we switch to Rust. We can't add dependencies to our project with Cargo without additional tooling. That's why we need to add it to our `Cargo.toml` manually in a section called `[dependencies]`. (You can watch [this issue](https://github.com/rust-lang/cargo/issues/5586) about adding a `$ cargo add <package-name>` command which will work similar to `$ npm install --save <package-name>`.)

```bash
[dependencies]
rfnd-hello-world = "1.0.1"
```

The crate will be _automatically_ fetched as soon as we compile our program. Note that using `1.0.1` actually translates to `^1.0.1`! If you want a very specific version you should use `=1.0.1`.

This is how our [`src/main.rs`](dependencies/rust/src/main.rs) looks like:

```rust
use rfnd_hello_world::HELLO_WORLD;

fn main() {
    println!("Required: {}.", HELLO_WORLD);
}
```

Note that even though our external crate is called `rfnd-hello-world` we access it with `rfnd_hello_world`. Aside from the import we do with the `use` keyword, you can see how the string interpolation works with the `println!()` macro where `{}` is a placeholder and we pass the value as the second parameter. (Printing to the terminal can be actually quite complex. Read [this article](https://doc.rust-lang.org/rust-by-example/hello/print.html) to learn more.) In case you didn't know: `console.log()` in Node can behave quite similar. We could rewrite `` console.log(`Required "${HELLO_WORLD}".`); `` to `console.log('Required "%s".', HELLO_WORLD);`. Try it!

As we use `HELLO_WORLD` just a single time we could also have written the example like this:

```rust
fn main() {
    println!("Required: {}.", rfnd_hello_world::HELLO_WORLD);
}
```

If `rfnd_hello_world` would expose more than one constant we can use a syntax similar to ES2015 destructing.

```rust
use rfnd_hello_world::{HELLO_WORLD, SOME_OTHER_VALUE};

fn main() {
    println!("Required: {}.", HELLO_WORLD);
    println!("Also: {}.", SOME_OTHER_VALUE);
}
```

Nice. Now test your programm:

```bash
$ cargo run
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling use-hello-world v0.1.0 (file:///Users/donaldpipowitch/Workspace/rust-for-node-developers/package-manager/dependencies/rust)
     Running `target/debug/use-hello-world`
Required "Hello world!".
```

It works! :tada:

To summarize: `use rfnd_hello_world::HELLO_WORLD;` (or `use rfnd_hello_world::{HELLO_WORLD};` for multiple imports) works similar to `import { HELLO_WORLD } from 'rfnd-hello-world';`, but we can also inline the "import" as `println!("Required: {}.", rfnd_hello_world::HELLO_WORLD);` which would be very similar to `` console.log(`Required "${require('rfnd-hello-world').HELLO_WORLD}".`); ``.

---

← [prev _"Hello World"_](../hello-world/README.md) | [next](../read-files/README.md) →
