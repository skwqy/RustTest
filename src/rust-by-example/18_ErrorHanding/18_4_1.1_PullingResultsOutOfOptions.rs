/**
 * 将 Option<T> 转换为 Result<T, E> 最简单的方法是：
 * 使用 map() 方法来将 Option 转换为 Result。
 * 使用 map_err() 方法来将 Result 转换为 Option。
 * 使用 and_then() 方法来将 Option 转换为 Result。
 * 使用 or_else() 方法来将 Result 转换为 Option。
 * 使用 unwrap_or() 方法来将 Option 转换为 Result。
 * 使用 unwrap_or_else() 方法来将 Option 转换为 Result。
 * 使用 unwrap_or_default() 方法来将 Option 转换为 Result。
 * 使用 unwrap_or_else() 方法来将 Option 转换为 Result。
 * 使用 unwrap_or_default() 方法来将 Option 转换为 Result。
 * 使用 unwrap_or_else() 方法来将 Option 转换为 Result。
*/
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}

