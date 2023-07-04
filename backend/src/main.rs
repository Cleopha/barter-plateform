fn main() {
    print!("Hello World! {:?}", simple_test())
}

fn simple_test() -> u16 {
    12
}

#[test]
fn feature() {
    assert_eq!(simple_test(), 12);
}
