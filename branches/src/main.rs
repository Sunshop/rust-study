fn main() {
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");

    // }

    // let number = 3;

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // 在 let 语句中使用 if
    // if 的每个分支的可能的返回值都必须是相同类型

    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("{number}")

    // 循环

    // loop
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("{counter}");
    // println!("{result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count == {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("end count = {count}");

    // while
    // let mut number = 3;
    // while number != 0 {
    //     println!{"{number}"};

    //     number -= 1;
    // }
    // println!("out!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }
    // for number in 1..4 {
    //     println!("{number}");
    // }
    // for number in (1..4).rev() {
    //     println!("{number}");
    // }

    // let x = change_temperature(88.0, 'h');

    // println!("{x}");

    // 创建字符串对象
    // 1. 创建一个新的空字符串
    let s = String::new();
    // 2. 从字符串字面值创建字符串
    let s = String::from("基");

    let index = s.len();

    println!("{index}")

}

fn change_temperature(temperature: f32, t_type: char) -> f32 {
    let mut change_temp: f32 = 0.0;
    if temperature != 0.0 {
        if t_type == 'h' {
            change_temp = 1.8 * temperature + 32.0;
        }
        if t_type == 's' {
            change_temp = (temperature - 32.0) / 1.8;
        }
    }
    return change_temp;
}

