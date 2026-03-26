pub fn run() {
    println!("标量类型: 整数、浮点、布尔、字符。");
    let int_num: i32 = -42;
    let float_num: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let ch: char = 'R';
    println!(
        "i32={}, f64={}, bool={}, char={}",
        int_num, float_num, is_rust_fun, ch
    );

    println!("复合类型: 元组和数组。");
    let tup: (i32, f64, char) = (500, 6.4, 'Z');
    let (x, y, z) = tup;
    println!("tuple 解构: x={}, y={}, z={}", x, y, z);

    let arr = [10, 20, 30, 40, 50];
    println!("array 第一个元素={}，长度={}", arr[0], arr.len());

    println!("切片 &str / &[T] 是常见借用视图。");
    let s = String::from("hello rust");
    let hello = &s[0..5];
    let part = &arr[1..4];
    println!("字符串切片={}, 数组切片={:?}", hello, part);
}
