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

