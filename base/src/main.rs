mod test;

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 2;
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function(28);

    test::test_fn();

    let x = five();

    println!("{x}");


    let y = plus_one(9);

    println!("{y}");
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}
