fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    let coin_number = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("{coin_number}");


    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // 注释此行,会编译报错 match的匹配是穷尽的:必须穷举到所有可能性来使得代码有效
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six {:?}!", six);


    // 通配符

    let dice_roll = 3;
    match dice_roll {
        3 => {
            println!("加帽子");
        }
        7 => {
            println!("取帽子");
        }
        other => {
            println!("移动 => {other}");
        }
    }

    match dice_roll {
        3 => {
            println!("加帽子");
        }
        7 => {
            println!("取帽子");
        }
        _ => {
            // 也可使用 _ 来代替 other
            println!("移动");
        }
    }

}

