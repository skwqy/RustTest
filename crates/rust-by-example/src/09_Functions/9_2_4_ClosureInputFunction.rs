/*
   虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法是不允许的。当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的。其受限制程度按以下顺序递减：

       1、Fn：表示捕获方式为通过引用（&T）的闭包
       2、FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
       3、FnOnce：表示捕获方式为通过值（T）的闭包
   对闭包所要捕获的每个变量，编译器都将以限制最少的方式来捕获。
   译注：这句可能说得不对，事实上是在满足使用需求的前提下尽量以限制最多的方式捕获。

*/

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}

