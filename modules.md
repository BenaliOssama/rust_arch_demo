Here's a **minimal example** showing the **three ways** to declare and define a `garden` module in Rust:

---

## 📁 Project structure (created with `cargo new module_example`)

```
module_example/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── garden.rs          <-- Option 2
    └── garden/
        └── mod.rs         <-- Option 3
```

---

## ✅ Option 1: Inline module (inside `main.rs`)

**`src/main.rs`**

```rust
mod garden {
    pub fn plant() {
        println!("Planting something!");
    }
}

fn main() {
    garden::plant();
}
```

---

## ✅ Option 2: External file `src/garden.rs`

**`src/main.rs`**

```rust
mod garden;

fn main() {
    garden::plant();
}
```

**`src/garden.rs`**

```rust
pub fn plant() {
    println!("Planting something!");
}
```

---

## ✅ Option 3: External file `src/garden/mod.rs`

**`src/main.rs`**

```rust
mod garden;

fn main() {
    garden::plant();
}
```

**`src/garden/mod.rs`**

```rust
pub fn plant() {
    println!("Planting something!");
}
```

---

📚 Docs: [Modules - The Rust Book](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
/*_______________________________________________________________________*/
Here’s a clear summary of what we discussed about Rust modules:

---

### 1. **Top-level modules**

* Declared only in the **crate root file**:

  * `src/main.rs` (binary crate)
  * `src/lib.rs` (library crate)
* Example in `main.rs`:

  ```rust
  mod garden;  // declares top-level module 'garden'
  ```
* Rust looks for `src/garden.rs` or `src/garden/mod.rs`.

---

### 2. **Submodules**

* Declared inside a module file (not crate root).
* For example, inside `garden.rs`:

  ```rust
  pub mod vegetables;  // declares submodule 'vegetables'
  ```
* Rust looks for `src/garden/vegetables.rs` or `src/garden/vegetables/mod.rs`.

---

### 3. **File and folder structure rules**

* For a module `foo` declared in a file `bar.rs` or `bar/mod.rs`:

  * Submodules go inside a folder named `bar/`.
  * Files like `bar/foo.rs` or `bar/foo/mod.rs` hold submodule code.

---

### 4. **About `src/bin/` folder**

* Each `.rs` file in `src/bin/` is its own binary crate root.
* You **cannot declare sibling modules** next to `app.rs` in `src/bin/`.
* To add modules to a binary `app.rs`, create a folder `src/bin/app/` and place submodules there.
* Declare submodules inside `app.rs` like:

  ```rust
  mod helper;
  ```


