pub fn run() {
    println!("1) let 默认不可变，let mut 可变。");
    let x = 10;
    println!("不可变变量 x = {}", x);

    let mut y = 20;
    y += 5;
    println!("可变变量 y = {}", y);

    println!("2) 常量使用 const，必须标注类型。");
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    println!("3) 变量遮蔽（shadowing）允许复用同名变量。");
    let spaces = "   ";
    let spaces = spaces.len(); // 从 &str 变为 usize
    println!("shadowing 后 spaces = {}", spaces);

    println!("4) 解构赋值。");
    let (a, b, c) = (1, 2, 3);
    println!("a = {}, b = {}, c = {}", a, b, c);
}
