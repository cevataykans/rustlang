# Rust Systems Programming

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

# publishing crates on cargo.io
cargo package
cargo login "API_KEY"
caego publish

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

## Packages

* The crossbeam crate provides a number of valuable concurrency facilities, including a scoped thread facility.

## Rust Primitives
variables, variables are immutable by default
modern hardware implements them
num crate contains more adaqute representations like complex!
* isize
* usize (signed and unsigned ints, the same size as an addr on the machine)
* i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
* Rust uses u8 for bytes!
* 0x, 0o and 0b designate hexademical, octal and binary literals!
* 42u8 //u8 value, for marking literals with specific types
* 128i16 //i16
* 1729usize

BYTE Literal
ONLY ASCII CHARS CAN APPEAR IN BYTE LITERALS!
b'X' represents ASCII code for the character X as a u8 value
* b'A' == 65_u8

hex values can be used to write byte literals
* b'\xHH' // x -> indicates hex value, HH is the 0-F 
* b'\xFF' == 255_u8
* b'\x1b' == escape character, 27_u8

// std lib provides some ops as methods on integers. For example
```rust
assert_eq!(2_u16.pow(4), 16); // exponentiation
assert_eq!((-4_i32).abs(), 4); // absolute value
assert_eq!(0b101101_u8.count_ones(), 4); // population count
```

You can put _ for readability, place does not matter (std::i32)
* 0xffff_ffff
* 4_294_967_295
* 127_u8 // ***** niceu niceuuuu
* 0xcafeu32 // type: u32, decimal value: 51966

> Note that method calls have a higher precedence than unary prefix operators, so be
careful when applying methods to negated values. Without the parentheses around
-4_i32 in the first statement, -4_i32.abs() would apply the abs method to the posi‐
tive value 4, producing positive 4, and then negate that, producing -4.

```rust
println!("{}", (-4).abs()); // wont compile!
println!("{}", (-4_i32).abs());
println!("{}", i32::abs(-4));
```

Operation names that follow checked_, wrapping_, saturating_ or overflowing_:
* add
* sub
* mul
* div
* rem // remainder
* neg // negation
* abs
* pow
* shl // bitwise shift left
* shr

f32, f64
9.109_383_56e-31_f64 // wow
=> -1.5625 2. 0.25 1e4 40f32 9.109_383_56e-31_f64
* f32::INFINITY
* f32::NEG_INFINITY
* f32::NAN // (not a number)
* f32::MIN, f32::MAX

std::(f32 or f64)::consts // PI, E ..., square root of two

bool (true, false)

Rust's as operator can convert bool to int
Rust uses a byte to hold bools so that if you need it you can create a pointer asd point to it!
```rust
assert_eq!(false as i32, 0);
assert_eq!(true as i32, 1);
```

char => unicode, different than u8 nor u32, although 32 bit long!

YOU CAN WRITE ANY UNICODE CHAR AS '\u{HHHHHH}' where HHHHHH is a hexadecimal num up to six digits long!
* '\u{CA0}''_''\u{CA0}' -> “ಠ_ಠ”

You can use as op to convert char to int
u8 is the only type the as op will convery to char!

(char, u8, i32) // mixed types tuple
* tuple.0, tuple.1 -> access elements by only referencing them with indices!

() => void empty tuple, functions that have no return value has this!

```rust
let text = "I see the eigenvalue in thine eye";
let (head, tail) = text.split_at(21); // fn split_at(&self, mid: usize) -> (&str, &str);
assert_eq!(head, "I see the eigenvalue ");
assert_eq!(tail, "in thine eye");

fn swap<T>(x: &mut T, y: &mut T);
// is the same as:
fn swap<T>(x: &mut T, y: &mut T) -> ();
```

> Rust consistently permits an extra trailing comma every‐where commas are used: function arguments, arrays, struct and enum definitions, and so on. 

String UTF-8 str, dynmaically sized

&str Reference to str, non-owning pointer to UTF8 text

> At run time, a reference to an i32 is a single machine word holding the address of the i32, which may be on the stack or in the heap.

> The expression &x produces a reference to x; in Rust terminology, we say that it borrows a reference to x. Given a reference r, the expression *r refers to the value r points to.

**RUST REFERENCES ARE NEVER NULL**

* &T
    * an immutable, shared reference, can have many shared references, read only!
