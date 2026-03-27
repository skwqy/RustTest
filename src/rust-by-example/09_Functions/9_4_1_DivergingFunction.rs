/*
   发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型
*/

// 发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。
// 和所有其他类型相反，这个类型无法实例化，因为此类型可能具有的所有可能值的集合为空。 注意，它与 () 类型不同，后者只有一个可能的值。
fn some_fn() {
    ()
}

// 如下面例子，虽然返回值中没有信息，但此函数会照常返回。
fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line.")
}



