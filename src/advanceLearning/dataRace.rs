use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
fn main() {
    let data = vec![1, 2, 3, 4];
    // 使用 Arc，这样即使程序执行完毕，存储 AtomicUsize 的内存依然存在，
    // 否则由于 thread::spawn 的生命周期限制，Rust 不会为我们编译这段代码
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    // `move` 捕获了 other_idx 的值，将它移入这个线程
    thread::spawn(move || {
        // 因为这是一个原子变量，不存在数据竞争问题，所以可以修改 other_idx 的值
        other_idx.fetch_add(10, Ordering::SeqCst);
    });

    // 因为我们只读取了一次原子的内存，因此用原子中的值做索引是安全的，
    // 然后将读出的值的拷贝传递给 Vec 做为索引，
    // 索引过程可以做正确的边界检查，并且在执行索引期间这个值也不会发生改变。
    // 但是，如果上面的线程在执行这句代码之前增加了这个值，这段代码会 panic。
    // 因为程序的正确执行（panic 几乎不可能是正确的），所以这就是一个 *竞态*，
    // 其执行结果依赖于线程的执行顺序
    println!("{}", data[idx.load(Ordering::SeqCst)]);
    //println!("{}", other_idx.load(Ordering::SeqCst));
}
