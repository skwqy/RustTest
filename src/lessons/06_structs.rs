#[derive(Debug)]
struct User {
    username: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(u8, u8, u8); // 元组结构体

#[derive(Debug)]
struct AlwaysEqual; // 单元结构体

impl User {
    fn deactivate(&mut self) {
        self.active = false;
    }
}

pub fn run() {
    println!("结构体用于把相关数据组织在一起。");
    let mut user = User {
        username: String::from("rustacean"),
        active: true,
        sign_in_count: 1,
    };
    println!("初始 user: {:?}", user);

    user.sign_in_count += 1;
    user.deactivate();
    println!("更新后 user: {:?}", user);
    println!("用户名字段读取: {}", user.username);

    let black = Color(0, 0, 0);
    println!("元组结构体 Color: {:?}", black);

    let marker = AlwaysEqual;
    println!("单元结构体 marker: {:?}", marker);
}
