fn main() {
    println!("Hello, world!");
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