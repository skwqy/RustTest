#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

pub fn run() {
    println!("枚举可以表示同一类型下的多种可能。");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    route(loopback);

    println!("Option<T> 表示值可能存在或不存在。");
    let maybe_num: Option<i32> = Some(10);
    match maybe_num {
        Some(v) => println!("Some({})", v),
        None => println!("None"),
    }

    println!("if let 适合只关心一种分支。");
    let config = Some("dark");
    if let Some(mode) = config {
        println!("当前模式: {}", mode);
    }
}
