use hello_cargo;
use hello_cargo::{mymodule, mydirmodule, utils};


fn main() {
    utils::guess_number();
    // test_mut();
    mymodule::hello();
    mymodule::submodule::greeting();
    greetings::nihao();
    mydirmodule::hello();
    // the following two calls are the same, cuz they are remapped within mydirmodule
    mydirmodule::greeting();
    mydirmodule::submodule::greeting();

    // try lib, note that lib uses the package namespace
    hello_cargo::hello_lib();
}

// a small module within bin, only for small stuff
mod greetings {
    pub fn nihao() {
        println!("你好");
    }
}
