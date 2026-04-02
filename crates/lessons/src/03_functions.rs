fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式返回值，不写分号
}

fn print_label(label: &str) {
    println!("label: {}", label);
}

fn block_expression_demo(x: i32) -> i32 {
    let y = {
        let inner = x * 2;
        inner + 1
    };
    y
}

pub fn run() {
    println!("函数使用 fn 定义，可声明参数和返回值类型。");
    let sum = add(3, 5);
    println!("add(3, 5) = {}", sum);

    println!("函数也可以无返回值（返回 unit 类型 ()）。");
    print_label("Rust function");

    println!("代码块本身是表达式。");
    let result = block_expression_demo(10);
    println!("block expression result = {}", result);
}
