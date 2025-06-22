// rust_arch_demo/src/greetings.rs


#[cfg(feature = "fancy")]
pub fn say_hello(name: &str) -> String {
    format!("🌟 Hello, {}! 🌟", name)
}

#[cfg(not(feature = "fancy"))]
pub fn say_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}


