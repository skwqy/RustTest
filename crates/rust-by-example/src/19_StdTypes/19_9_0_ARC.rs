/**
 * 当线程之间所有权需要共享时，可以使用Arc（共享引用计数，Atomic Reference Counted 缩写）可以使用。
 * 这个结构通过 Clone 实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。
 * 由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除。
*/
use std::sync::Arc;
use std::thread;

fn main() {
    // 这个变量声明用来指定其值的地方。
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // 这里没有数值说明，因为它是一个指向内存堆中引用的指针。
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // 由于使用了Arc，线程可以使用分配在 `Arc` 变量指针位置的值来生成。
            println!("{:?}", apple);
        });
    }
}
/**
 * 核心区别就一句话：

    * 1. Rc<T>：单线程引用计数（Reference Counted），快，但不能跨线程
    * 2. Arc<T>：多线程原子引用计数（Atomic Reference Counted），可跨线程，但有原子操作开销
   * 再具体一点：
   * 
   * 线程安全
   *    Rc<T> 不是 Send/Sync
   *    Arc<T> 是线程安全的计数（前提 T 本身满足约束）
   * 性能
   *    Rc<T> 更轻量，适合单线程
   *    Arc<T> 因为原子操作会稍慢
   * 常见搭配
   *    可变共享：
   *        单线程：Rc<RefCell<T>>
   *        多线程：Arc<Mutex<T>> 或 Arc<RwLock<T>>
 */

