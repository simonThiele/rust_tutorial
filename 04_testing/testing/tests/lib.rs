extern crate adder;

#[test]
fn should_add() {
    assert_eq!(adder::add(3, 2), 5);
}
