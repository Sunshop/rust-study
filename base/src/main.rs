mod test;

fn main() {
    println!("Hello, world!");

    another_function(28);

    test::test_fn();
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}