* &mut T 
    * A mutable, exclusive reference. Can read and modify the value it points to. You can only access the value using this mutable reference.
    * **Single write or multiple reads principle!**
* Box<Attend>
    * Box, owning pointer to value in heap -> Box::new(Late(14))
    * simplest way to allocate value on heap
```rust
let t = (12, "eggs");
let b = Box::new(t); // allocate a tuple on the heap
```
When b goes out of scope, memory is freed immediately, unless b has been moved!
* Raw Pointers
    * C++ like pointers, used within unsafe blocks

[f64; 8], [u8; 256]
* arr, fixed length, elements all of same type [1.0, 0.0, 0.0, 1.0]
Vec<f64> // vector, varying length, elements of all same type -> vec![0.367, 2.718, 7.389]
* you cannot append new elements or shrink an arr
* [V; N] -> V is the value, N is the count for initializing arrays at creation
```rust
let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
let taxonomy = ["Animalia", "arthropoda", "Insecta"];
assert_eq!(lazy_caterer[3], 7);
assert_eq!(taxonomy.len(), 3);

let mut sieve = [true; 10_000]; // an arr of 10,000 bool elements, al lset to true!
// [0u8; 1024] => 1 KB of buffer, filled with zero bytes
```

Vec\<T>, vector of Ts
* dynamically allocated
* live on heap -> push, delete, shrink, append, grow...
* consists three values:
    * ptr to the heap allocated buffer
    * the capacity
    * the actual num of elements
* Vec::with_capacity -> to create vectors with given length

```rust
let mut primes = vec![2, 3, 5, 7, 11];
// is equivalant to:
let mut pal = Vec::new();
pal.push("step");
pal.push("on");
pal.push("no");
pal.push("pets");
assert_eq!(pal, vec!["step", "on", "no", "pets"]);

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

let v: Vec<i32> = (0..5).collect(); // iterator way

let mut v = Vec::with_capacity(2);
assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 2);
v.push(1);
v.push(2);
assert_eq!(v.len(), 2);
assert_eq!(v.capacity(), 2);
v.push(3);
assert_eq!(v.len(), 3);
// Typically prints "capacity is now 4":
println!("capacity is now {}", v.capacity());

// Get our command-line arguments as a vector of Strings.
let languages: Vec<String> = std::env::args().skip(1).collect();
for l in languages {
    println!("{}: {}", l,
    if l.len() % 2 == 0 {
        "functional"
    } else {
        "imperative"
    });
}
```

&[u8], &mut [u8] => SLICES
* reference to slice: reference to a portion of an array or vector, comprising pointer and length -> &v[10..20], &mut a[..]
* mutable ptr allows you to read & modigy elements within the range
* the other one allows shared access! no write!
* **Fat pointer** contains => ptr to the slice's first element, and the number of elements in the slice
* Always passed by reference!

```rust
let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
let sv: &[f64] = &v;
let sa: &[f64] = &a;

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}
print(&a); // works on arrays
print(&v); // works on vectors

print(&v[0..2]); // print the first two elements of v
print(&a[2..]); // print elements of a starting with a[2]
print(&sv[1..3]); // print v[1] and v[2]
```

> Values representing the sizes of arrays or vectors or counts of the number of elements in some data structure also generally have the usize type.

```rust
// named-field struct -> S { x: 120.0, y: 209.0 }
struct S { 
    x: f32,
    y: f32
}

// Tuple like struct
struct T ( i32, char); // T(120, 'a')

// Unit like struct, no fields
struct E; // E
```

Strings
```rust
let speech = "\"Ouch!\" said the well.\n";

// includes the newline character so are the spaces in the second line!
println!("In the room the women come and go, 
    Singing of Mount Abora");

// new line and leadign whitespaces are dropped!
// the print below prints a single line!
println!("It was a bright, cold day in April, and \
    there were four of us—\
    more or less.");

// RAW STRING
let a = r##"hi 
asdadaf
" asd
\n asdkn 
" ada
asd"## // as many pound signs can be added to clearly mark where the string starts and ends

// BYTE STRING => slice of u8 values
// They cannot contain arbitrary Unicode chars
let method = b"GET";
assert_eq!(method, &[b'G', b'E', b'T']);
```

