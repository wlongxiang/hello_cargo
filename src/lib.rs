// this decides what modules or APIs to expose to bins

pub mod mydirmodule;
pub mod mymodule;
pub mod utils;

pub fn hello_lib() {
    println!("Hello from within lib!");
}