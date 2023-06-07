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


    // if let 简洁控制流

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // 可简写为
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。
    // 换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
}

