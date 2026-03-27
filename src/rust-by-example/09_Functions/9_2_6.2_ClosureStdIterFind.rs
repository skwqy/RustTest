/*
   Iterator::find 是一个函数，在传给它一个迭代器时，将用 Option 类型返回第一个满足谓词的元素。

   最关键是它和 iter / iter_mut 的区别：

    1、iter()：产出 &T（不可变借用）
    2、iter_mut()：产出 &mut T（可变借用）
    3、into_iter()：通常产出 T（拿走所有权，原值可能不能再用）
*/

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    // x 实际是引用类型，和 2（i32）比较类型不匹配（E0277），所以需要解引用 *x
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| *x == 2));
}



