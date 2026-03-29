use std::{
    cmp::max,
    mem::{align_of, size_of},
    ptr,
};

struct Carton<T>(ptr::NonNull<T>);

impl<T> Carton<T> {
    pub fn new(value: T) -> Self {
        // 在堆上分配足够的可以存储一个类型 T 大小的空间
        assert_ne!(
            size_of::<T>(),
            0,
            "Zero-sized types are out of the scope of this example"
        );
        let mut memptr: *mut T = ptr::null_mut();
        unsafe {
            let ret = libc::posix_memalign(
                (&mut memptr as *mut *mut T).cast(),
                max(align_of::<T>(), size_of::<usize>()),
                size_of::<T>(),
            );
            assert_eq!(ret, 0, "Failed to allocate or invalid alignment");
        };

        // NonNull 仅仅是对于指针的一层封装，强制要求指针是非空的
        let ptr = {
            // 安全保证：因为我们从一个引用创建了 memptr，并且独占了所有权，所以可以解引用
            ptr::NonNull::new(memptr.cast::<T>())
                .expect("Guaranteed non-null if posix_memalign returns 0")
        };

        // 将数据从栈上复制到堆上
        unsafe {
            // 安全保证：如果 ptr 是非空的，posix_memalign 会返回一个已经内存对齐的有效的可写指针
            ptr.as_ptr().write(value);
        }

        Self(ptr)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // 安全保证：self 指针已经内存对齐，并且初始化了, 在 `Self::new` 方法中已经解引用，
            // 我们要求 readers 引用 Carton，而这里返回值的生命周期和输入的 self 的生命周期对齐，
            // 因此 borrow checker 会强制保证这一点：
            // 直到这个引用被 drop，不能修改 Carton 中的内容
            self.0.as_ref()
        }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // 安全保证：self 指针已经内存对齐，并且初始化了, 在 `Self::new` 方法中已经解引用，
            // 我们要求 writer 可写引用 Carton，而这里返回值的生命周期和输入的 self 的生命周期对齐，
            // 因此 borrow checker 会强制保证这一点:
            // 直到这个引用被 drop，不能访问 Carton 中的内容
            self.0.as_mut()
        }
    }
}

// 安全保证：除了我们没有人拥有Carton中的裸指针，因此，只需要T可以Send，Carton就可以Send
unsafe impl<T> Send for Carton<T> where T: Send {}

// 安全保证：存在将 `&Carton<T>` 转变为 `&T` 的公开 API，
// 而这些 API 是 unsynchronized 的（比如 `Deref`），
// 因此只有在T是 `Sync` 的情况下，`Carton<T>` 才可以是 `Sync` 的，
// 反过来说，`Carton` 本身没有使用到任何 `内部可变性`，
// 所有可变引用都只能通过独占的方式获取 (`&mut`)，
// 这也就意味着 `T` 的 `Sync` 特性可以传递给 `Carton<T>`
unsafe impl<T> Sync for Carton<T> where T: Sync  {}
// 删除了重复的 Send 实现：
// `unsafe impl<T> Send where T: Send` 已经覆盖了语义，再写一份会产生 trait 冲突。


impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe {
            // 先显式 drop 掉堆上的 T，确保 T 的析构逻辑（比如 String/Vec 的资源释放）被执行。
            // 如果只 free 内存而不 drop_in_place，会造成逻辑资源泄漏。
            ptr::drop_in_place(self.0.as_ptr());
            // 再释放由 posix_memalign 申请的原始内存，顺序必须在 drop T 之后。
            libc::free(self.0.as_ptr().cast());
        }
    }
}

fn main() {
    // 最小示例：验证 Deref/DerefMut 能像 Box 一样读取与修改值。
    let mut carton = Carton::new(String::from("hello"));
    assert_eq!(carton.len(), 5); // 通过 Deref 调用 String 的方法
    carton.push_str(" carton"); // 通过 DerefMut 修改堆上的值
    println!("Carton contains: {}", *carton);
    // main 结束时会自动调用 Carton::drop：
    // 先 drop 堆上的 T，再 free 原始堆内存。
}

#[cfg(test)]
mod tests {
    use super::Carton;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct DropCounter<'a> {
        dropped: &'a AtomicUsize,
    }

    impl<'a> Drop for DropCounter<'a> {
        fn drop(&mut self) {
            self.dropped.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn deref_and_deref_mut_work() {
        let mut carton = Carton::new(41usize);
        assert_eq!(*carton, 41);
        *carton += 1;
        assert_eq!(*carton, 42);
    }

    #[test]
    fn drop_runs_inner_value_destructor_once() {
        let dropped = AtomicUsize::new(0);
        {
            let _carton = Carton::new(DropCounter { dropped: &dropped });
            // 作用域内尚未 drop
            assert_eq!(dropped.load(Ordering::SeqCst), 0);
        }
        // 作用域结束后，Carton::drop 会先 drop T；这里验证析构确实发生且只发生一次。
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }
}