String in memory
* Vec<u8> // characters can be a single byte value or multiple bytes -> len returns the number of bytes!
* A string literal is a &str that refers to preallocated text, typically stored in read-only memory laong with the program's machine code!
* It is **impossible** to modify a &str
* For creating new strings at run time, use String.
* &str is very much like &[T]: a fat pointer to some data
* String is analogoues to Vec<T>
    * .to_string() converts &str -> String (copies string)
    * format!()
        * like println!(), except:
        * returns a new string instead of writing to Stdout
        * does not auto adds new line
    * Arrs, slices and vectors of strings have two methods:
        * .concat()
        * .join(sep)
* Support == and != and comparison: < <= > >=

```rust
let noodles = "noodles".to_string(); // Vec<u8> (String)
let oodles = &noodles[1..]; // slice to noodles, &str
let poodles = "ಠ_ಠ"; // slice ptr! readonly!, &str

assert_eq!("ಠ_ಠ".len(), 7);
assert_eq!("ಠ_ಠ".chars().count(), 3);

let mut s = "hello";
s[0] = 'c'; // error: `&str` cannot be modified, and other reasons
s.push('\n'); // error: no method named `push` found for reference `&str`

assert_eq!( format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());

let bits = vec!["veni", "vidi", "vici"];
assert_eq!(bits.concat(), "venividivici");
assert_eq!(bits.join(", "), "veni, vidi, vici");

assert_eq!("ONE".to_lowercase() == "one");
```

Other systems may not use UTF-8 and we need to deal with strings that are not valid unicodes:
* Stick to String and &str for Unicode text
* filenames: use std::path::PathBuf and &Path instead
* Binary data that is not UTF-8 encoded at all: Vec<u8> and &[u8]
* Env variable names and command-line args in the native form presented by the OS: use OsString and &OsStr
* Interoperating with C libraries that use null-terminated strings: std::ffi::CString and &CStr

TYPE
* type keyword can be used like typedef in C++:
```rust
type Bytes = Vec<u8>;
fn decode( data: &Bytes) {
    ...
}
```

Enumeration, algebraic data type -> Attend::Late(5), Attend::OnTime

enum Attend {
    OnTime,
    Late(u32)
}

&i32, &mut i32 // shared and mutable references: non owning pointers that must not outlive their referent &s.y, &mut v

Option<&str> // Optional value, either None or Some(v), present with value v -> Some("Dr."), None

Result<u64, Error> // REsult of op that may fail, either a success value of Ok(v) or an error Err(e) Ok(4096), Err(Error::last_os_error())

&dyn Any, &mut dyn Read // Trait object, reference to any value that implements a given set of methods -> value as &dyn Any, &mut file as &mut dyn Read

fn(&str) -> bool // pointer to function, str::is_empty

Closure types have no written form // closure
// -> |a, b| { a*a + b*b }

## Ownership & Moves

Some class owns some object that it points to
* This generally means that the owning object gets to decide when to free the owned object: when the owner is destroyed, it destroys its possessions along with it

* A variable owns its value. When control leaves the block in which the variable is
declared, the variable is dropped, so its value is dropped along with it

* A variable owns its value. When control leaves the block in which the variable is
declared, the variable is dropped, so its value is dropped along with it

* The standard library provides the reference-counted pointer types Rc and Arc,
which allow values to have multiple owners, under some restrictions

* Very simple types like integers, floating-point numbers, and characters are
excused from the ownership rules. These are called Copy types.

* You can move values from one owner to another. This allows you to build,
rearrange, and tear down the tree

```rust
let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
let t = s; // s is now considered uninitialized
let u = s; // compile error!

let mut s = "Govinda".to_string();
s = "Siddhartha".to_string(); // value "Govinda" dropped here

let mut s = "Govinda".to_string();
let t = s;
s = "Siddhartha".to_string(); // nothing is dropped here

let x = vec![10, 20, 30];
if c {
    f(x); // ... ok to move from x here
} else {
    g(x); // ... and ok to also move from x here
} 
h(x); // bad: x is uninitialized here if either path uses it

let x = vec![10, 20, 30];
while f() {
    g(x);   // bad: x would be moved in first iteration,
            // uninitialized in second
}

let mut x = vec![10, 20, 30];
while f() {
    g(x);       // move from x
    x = h();    // give x a fresh value
} 
e(x);


// possible ops to move an element from vec to a variable
// Build a vector of the strings "101", "102", ... "105"
let mut v = Vec::new();
for i in 101 .. 106 {
    v.push(i.to_string());
}
// 1. Pop a value off the end of the vector:
let fifth = v.pop().expect("vector empty!");
assert_eq!(fifth, "105");
// 2. Move a value out of a given index in the vector,
// and move the last element into its spot:
let second = v.swap_remove(1);
assert_eq!(second, "102");
// 3. Swap in another value for the one we're taking out:
let third = std::mem::replace(&mut v[2], "substitute".to_string());
assert_eq!(third, "103");
// Let's see what's left of our vector.
assert_eq!(v, vec!["101", "104", "substitute"]);


let v = vec!["liberté".to_string(),
    "égalité".to_string(),
    "fraternité".to_string()];
// consumes all elements in the loop. v is uninitialized!
for mut s in v {
    s.push('!');
    println!("{}", s);
}
```

