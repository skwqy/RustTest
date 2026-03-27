/*
    闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。这使得捕获（capture）能够灵活地适应用例，既可移动（move），又可借用（borrow）。
    闭包可以通过以下方式捕获变量：

        1、通过引用：&T
        2、通过可变引用：&mut T
        3、通过值：T
    闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式。
 */

 fn main() {
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}


