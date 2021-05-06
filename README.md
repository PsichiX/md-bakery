# Markdown Bakery
### CLI tool to bake your fresh and hot MD files

## Install

You can install from crates.io using `cargo`:

```bash
cargo install md-bakery
```

> **Note:** The command used to run the program after installing is `mdbakery` **not** `md-bakery`.

## About
At least once in your Rust dev lifetime you wanted to make sure all code examples
in your markdown files are up-to-date, correct and code is formated, but you
couldn't make that done with already existing tools - fear not!

Markdown Bakery allows you to create markdown template files with rust code
injected from source files using special code block:

    # Content of `./src/main.rs`:

    ```rust: source
    ./src/main.rs
    ```

After language part you tell MD Bakery this code part has to inject source code from path specified inside the code block.

If you want to escape MD Bakery code blocks you can put `!` in front of `source` (sadly we can't show it here since that would print double `!` in the readme file you read right now, more about why later).

If you prefer to inject different parts of the code file instead of keeping each code snippet in separate files, all you have to do is to inject named source:

    # Snippet A:

    ```rust: source @ snippet-a
    ./src/main.rs
    ```

    # Snippet B:

    ```rust: source @ snippet-b
    ./src/main.rs
    ```

And then in your source file you specify named blocks of the code where snippets are located, using comments like this:

```rust
use std::collections::HashMap;

// [md-bakery: begin @ snippet-a]
#[derive(Debug)]
struct Foo {
    bar: HashMap<String, usize>,
}
// [md-bakery: end]

fn main() {
    // [md-bakery: begin @ snippet-b]
    let foo = Foo {
        bar: {
            let mut result = HashMap::new();
            result.insert("answer".to_owned(), 42);
            result
        },
    };

    println!("{:?}", foo);
    // [md-bakery: end]
}
```

And then all of that renders into:

    # Snippet A:

    ```rust
    #[derive(Debug)]
    struct Foo {
        bar: HashMap<String, usize>,
    }
    ```

    # Snippet B:

    ```rust
    let foo = Foo {
        bar: {
            let mut result = HashMap::new();
            result.insert("answer".to_owned(), 42);
            result
        },
    };
    
    println!("{:?}", foo);
    ```

**Of course this readme file was baked too, you can see template sources in `/examples` folder!**
So to see how to escape MD Bakery code blocks go there and look it up yourself :D

## Usage:
```bash
md-bakery 1.0.0
Patryk 'PsichiX' Budzynski <psichix@gmail.com>
Markdown Bakery CLI app

USAGE:
    mdbakery.exe [OPTIONS] --input <FILE> --output <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     Markdown template file name
    -o, --output <FILE>    Markdown generated file name
    -r, --root <FILE>      Source files root path
```

## TODO:
- [ ] Add `exec` code block variant that runs executable with parameters specified in block content, then put its stdout:

      ```bash: exec
      cargo run -- --help
      ```
