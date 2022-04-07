use std::fs;
use std::io::{Result, Read, Write};
use std::net::{TcpListener, TcpStream};

// handle_client 流处理函数
fn handle_client(mut stream: TcpStream) {
    // 创建一个 Buffer 缓冲区存储临处理响应数据
    let mut buffer = [0; 4096];
    // 将响应流内数据写入缓冲区
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 读取 index.html 文件数据
    let content = fs::read_to_string("index.html").unwrap();
    // 返回响应内容中
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    // 将响应转换为字节写入到流
    stream.write(response.as_bytes()).unwrap();
    // 手动的调用flush, 以确保数据已经写入
    stream.flush().unwrap();
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
                println!("Server error!");
             }
        }
    }

    // 默认返回空元组
    Ok(())
}
