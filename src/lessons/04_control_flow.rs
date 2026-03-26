pub fn run() {
    println!("if / else 是表达式，可直接赋值。");
    let number = 6;
    let parity = if number % 2 == 0 { "偶数" } else { "奇数" };
    println!("{} 是 {}", number, parity);

    println!("loop + break 返回值。");
    let mut counter = 0;
    let doubled = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("loop 返回 doubled = {}", doubled);

    println!("while 循环。");
    let mut n = 3;
    while n > 0 {
        print!("{} ", n);
        n -= 1;
    }
    println!();

    println!("for 遍历集合。");
    let arr = [11, 22, 33];
    for item in arr {
        print!("{} ", item);
    }
    println!();
}
