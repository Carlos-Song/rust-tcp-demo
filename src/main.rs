use std::io::Result;
use std::net::{TcpListener, TcpStream};

// handle_client 流处理函数
fn handle_client(_: TcpStream) {
    println!("Get some thing from client!")
}

fn main() -> Result<()> {
    // 构建一个 127.0.0.1:8080 listener
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("http://127.0.0.1:8080 is listening");

    // listener.incoming 调用返回接收内容的迭代器
    for stream in listener.incoming() {
        // 消费监听流即将到来的数据， 发生错误时打印错误信息
        match stream {
            Ok(stream) => {
                // 将处理成功的数据传入处理函数
                handle_client(stream)
            },
            Err(_) => {
                println!("Error!");
             }
        }
    }

    Ok(())
}