* Copy Types
    * all the machine integer and floating-point numeric types
    * the char and bool types
    * A tuple or fixed-sized arr of Copy types is itself a copy type

* As a rule of thumb, any type that needs to do something special when a value is drop‐
ped cannot be Copy

* What about types you define yourself? **By default, struct and enum types are not Copy**

```rust
// womt compile
struct Label { number: u32 }
fn print(l: Label) { 
    println!("STAMP: {}", l.number); 
}
let l = Label { number: 3 };
print(l);
println!("My label number is: {}", l.number);

/*
If all the fields of your
struct are themselves Copy, then you can make the type Copy as well by placing the
attribute #[derive(Copy, Clone)]
*/
#[derive(Copy, Clone)]
struct Label { number: u32 }
```
> In Rust, every move is a byte-for-byte, shallow copy that leaves the source uninitialized. Copies are the same, except that the source remains initialized. 

Arc -> Atomic reference count (safe to share between threads)

Rc -> Reference count

Python, Java like ptr assignment:
```rust
use std::rc::Rc;
// Rust can infer all these types; written out for clarity
let s: Rc<String> = Rc::new("shirataki".to_string());
let t: Rc<String> = s.clone();
let u: Rc<String> = s.clone();
```

* Rust assumes the referent of an Rc pointer might in general be shared, so it must not be mutable. 

* std::rc::Weak

* **PASS BY VALUE**
    * when ownership changes
* **PASS BY REFERENCE**
    * When a reference is given

> in Rust you use the & and * operators to create and follow references, with the exception of the . operator, which borrows and dereferences implicitly.

```rust
let mut x = 10;
let r1 = &x;
let r2 = &x; // ok: multiple shared borrows permitted
x += 10; // error: cannot assign to `x` because it is borrowed
let m = &mut x; // error: cannot borrow `x` as mutable because it is
// also borrowed as immutable
println!("{}, {}, {}", r1, r2, m); // the references are used here,
// so their lifetimes must last
// at least this long

let mut y = 20;
let m1 = &mut y;
let m2 = &mut y; // error: cannot borrow as mutable more than once
let z = y; // error: cannot use `y` because it was mutably borrowed
println!("{}, {}, {}", m1, m2, z); // references are used here

let mut w = (107, 109);
let r = &w;
let r0 = &r.0; // ok: reborrowing shared as shared
let m1 = &mut r.1; // error: can't reborrow shared as mutable
println!("{}", r0); // r0 gets used here

let mut v = (136, 139);
let m = &mut v;
let m0 = &mut m.0; // ok: reborrowing mutable from mutable
*m0 = 137;
let r1 = &m.1; // ok: reborrowing shared from mutable,
// and doesn't overlap with m0
v.1; // error: access through other paths still forbidden
println!("{}", r1); // r1 gets used here
```

## Expressions

**Rust is what is called an expression language.**

Declaration => let name: type = expr;
* Type and initializer are optional
* Semicolon required 

