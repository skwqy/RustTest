fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn run() {
    println!("生命周期用于描述引用有效期之间的关系。");
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let r = longest(&s1, &s2);
    println!("longest = {}", r);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap_or("");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt = {:?}", excerpt);
    println!("excerpt.part = {}", excerpt.part);
}
