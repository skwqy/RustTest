/*
   虽然这看起来像是一个抽象的概念，但实际上这非常有用且方便。
   这种类型的主要优点是它可以被转换为任何其他类型，从而可以在需要精确类型的地方使用，
   例如在 match 匹配分支。 这允许我们编写如下代码：

   这也是永远循环（如 loop {}）的函数（如网络服务器）或终止进程的函数（如 exit()）的返回类型。
*/

fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}




