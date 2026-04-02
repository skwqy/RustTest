#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

/*
Person<'a> 里的 'a 读作 “生命周期参数 a”（lifetime）。它表示：Person 这个结构体里存放的引用（这里是 &'a str）至少要活到 'a 这么久，
从而保证 Person 在使用期间不会引用已经失效的数据。

在你的代码里：
    name: &'a str 的意思是：name 引用的那段字符串数据，生命周期必须 ≥ 'a
    Person<'a> 的 'a 就是把这个“引用要活多久”的约束，显式写在类型上
*/
fn main() {
    let name = "Peter";
    let age = 27;
    
    let peter = Person { name, age };

    println!("name={}, age={}", peter.name, peter.age);

    // Rust 也通过 {:#?} 提供了 “美化打印” 的功能：
    println!("{:#?}", peter);
}
