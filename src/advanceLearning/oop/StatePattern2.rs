/**
 * 状态模式的一个缺点是因为状态实现了状态之间的转换，一些状态会相互联系。
 * 如果在 PendingReview 和 Published 之间增加另一个状态，比如 Scheduled，
 * 则不得不修改 PendingReview 中的代码来转移到 Scheduled。
 * 如果 PendingReview 无需因为新增的状态而改变就更好了，不过这意味着切换到另一种设计模式。
 * 
 * 另一个缺点是我们会发现一些重复的逻辑。
 * 为了消除它们，可以尝试为 State trait 中返回 self 的 request_review 和 approve 方法增加默认实现；
 * 然而这样做行不通：当将 State 用作 trait 对象时，trait 并不知道 self 具体是什么类型，
 * 因此无法在编译时确定返回类型。（这是前面提到的 dyn 兼容性规则之一。）
 * 
 * 另一个重复是 Post 中 request_review 和 approve 这两个类似的实现。
 * 它们都会对 Post 的 state 字段调用 Option::take，如果 state 为 Some，
 * 就将调用委托给封装值的同名方法，并将返回结果重新赋值给 state 字段。
 * 如果 Post 中的很多方法都遵循这个模式，我们可能会考虑定义一个宏来消除重复
 */

// 改进方案： 将状态和行为编码为类型
// Post 和 DraftPost 结构体都有一个私有的 content 字段来储存博文的文本。
// 这些结构体不再有 state 字段因为我们将状态编码改为结构体类型本身。
// Post 将代表发布的博文，它有一个返回 content 的 content 方法。
// DraftPost 将代表草稿博文，它有一个添加文本的 add_text 方法。

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    //error[E0599]: no method named `content` found for reference `&DraftPost` in the current scope
    // println!("draft post = {:?}", post.content());

    let post = post.request_review();
    // error[E0599]: no method named `content` found for reference `&PendingReviewPost` in the current scope
    // println!("review post = {:?}", post.content());

    let post = post.approve();
    println!("approved post = {:?}", post.content());

    assert_eq!("I ate a salad for lunch today", post.content());
}