```rust
// Dont use
for line in file.lines() {
    let line = line?;
    ...
}

// Use as convention!
for line_resilt in file.lines() {
    let line = line_result?;
    ...
}

if condition1 {
    block1
} else if condition2 {
    block2
} else {
    block_n
}

match value {
    pattern => expr,
    ...
}
// Can be optimized if constants are returned and no branching is even done!
// Jump tables are used for optimization
// Variety of patterns can be used on the left side
match code {
    0 => println!("OK"),
    1 => println!("Wires Tangled"),
    2 => println!("User Asleep"),
    _ => println!("Unrecognized Error {}", code) // matches alles
}

if let pattern = expr {
    block1
} else {
    block2
}

if let Some(cookie) = request.session_cookie {
    return restore_session(cookie);
}
if let Err(err) = show_cheesy_anti_robot_task() {
    log_robot_attempt(err);
    politely_accuse_user_of_being_a_robot();
} else {
    session.mark_as_human();
}

//Equivalent of match
match expr {
    pattern => { block1 }
    _ => { block2 }
}

// LOOPS

while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in iterable {
    block
}

// Standard C for loop for ( int i = 0; i < 20; i++>) written like:
for i in 0..20 {
    println!("{}", i);
}

// to avoid moving values in vectors, arrays!
for rs in &strings {
    println!("String {:?} is at address {:p}.", *rs, rs);
}

let answer = loop {
    if let Some(line) = next_line() {
        if line.starts_with("answer: ") {
            break line;
        }
    } else {
        break "answer: nothing";
    }

    continue; //jumps to the next loop iteration
    // in for loop, if there are no more values, the loop exits
};

'search:
for room in apartment {
    for spot in room.hiding_spots() {
        if spot.contains(keys) {
            println!("Your keys are {} in the {}.", spot, room);
            break 'search; // use labels to exit correct loop blocks
        }
    }
}

// A BREAK can habe both A LABEL and A VALUE EXPRESSION for the let declaration.

// Perfectly normal => ! return type makes the function -> divergent function
fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
    socket.listen();
    loop {
        let s = socket.accept();
        handler.handle(s);
    }
}

// Type - associated function call!
// SIMILAR TO STATIC METHODS IN OOP
let mut numbers = Vec::new();

// One quirk!
return Vec<i32>::with_capacity(1000); // error: something about chained comparisons
let ramp = (0 .. n).collect<Vec<i32>>(); // same error

return Vec::<i32>::with_capacity(1000); // ok, using ::<
let ramp = (0 .. n).collect::<Vec<i32>>(); // ok, using ::<

::<...> => turbofish!

return Vec::with_capacity(10); // ok, if the fn return type is Vec<i32>
let ramp: Vec<i32> = (0 .. n).collect(); // ok, variable's type is given

.. // RangeFull
a .. // RangeFrom { start: a }
.. b // RangeTo { end: b }
a .. b // Range { start: a, end: b }

..= b // RangeToInclusive { end: b }
a ..= b // RangeInclusive::new(a, b)

// Quicksort might look like this!
fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return; // Nothing to sort.
    }
    // Partition the slice into two parts, front and back.
    let pivot_index = partition(slice);
    // Recursively sort the front half of `slice`.
    quicksort(&mut slice[.. pivot_index]);
    // And the back half.
    quicksort(&mut slice[pivot_index + 1 ..]);
}

let padovan: Vec<u64> = compute_padovan_sequence(n);
for elem in &padovan { // elem is &u64, *elem is u64
    draw_triangle(turtle, *elem);
}

RUST USES ! for BITWISE NOT
let hi: u8 = 0xe0;
let lo = !hi; // 0x1f

// CLOSURES
let is_even = |x| x % 2 == 0;

// If return type is given, block is requried
let is_even = |x: u64| -> bool x % 2 == 0; // error
let is_even = |x: u64| -> bool { x % 2 == 0 }; // ok
```

* A block can contain item declarations (fn inside fn eg.)

* **It’s considered good style to omit the types whenever they can be inferred.**

* Bit shifting is always sign-extending on signed integer types and zero-extending on
unsigned integer types.

## Errors

Errors are handled using Result type
* Environmental errors: input, network, permissions etc.
* Can be unavoidable, not related to us

**Panic is for the other kind of error, the kind that should never happen**

You can use panic!() macro
* Panics are all programmer related!
* Panics are like runtimeexceptions seen in Java, the behavior is well-defined, it just shouldnt be happening!
* Panics are not **crash or undefined behavior**
* Panics are per thread
* std::panic::catch_unwind() can be used to cath stack unwinding, and continue execution

* Note that you can catch panics that unwind the stack only, not every panic has the same routine!

