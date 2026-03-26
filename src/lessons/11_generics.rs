fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    println!("泛型让函数/结构体可复用于多种类型。");
    let nums = vec![10, 99, 24, 77];
    let chars = vec!['a', 'z', 'k'];
    println!("largest(nums) = {}", largest(&nums));
    println!("largest(chars) = {}", largest(&chars));

    let p = Point { x: 3, y: 4 };
    println!("Point = {:?}, x = {}", p, p.x());
    println!("y = {}", p.y);
}
