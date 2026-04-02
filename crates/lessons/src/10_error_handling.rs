use std::num::ParseIntError;

fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
    let n: i32 = input.parse()?;
    Ok(n * 2)
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn run() {
    println!("可恢复错误: Result<T, E>。");
    match parse_and_double("21") {
        Ok(v) => println!("parse_and_double(\"21\") = {}", v),
        Err(e) => println!("解析失败: {}", e),
    }

    println!("? 运算符用于错误传播，减少样板代码。");
    let bad = parse_and_double("abc");
    println!("parse_and_double(\"abc\") = {:?}", bad);

    println!("不可恢复场景常用 panic!（本示例不触发 panic）。");

    println!("Option<T> 常用于无值但不算错误的情况。");
    println!("10 / 2 = {:?}", divide(10.0, 2.0));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));
}
