Step 1: **Create the Project Structure**

Run this command in your terminal:

```bash
cargo new --lib rust_arch_demo
```

This creates a library package named `rust_arch_demo`, which we’ll later expand with:

* A binary crate (main app)
* Internal modules
* A separate local crate (in `crates/` directory)
* Reusable paths and `use` statements

Step 2: **Add a Binary Crate Inside the Package**

Inside the `rust_arch_demo` folder, create a `src/bin` directory:

```bash
mkdir -p rust_arch_demo/src/bin
```

Then create a binary file `app.rs` inside `src/bin`:

```rust
// rust_arch_demo/src/bin/app.rs
fn main() {
    println!("Hello from the binary crate!");
}
```

Now your package contains:

* A library crate (`src/lib.rs`)
* A binary crate (`src/bin/app.rs`)

You can run the binary with:

```bash
cargo run --bin app
```



Step 3: **Organize the Library (`lib.rs`) with Modules**

Inside `rust_arch_demo/src/lib.rs`, create two modules:

```rust
// rust_arch_demo/src/lib.rs
pub mod math;
pub mod greetings;
```

Create the module files:

* `rust_arch_demo/src/math.rs`
* `rust_arch_demo/src/greetings.rs`

Add simple functions inside each:

```rust
// rust_arch_demo/src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
// rust_arch_demo/src/greetings.rs
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Then, modify the binary crate to use these modules via `use`:

```rust
// rust_arch_demo/src/bin/app.rs
use rust_arch_demo::{math, greetings};

fn main() {
    let sum = math::add(5, 7);
    let greeting = greetings::say_hello("User");

    println!("Sum: {}", sum);
    println!("{}", greeting);
}
```

Run again with:

```bash
cargo run --bin app
```


Step 4: **Create Nested Modules in `math`**

1. Inside `src/math.rs`, replace the simple function with a module declaration:

```rust
// rust_arch_demo/src/math.rs
pub mod operations;
```

2. Create a folder `src/math/` and inside it a file `operations.rs`:

```bash
mkdir rust_arch_demo/src/math
```

```rust
// rust_arch_demo/src/math/operations.rs
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

3. Modify `math.rs` to re-export `multiply` for easier access:

```rust
// rust_arch_demo/src/math.rs
pub mod operations;

pub use operations::multiply;
```

4. Modify `app.rs` to use the new nested function:

```rust
use rust_arch_demo::{math, greetings};

fn main() {
    let sum = math::add(5, 7);
    let product = math::multiply(3, 4);
    let greeting = greetings::say_hello("User");

    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("{}", greeting);
}
```

Now running the binary will show results from nested modules.

Confirm when ready. Next step: create a local crate inside a `crates` folder and add it as a dependency.


Step 5: **Create a Local Crate and Add It as a Dependency**

1. In the root of your project (`rust_arch_demo`), create a folder named `crates`:

```bash
mkdir crates
```

2. Inside `crates`, create a new library crate:

```bash
cargo new --lib crates/utils
```

This creates `crates/utils` with its own `Cargo.toml` and `src/lib.rs`.

3. Add a simple utility function in `crates/utils/src/lib.rs`:

```rust
// crates/utils/src/lib.rs
pub fn print_message(msg: &str) {
    println!("Utils says: {}", msg);
}
```

4. Now, link this local crate in your main `Cargo.toml`:

Open `rust_arch_demo/Cargo.toml` and add:

```toml
[dependencies]
utils = { path = "crates/utils" }
```

5. Use this crate in your library or binary.

For example, update `app.rs` to use it:

```rust
use rust_arch_demo::{math, greetings};
use utils;

fn main() {
    let sum = math::add(5, 7);
    let product = math::multiply(3, 4);
    let greeting = greetings::say_hello("User");

    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("{}", greeting);

    utils::print_message("This is a message from the utils crate!");
}
```

Run:

```bash
cargo run --bin app
```


Step 6: **Use `pub use` and Aliasing with `as`**

We’ll improve code readability and demonstrate `pub use` + `as`.

### 1. Re-export `add` and `multiply` from `lib.rs`

Edit `lib.rs`:

```rust
// src/lib.rs
mod math;
mod greetings;

pub use math::{add, multiply};
pub use greetings::say_hello;
```

Now you can call these functions directly via the crate root.

### 2. Alias imports in `app.rs`

Update `app.rs` to use aliasing:

```rust
use rust_arch_demo::{add as add_numbers, multiply, say_hello};
use utils::print_message;

fn main() {
    let sum = add_numbers(5, 7);
    let product = multiply(3, 4);
    let greeting = say_hello("User");

    println!("Sum: {}", sum);
    println!("Product: {}", product);
    println!("{}", greeting);

    print_message("This is a message from the utils crate!");
}
```

### Summary of what's demonstrated:

* `pub use` to re-export from submodules
* `use ... as` for aliasing
* Clean public API via `lib.rs`

Run it again:

```bash
cargo run --bin app
```


