/*
   Iterator::any 是一个函数，若传给它一个迭代器（iterator），当其中任一元素满足谓词（predicate）时它将返回 true，否则返回 false
   （译注：谓词是闭包规定的， true/false 是闭包作用在元素上的返回值）。

   最关键是它和 iter / iter_mut 的区别：

    1、iter()：产出 &T（不可变借用）
    2、iter_mut()：产出 &mut T（可变借用）
    3、into_iter()：通常产出 T（拿走所有权，原值可能不能再用）
*/

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 举出 `i32`。
    // array2.into_iter().any(|x| x == 2) 
    // x 是引用类型（&i32），与 2（i32）比较时类型不匹配（E0277），所以需要解引用 *x
    // 官方例子是错误的❌️
    println!("2 in array2: {}", array2.into_iter().any(|x| *x == 2));
}


