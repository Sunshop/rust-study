mod test;

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    return x + 2;
}

fn main() {
    println!("Hello, world!");

    another_function(28);

    test::test_fn();

    let x = five();

    let y = plus_one(x);

    println!("{x}");
    println!("{y}");

    // 可变引用
    let mut a = String::from("hello");
    change(&mut a);


}

// fn first_word() ->

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn another_function(x: i32) {
    println!("Another function. {x}");
}
