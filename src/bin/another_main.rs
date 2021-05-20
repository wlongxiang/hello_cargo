use hello_cargo;
use hello_cargo::mymodule::submodule;
use hello_cargo::utils;

fn main() {
    hello_cargo::hello_lib();
    submodule::greeting();
    utils::test_mut();
}