fn takes_ownership(s: String) {
    println!("拿到所有权: {}", s);
}

fn borrows_value(s: &String) {
    println!("借用引用: {}", s);
}

fn mutable_borrow(s: &mut String) {
    s.push_str(" + changed");
}

pub fn run() {
    println!("所有权规则: 每个值有唯一所有者，离开作用域自动释放。");
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("{}", s1); // s1 已被 move，不能再用

    println!("借用允许访问而不转移所有权。");
    let s2 = String::from("borrow me");
    borrows_value(&s2);
    println!("借用后仍可用: {}", s2);

    println!("可变借用在同一时刻只能有一个。");
    let mut s3 = String::from("mutable");
    mutable_borrow(&mut s3);
    println!("可变借用后: {}", s3);

    println!("切片是常见借用类型。");
    let first_word = &s3[0..3];
    println!("first slice: {}", first_word);
}
