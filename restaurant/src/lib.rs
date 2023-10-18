mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    // 结构体
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 关联函数
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 结构体
    pub struct Lunch {
        pub staple_food: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }


    fn fix_incorrect_order() {
        cook_order();
        // 使用super访问父模块
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用关联函数创建实例
    // 黑麦土司为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换面对类型
    meal.toast = String::from("Wheat");

    // error 报错
    // 不允许查看或者修改早餐附带的水果
    meal.seasonal_fruit = String::from("blueberries");

    // 使用关联函数创建实例
    // 因为 Lunch 中包含私有字段，所以需要提供一个公共的关联函数来创建实例
    // error 报错
    let mut curLunch = back_of_house::Lunch {
        staple_food: String::from("rice"),
        seasonal_fruit: String::from("apple"),
    };

    // 枚举值默认成员都是公有的，结构体遵循常规，内部全部为私有，可使用pub关键字
    let order = back_of_house::Appetizer::Soup;
}
