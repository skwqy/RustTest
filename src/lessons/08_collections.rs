use std::collections::HashMap;

pub fn run() {
    println!("Vec<T>: 动态数组。");
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("vec = {:?}, 第2个元素={}", nums, nums[1]);
    if let Some(last) = nums.pop() {
        println!("pop -> {}", last);
    }

    println!("String: UTF-8 字符串。");
    let mut s = String::from("hello");
    s.push_str(" rust");
    println!("string = {}", s);
    for ch in s.chars() {
        print!("{} ", ch);
    }
    println!();

    println!("HashMap<K, V>: 键值映射。");
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 82);

    let alice = scores.get("Alice").copied().unwrap_or(0);
    println!("Alice score = {}", alice);

    scores.entry(String::from("Bob")).and_modify(|v| *v += 5);
    scores.entry(String::from("Carol")).or_insert(88);
    println!("scores = {:?}", scores);
}
