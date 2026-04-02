/*
   发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型
*/

// 发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。
// 和所有其他类型相反，这个类型无法实例化，因为此类型可能具有的所有可能值的集合为空。 注意，它与 () 类型不同，后者只有一个可能的值。
#![feature(never_type)]

// 下面这个函数相反，这个函数永远不会将控制内容返回给调用者。
fn main() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}



