/**
 * 当发生 panic 时，程序会打印出错误信息，并退出程序。
 * 在大多数情况下，panic 是一种显式地失败（fail）的好方法。
 * 在原型开发中这很有用，比如 用来测试还没有实现的函数，不过这时使用 unimplemented 更能表达意图。
 * 另外在 测试中，panic 是一种显式地失败（fail）的好方法。
*/
fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}