> If you compile with -C panic=abort, the first panic in your program immediately aborts the process. (With this option, Rust does not need to know how to unwind the stack, so this can reduce the size of your compiled code.

> If a .drop() method triggers a second panic while Rust is still trying to clean up after the first, this is considered fatal. Rust stops unwinding and aborts the whole process.

```rust
// funcs dont have exceptions, but funcs that can fail have a return type!
fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>

// most common way, rust way of try/catch
match get_weather(hometown) {
    Ok(report) => {
        display_weather(hometown, &report);
    }
    Err(err) => {
        println!("error querying the weather: {}", err);
        schedule_weather_retry();
    }
}

fn remove_file(path: &Path) -> Result<()> // type alias!
pub type Result<T> = result::Result<T, Error>;

//Printing every bit of err details

use std::error::Error;
use std::io::{Write, stderr};

/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
```

**You can add a ? to any expression that produces a result**

* On success, unwraps

* On error, immediately returns from the enclosing function, passing the err up the call chain

* ? also works similarly with the Option type. In a function that returns Option, you can use ? to unwrap a value and return early in the case of None

```rust
let weather = get_weather(hometown).ok()?;
```

**thiserror crate** can help defining good error types when dealing with multiple errors in a function

> All of the standard library error types can be converted to the type Box<dyn std::error::Error + Send + Sync + 'static>.

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

let io_error = io::Error::new( // make our own io::Error
    io::ErrorKind::Other, "timed out");
return Err(GenericError::from(io_error)); // manually convert to GenericError

loop {
    match compile_project() {
        Ok(()) => return Ok(()),
        Err(err) => {
        if let Some(mse) = err.downcast_ref::<MissingSemicolonError>() { // borrow a ref to err type, if it happens to be the MissingSemicolonError
        // this way, matches an error that we want to cover specifically for
            insert_semicolon_in_source_code(mse.file(), mse.line())?;
            continue; // try again!
        }
            return Err(err);
        }
    }
}

let num = digits.parse::<u64>().unwrap(); // use this if you know that an error just cant happen!
"99999999999999999999".parse::<u64>() // overflow error, using unwrap here would be a bug

// like Go
let _ = ...
writeln!(stderr(), "error: {}", err); // warning: unused result
let _ = writeln!(stderr(), "error: {}", err); // ok, ignore result

// Custom Errors
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")] // generates code!
pub struct JsonError {
    message: String,
    line: usize,
    column: usize,
}
```

Crates -> Code sharing

Modules -> namespaces!

> Modules do not automatically inherit names from their parent modules. For example,
suppose we have this in our proteins/mod.rs:

* By default, paths are relative to the current module

* self is also a synonym for the current module, so we could write either:
    * use self::AminoAcid::*;
    * AminoAcid::*;

> The keywords super and crate have a special meaning in paths: super refers to the
parent module, and crate refers to the crate containing the current module.

*  an absolute path, starting with ::, which always refers to an external crate.
    * use ::image::Pixels; // the `image` crate's `Pixels`
    * use self::image::Sampler; // the `image` module's `Sampler`

> A struct’s fields, even private fields, are accessible throughout the module where the
struct is declared, and its submodules. Outside the module, only public fields are
accessible.

> A constant is a bit like a C++ #define: the value is compiled into your code every
place it’s used. A static is a variable that’s set up before your program starts running
and lasts until it exits. 

Turning this program into a library is easy. Here are the steps
* Rename the file src/main.rs to src/lib.rs
* Add the pub keyword to items in src/lib.rs that will be public features of our
library.
* Move the main function to a temporary file somewhere. We’ll come back to it in a
minute

#[test] -> runs func when cargo test is called

> Use debug_assert! and debug_assert_eq! instead to write assertions that are checked only in debug build.

You can test error cases:

```rust
/// This test passes only if division by zero causes a panic,
/// as we claimed in the previous chapter.
#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected="divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0; // should panic!
}

...

cfg -> conditional compilation! // book page 191

#[cfg(test)] // include this module only when testing
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool { // avoids compiler warnings!
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
```

> Integration tests are valuable in part because they see your crate from the outside just as a user would. They test the crate’s public API

* Create tests under alongside src for integration tests!

## Structs

> The convention in Rust is for all types, structs included, to have names
that capitalize the first letter of each word, like GrayscaleMap, a convention called
CamelCase (or PascalCase). Fields and methods are lowercase, with words separated
by underscores. This is called snake_case

## Code Samples

```rust
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

// not using type inference
fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

// usign type inference
fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// closures
HttpServer::new( || ...) //args between ||
```

