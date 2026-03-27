/*
   编译器提供了 dead_code（死代码，无效代码）lint，这会对未使用的函数产生警告。可以用一个属性来禁用这个 lint。
*/

fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// 改正 ^ 增加一个属性来消除警告

fn main() {
    used_function();
}
