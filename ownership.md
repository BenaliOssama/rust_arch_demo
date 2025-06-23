
### ✅ The 3 Ownership Rules (core concept of Rust memory safety):

1. **Each value in Rust has a *single owner*.**
2. **When the owner goes out of scope, the value is dropped (memory freed).**
3. **A value can only have *one mutable reference* OR *any number of immutable references*, not both at the same time.**

---

### 🧠 Why Ownership Exists

Rust does not have a garbage collector. It ensures memory safety *at compile time* using ownership rules. This avoids memory leaks and data races.

---

### 🔄 Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// s1 is now invalid
```

You can't use `s1` after this. Ownership moved to `s2`.

---

### 📋 Clone

To keep using `s1`, you must explicitly copy the value:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy
```

---

### 📌 Borrowing (References)

* **Immutable borrow** (read-only):

```rust
let s = String::from("hello");
let len = calculate_length(&s); // pass a reference
// s is still valid
```

* **Mutable borrow** (write):

```rust
let mut s = String::from("hello");
change(&mut s); // pass mutable reference
```

Only one mutable reference is allowed at a time.

---

### 🔒 No Dangling References

Rust prevents you from returning references to values that go out of scope:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ❌ this would be a dangling reference
}
```

---

### 📚 Rust Docs

Here’s the official chapter on ownership (read it slowly):
👉 [https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

