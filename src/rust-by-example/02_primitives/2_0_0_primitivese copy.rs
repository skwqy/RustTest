
/**
标量类型（scalar type）
    1、有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
    2、无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
    3、浮点数（floating point）： f32、f64
    4、char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
    5、bool（布尔型）：只能是 true 或 false
    6、单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组
    
    尽管单元类型的值是个元组，它却并不被认为是复合类型，因为并不包含多个值。

内置复合类型（compound type）
    1、数组（array）：如 [1, 2, 3]
    2、元组（tuple）：如 (1, true)

自定义复合类型：
    3、结构体（struct）：如 struct Color { red: u8, green: u8, blue: u8 }
    4、枚举（enum）：如 enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(i32, i32, i32) }
    5、联合体（union）：如 union MyUnion { f1: u8, f2: f32 }
    6、切片（slice）：如 &[1, 2, 3]
    7、引用（reference）：如 &str
    8、智能指针（smart pointer）：如 Box<T>
    9、动态数组（dynamic array）：如 Vec<T>
    10、哈希表（hash map）：如 HashMap<K, V>
    11、集合（set）：如 HashSet<T>
    12、队列（queue）：如 Queue<T>
    13、堆（heap）：如 Heap<T>
    14、栈（stack）：如 Stack<T>
    15、图（graph）：如 Graph<T>
    16、树（tree）：如 Tree<T>
    17、链表（linked list）：如 LinkedList<T>
**/

fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
    mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}

