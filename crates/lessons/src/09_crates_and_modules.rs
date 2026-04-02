mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hosting::add_to_waitlist");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: toast.to_string(),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }
}

use front_of_house::hosting;

pub fn run() {
    println!("crate 是编译单元，module 用于组织命名空间。");
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("toast = {}, fruit = {}", meal.toast, meal.fruit());

    println!("pub 决定可见性；use 可简化路径引用。");
}
