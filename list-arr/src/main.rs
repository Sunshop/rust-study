

mod list_test {
    pub fn test() {
        let mut v: Vec<i32> = Vec::new();
        
        // 宏
        let v1 = vec![1, 2, 3];

        v.push(5);
        v.push(6);
        v.push(7);

        // 读取
        // 通过索引
        let third1: &i32 = &v1[2];
        println!("The third element is {}", third1);
        // 通过 get 方法
        let third: Option<&i32> = v1.get(2);
        match third {
            Some(i) => println!("The third element is {}", i),
            None => println!("There is no third element."),
        }
    }
}

fn main() {
    println!("Hello, world!");
    list_test::test();
}
