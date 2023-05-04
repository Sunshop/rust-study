use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println() 普通函数调用
    // println! 是一个宏（macro），而不是函数
    println!("Guess the number!");

    println!("please input your guess.");

    // rand::thread_rng() 函数提供实际使用的随机数生成器
    // 生成一个1和100之间的随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // 在Rust中，变量默认是不可变的
        // 在变量名前使用 mut 来使变量可变
        // String 是标准库提供的字符串类型
        // ::new() 表示 new 是 String 类型的一个 关联函数（associated function）
        // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个实例
        // 关联函数类似于静态方法（static method）
        // 关联函数的调用方式是 :: ，而不是 . ，因为它们不是针对某个实例
        // 关联函数经常被用作构造器（constructor）
        // new 函数创建了一个新的空字符串，接着 read_line 方法读取用户输入并附加到这个字符串上
        let mut guess = String::new();

        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝
        // read_line 返回一个类型为 Result 的值，它是一个枚举（enumeration），它的成员是 Ok 和 Err
        // 如果 io::Result 的实例的值是 Err，expect 会导致程序崩溃，并显示传递给它的参数的内容
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust 允许用一个新值来 隐藏（shadow） guess 之前的值
        // parse 不能将
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match 表达式是 Rust 中的控制流运算符
        // match 表达式由分支（arms）构成
        // 分支包含一个模式（pattern）和表达式开头的值与模式相匹配时应该执行的代码
        // 50 与 38 时，cmp 方法会返回 Ordering::Greater，然后执行关联的代码块
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 退出循环
            },
        }
    }
}
