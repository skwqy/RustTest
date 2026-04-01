use async_std::task;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
/**
 * 
  task::spawn();的作用是什么？是把这个future放到excutor上面执行吗？
  对，你理解得很对。
    task::spawn(fut) 的核心作用就是：
        1.把 fut（一个 Future）提交给 async runtime 的 executor
        2.让它在后台被调度执行（通常并发执行）
        3.spawn 会立刻返回一个 JoinHandle（你可以选择 .await 它拿结果，也可以不等）
    在你这个 3AsyncMain.rs 场景里，一般是：
        1.每来一个连接就 spawn(handle_connection(stream))
        2.主循环继续接收下一个连接
        3.不会因为某个连接（比如 /sleep）阻塞整个服务器
    简化理解：
        不用 spawn：当前任务自己 await，一个连接慢会拖住后续处理
        用 spawn：每个连接拆成独立任务，交给 executor 调度并发跑
    补一句细节：
        1.spawn 不是新建 OS 线程（不是 std::thread::spawn），而是创建一个异步任务，
          由 executor 在线程池/事件循环上调度。
 */
#[async_std::main]
async fn main() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 阻塞等待请求的进入
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        task::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

//127.0.0.1:7878/sleep
//127.0.0.1:7878
