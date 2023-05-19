#[derive(Debug)]
// 结构体传参
struct Rectangle {
    width: u32,
    height: u32,
}

// 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area1(&self, other: &Rectangle) -> u32 {
        // ...执行
        8
    }
}

struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 传统传参
    let widht1 = 30;
    let height1 = 50;

    println!(
        "面积为：{}",
        area(widht1, height1)
    );

    // 元组传参
    let retc1 = (30, 50);

    println!(
        "面积为1：{}",
        area1(retc1)
    );

    // 使用结构体
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 80,
        height: 100,
    };
    println!(
        "面积为2：{}",
        area2(&rect2)
    );
    println!(
        "面积为2：{}",
        rect2.area()
    );
    rect3.area1(&rect2);

    let reac4 = Rectangle2::square(8);

}

fn area(widht: u32, height: u32) -> u32 {
    widht * height
}

fn area1(deimensions: (u32, u32)) -> u32 {
    deimensions.0 * deimensions.1
}

fn area2(params: &Rectangle) -> u32 {
    params.width * params.height
}
