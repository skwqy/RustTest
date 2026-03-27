/*
    我们经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数。我们得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现。
    只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。
*/

/*
    unwrap() 的含义是：“我确定这里一定有值/一定成功，否则就直接 panic 崩溃”。

    它常见在两种类型上：

    Option<T>
        Some(v).unwrap() -> 返回 v
        None.unwrap() -> panic
    Result<T, E>
        Ok(v).unwrap() -> 返回 v
        Err(e).unwrap() -> panic（并打印错误）
 */
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println! {"Sum: {:?}", sum};
}
