# Rust Web Book

## Systems Programming
* Operating Systems
* Device drivers of all kinds
* Filesystems
* Databases
* Code that runs in very cheap devices or devices that must be extremely reliable **
* Cryptography
* Media codecs (read/write audio, video, image files)
* Media processing (speech recognition, photo editing software)
* Memory management (garbage collector)
* Text rendering
* Implementing higher-level pl
* Networking
* Virtualization and Software containers **
* Scientific simulations
* Games

* RUST
    * free of undefined behavior
    * safe and pleasant to use
    * Cargo (like npm, RubyGems)

> Resource constrained programming!

```bash
# Bash
rustup update
rustup self uninstall
rustup doc
rustup component add rustfmt

rustc --version
rustc main.rs # compiles

cargo fmt # formats any cargo project
cargo fix
cargo build (--release) # release flag compiles with optimizations
cargo --version
cargo new "prj_name"
cargo run
cargo clean # cleans generated files!
cargo check # checks if can be compiled!
cargo test # runs test cases
# cargo can create vcs for you, def is git, configurable.
# src contains all the code, top level dir contains all non-relevant code: licenses, conf files, README...
```

## Conventions

* File
    * hello.rc
    * hello_world.rc

* &x borrows a reference to x, and that *r is the value that the reference r refers to

## Packages

* The crossbeam crate provides a number of valuable concurrency facilities, including a scoped thread facility.

## Code Samples
```rust
// variables, variables are immutable by default
let apples = 5;
let mut banana = 5; // mutable

let pixel = (10, 20)
pixel.0 // first element of tuple
pixel.0 as f64 // how type conversion is made!

fn gcd( n: u64, mut m: u64) {

    // if statements dont require parantheses
    if m < n {
        // let defines local variable
        let t = m;
    }
    n // returns n! "FALLS OFF THE END"
}

#[test] // -> attribute
fn test_gcd() {
    //...
}

// closures
HttpServer::new( || ...) //args between ||
```

