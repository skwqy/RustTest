/*
    约束也可以使用 where 分句来表达，它放在 { 的前面，而不需写在类型第一次出现之前。
    另外 where 从句可以用于任意类型的限定，而不局限于类型参数本身。
*/

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
// 或者改用另一种间接的方法。
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
