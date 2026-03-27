use List::*;

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
/**
Box 在 Cons(u32, Box<List>) 里的作用：把下一个 List 节点放到堆（heap）上，并用一个固定大小的指针来引用它。

为什么必须这样做？
因为 enum List 是递归类型：

Cons 里面如果直接写 List，类型会“无限嵌套”，编译器无法知道大小
Box<List> 的大小是固定的（本质是一个指针），所以 List 的大小就能确定下来
可以理解成：

Cons(value, next)
value 是当前节点值
next 是指向下一个节点的“盒子指针”
示意：

enum List {
    Cons(u32, Box<List>),
    Nil,
}
Nil 是链表结尾
Box::new(...) 用来创建下一个节点并放到堆上


可以，按你这个实现（prepend + Box<List>）构造完后，逻辑结构是：

list = Cons(3, Box(Cons(2, Box(Cons(1, Box(Nil))))))

可以把它想成下面这样。

结构示意（值关系）
list
 └─ Cons(3, ptrA)
ptrA ──> Cons(2, ptrB)
ptrB ──> Cons(1, ptrC)
ptrC ──> Nil
栈 / 堆示意（简化）
栈（stack）:
+--------------------------------------+
| list: List = Cons(3, ptrA)           |
+--------------------------------------+
堆（heap）:
+---------------------------+      +---------------------------+      +-------------------+
| A: List = Cons(2, ptrB)   | ---> | B: List = Cons(1, ptrC)   | ---> | C: List = Nil      |
+---------------------------+      +---------------------------+      +-------------------+
说明：

栈上的 list 是当前头节点（值为 3），里面有一个指针 ptrA
ptrA/ptrB/ptrC 都是 Box<List> 持有的堆地址
每个 Cons 节点包含：
一个 u32（当前值）
一个 Box<List>（指向下一个节点）
Nil 是终止节点（递归 base case）
补一句你最关心的点：
Box 的存在让“下一个节点”变成固定大小指针，所以 List 类型大小可确定，递归类型才能成立。
*/