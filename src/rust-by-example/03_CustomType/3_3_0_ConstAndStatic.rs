/**
Rust 有两种常量，可以在任意作用域声明，包括全局作用域。它们都需要显式的类型声明：

const：不可改变的值（通常使用这种）。
static：具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）。

* 1、const 是常量，只能在编译时确定，不能在运行时确定。
* 2、static 是全局变量，可以在所有其他作用域之外声明，可以在运行时确定。
* 3、const 的值在编译时确定，static 的值在运行时确定。
*/

// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}

/*
    &'static str 可以拆成两部分看：

    &str：字符串切片引用（借用，不拥有数据）
    'static：这个引用的生命周期是整个程序运行期
    所以这句：

    static LANGUAGE: &'static str = "Rust";

    意思是：

    LANGUAGE 是一个全局静态变量
    它存的是一个字符串引用
    这个引用指向的字符串字面量 "Rust" 位于只读静态区，程序结束前一直有效
    再直观一点：

    普通引用：&'a str，只保证在某个作用域 'a 内有效
    &'static str：保证“从程序开始到结束都有效”
    常见来源（会是 'static）：

    字符串字面量："hello"（类型就是 &'static str）
    static/const 中引用的静态数据
    注意区分：

    &'static str：引用是 'static
    String：堆分配、拥有所有权，不自带 'static（除非你特意泄漏等非常规方式）
*/
