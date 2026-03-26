trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - by {}", self.title, self.author)
    }
}

fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
}

fn pair_summaries<T: Summary>(a: &T, b: &T) {
    println!("A: {}", a.summarize());
    println!("B: {}", b.summarize());
}

pub fn run() {
    println!("Trait 类似“行为接口”，定义共享能力。");
    let a1 = Article {
        title: String::from("Rust Trait Guide"),
        author: String::from("Alice"),
    };
    let a2 = Article {
        title: String::from("Zero-cost Abstractions"),
        author: String::from("Bob"),
    };

    notify(&a1);
    pair_summaries(&a1, &a2);
}
