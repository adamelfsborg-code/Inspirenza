mod user;
mod prelude {
    pub use crate::user::{User,Admin};
}

use crate::prelude::*; 

fn main() {
    let _u = User{ name: String::from("Adam Elfsborg") };
    let _a = Admin{ name: String::from("Adam Elfsborg") };

    println!("{:#?}", _a.name);

    foo("Hello");
}

fn foo(bar: &str) -> usize {
    bar.chars().count()
}

#[test]
fn test_foo() {
    let bar = String::from("Baaaa");
    let foo = foo(&bar);
    assert_eq!(foo, bar.len())
}