use http::{httprequest::HttpRequst, httpresponse::HttpResponse, httpresponse::ResponseBody};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::default;
use std::env;
use std::fs;
// use std::hash::Hash;


pub trait Handler {
    fn handle(req: &HttpRequst) -> HttpResponse;

    /**
     * # 文件加载
     * 根据不同的文件类型选择不同的Body
     * 对于文本文件返回`ResponseBody::Text(String)`
     * 对于图片等文件返回`ResponseBody::Binary(Vec[u8])`
     */
    fn load_file(file_name: &str) -> Option<ResponseBody> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let mut contents : Option<ResponseBody> = None;
        if full_path.ends_with(".jpg") || full_path.ends_with(".zip") { 
            contents = match fs::read(full_path) {
                  Ok(data) => Some(ResponseBody::Binary(data)),
                  _ => None,
            };
        }
        else {
            contents = match fs::read_to_string(full_path) {
                Ok(text) => Some(ResponseBody::Text(text)),
                Err(_) => None,
            };
        }

        contents
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct  WebServiceHandler;


#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    name: String,
    level: i32,
    health: f32,
    element: String,  
    skills: Vec<String>,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequst) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}


impl Handler for StaticPageHandler {
    /**
     * # 静态页面处理
     * 接受[`Router`]放过来的静态页面路由路径，找到相应文件，读取并返回
     * 
     * # Example
     ```rust
     match &req.resource {
        httprequest::Resource::Path(s) => {
            println!("Path: \"{}\"", s);
            let route: Vec<&str> = s.split("/").collect();
            match route[1] {
                "api" => WebServiceHandler::handle(&req),

                _ => StaticPageHandler::handle(&req)
            }
        }
    },
     ```
     */
    fn handle(req: &HttpRequst) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),

            //加载文件并判断类型，插入相应类型到报文头（或许我应该将这一部分逻辑抽出来，搞个表什么查出来会比较优雅）
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css"){
                        map.insert("Content-Type", "text/css");
                    }
                    else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else if path.ends_with(".jpg") || path.ends_with(".png") {  
                        map.insert("Content-Type", "image/jpeg"); // 对于.png，这里应该是"image/png"  
                    } else if path.ends_with(".zip") || path.ends_with(".png") {  
                        map.insert("Content-Type", "application/zip");
                    } 
                    else {
                        map.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html"))
            }
        }
    }
}



impl  WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "characters.json");
        println!("full_path:{}", full_path);
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents
                .unwrap()
                .as_str()
            )
            .unwrap();
        orders
    }
     
}

impl Handler for WebServiceHandler {
    /**
     * # 网页服务处理
     * 根据路由路径，对一些api进行响应
     * 目前的实现就是读取json并返回
     * 
     * # Example
     * [character](http://localhost:3000/api/shipping/characters)
     */
    fn handle(req: &HttpRequst) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "characters" => {
                let body = Some(serde_json::to_string_pretty(&Self::load_json()).unwrap());
                println!("{}", body.as_ref().unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), Some(ResponseBody::Text(body.unwrap())))
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html"))
        }
    }
}