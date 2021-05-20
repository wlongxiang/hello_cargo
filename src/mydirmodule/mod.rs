/*
This is a module organized in a directory, which has:
- An entry point mod.rs, which can also expose submodules within this module by `pub mod`
- Other submodules in a different file


It is best to think of this file as a definition for API, ie, what to expose to end users.
 */

pub mod submodule; // such that this submodule is visible to users of this module

pub use self::submodule::greeting; // Re-export `submodule::greeting`, usefule to expose interfaces

pub fn hello() { // ⭐️ The function has to be public to access from outside
    println!("Hello from mydirmodule");
}