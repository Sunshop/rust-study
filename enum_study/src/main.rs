
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // fn route(ip_kind: IpAddrKind) {

    // }

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let home2 = IpAddr::V4(192, 168, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    fn route(ip_kind: IpAddr) {

    }
    
    route(home2);

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    // let some_number = Option::Some(5);


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    if !y.is_none() {
        println!("y is some");
    }
    let sum = x + y.unwrap(); // 取值
    println!("{sum}");
}

// 枚举与结构体结合
// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//             // 在这里定义方法体
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }
