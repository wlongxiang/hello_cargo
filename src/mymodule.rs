/*
this creates a module with the same name as file name
one can import from main.rs by doing mod <filename>
 */



pub fn hello() {
    println!("Hello from mymodule");
}

// one can also create submodules within this module

pub mod submodule {
    pub fn greeting() {
        println!("你好");
    }
}