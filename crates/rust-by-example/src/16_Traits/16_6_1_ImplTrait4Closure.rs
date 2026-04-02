/**
*/
//更重要的是，某些 Rust 类型无法写出。
//例如，每个闭包都有自己未命名的具体类型。
//在使用 impl Trait 语法之前，必须在堆上进行分配才能返回闭包。
//但是现在你可以像下面这样静态地完成所有操作：
// 返回一个将输入和 `y` 相加的函数
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}

/**
 * move 的意思是：把闭包用到的外部变量所有权“搬”进闭包里（按值捕获），而不是默认的借用捕获。

    所以：

    let y = 10;
    let f = move |x: i32| { x + y };
    这里 y 会被复制/移动到闭包内部。

    如果外部变量实现了 Copy（如 i32），看起来像“复制了一份”
    如果不是 Copy（如 String），就是把所有权转移，外部就不能再用原变量了
    常见用途：

    让闭包脱离当前作用域后还能安全使用（比如线程）
    明确避免借用生命周期问题
    例如线程里几乎总要 move：

    let s = String::from("hi");
    std::thread::spawn(move || {
        println!("{s}");
    });
    // 这里不能再用 s
    
    不写 move 时，闭包会尽量借用外部变量（& 或 &mut），只有需要时才按值捕获。
 */
