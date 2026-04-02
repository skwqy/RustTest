use std::fmt;

struct Circle {
    radius: i32
}

//要把任何类型转换成 String，只需要实现那个类型的 ToString trait。
//然而不要直接这么做，您应该实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型，就像 print! 一节中讨论的那样。
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
