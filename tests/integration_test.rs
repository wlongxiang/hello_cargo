use hello_cargo;

#[test]
fn assert_hello() {
    assert_eq!(hello_cargo::mymodule::hello(), 1);
}