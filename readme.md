- https://doc.rust-lang.org/book/ch04-03-slices.html
- https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html

# Create crate
``` bash
cargo new <name_of_crate>
```

# Build and run
``` bash
cargo build --release; ./target/release/<name_of_crate>
cargo build --release; ./target/release/testcode
```

# Disable dumb warnings
export RUSTFLAGS="-A unused_imports -A dead_code -A unused_variables -A unused_macros"

## re-enable
export RUSTFLAGS=""

# formatting
``` bash
rustup component add rustfmt
cargo fmt
```

# args
``` rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

# Newtypes
It is sometimes useful to create a specific version of something because bigbrain rust developers don't implement named parameters.
Thus, confusion of order can happen easily:
``` rust
fn verbose_enough(set: Verbosity, requested: Verbosity) -> Bool {}

// --> 
let res = verbose_enough(msg_verbosity, set_verbosity); // wrong order!
```

This can be fixed like this:
``` rust
pub enum Verbosity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}
pub struct MessageVerbosity(Verbosity);
pub struct ConfiguredVerbosity(Verbosity);

# Strings
``` rust
// &str -> String
String::from(&str)
"test".to_string()

// String -> &str
let string = "test".to_string().as_str()

fn main() {
    let str_value = "test";
    let string = String::from(str_value);
    println!("{}", string);
}
```

# Related types
Still trying to figure out how to best organize related types.
Take:
``` rust
pub struct RelativePosixPath(String);
pub struct AbsolutePosixPath(String);
```

I want:
- Functions to allow any of those types
- Functions to return any of those types
- Easy way to get the string value, instead of the newtype "wrapper"

## 

## Impl trait for multiple types at once
With this macro, we can define a method for multiple structs at once.
``` rust
trait T {
    fn double(&self) -> u32;
}
macro_rules! impl_T {
    (for $($t:ty),+) => {
        $(impl T for $t {
            fn double(&self) -> u32 {
                self.x * 2
            }
        })*
    }
}

struct A {
    x: u32,
}
struct B {
    x: u32,
}
impl_T!(for A, B);

```

## Allow trait as function parameter
``` rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```