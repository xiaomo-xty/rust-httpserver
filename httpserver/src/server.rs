use super::router::Router;
use http::httprequest::HttpRequst;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    /**
     * 接受一个socket地址，返回一个Server
     * 
     ## Example
     ```rust
     let server = Server::new("localhost:3000");
     ```
     */
    pub fn new(socket_addr: &'a str) -> Self {
        Server {
            socket_addr
        }
    }
    

    /**
     * 启动server，开始监听`socket_addr`并处理请求
     ## Example
     ```rust
     server.run();
     ```
     */
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);


        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 1024*16];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequst = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}