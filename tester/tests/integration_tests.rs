#[test]
fn it_adds_two() {
    use tester::add::add;
    let result = add(2, 3);
    assert_eq!(result, 5);
}