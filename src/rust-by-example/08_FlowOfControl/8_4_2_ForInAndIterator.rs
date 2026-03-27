/*
    for in 结构能以几种方式与 Iterator 互动。在 迭代器 trait 一节将会谈到，如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器。
    这并不是把集合变成迭代器的唯一方法，其他的方法有 iter 和iter_mut 函数。
    1、iter() 方法生成一个不可变引用的迭代器。
    2、into_iter() 方法会消耗集合。
    3、iter_mut() 方法生成一个可变引用的迭代器。
 */

 fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // iter - 在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // names.into_iter() 会移动（move）names 的所有权,println!("{:?}", names)，属于“被 move 后再次借用”（E0382）
    // println!("names: {:?}", names);

    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}


 

