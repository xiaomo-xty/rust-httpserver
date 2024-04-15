use http::{httprequest::HttpRequst, httpresponse::HttpResponse, httpresponse::ResponseBody};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::default;
use std::env;
use std::fs;
// use std::path;
use std::path::PathBuf;
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

        let contents : Option<ResponseBody> = 
        if let Some(ext) = PathBuf::from(full_path.clone()).extension().and_then(|ext| ext.to_str()) {
            match ext {  
                "html" | "css" | "js" |"xml" |"json" |"txt" => {
                    match fs::read_to_string(full_path) {
                        Ok(txt) => Some(ResponseBody::Text(txt)),
                        _ => None,
                    }
                },
                _ => {
                    match fs::read(full_path) {
                        Ok(data) => Some(ResponseBody::Binary(data)),
                        _ => None,
                  }
                },
            }
        } else {None};
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
        let mut file_name = match route[1] {
            "" => "index".to_string(),
            file_name => file_name.to_string(),
        };

        let mut headers = HashMap::new();
        let mut content_type = "text/html";


        let binding = PathBuf::from(file_name.clone());
        if binding.extension().is_none() {
            file_name.push_str(".html");
        }
        // 尝试从文件名获取扩展名，并设置适当的Content-Type  
        else if let Some(ext) = binding.extension().and_then(|ext| ext.to_str()) {  
            match ext {  
                "html" => content_type = "text/html",
                "css" => content_type = "text/css",  
                "js" => content_type = "text/javascript",  
                "xml" => content_type = "text/xml",
                "json" => content_type = "text/json",
                "txt" => content_type = "text/plain",  
                "gif" => content_type = "image/gif",  
                "png" => content_type = "image/png",  
                "jpg" | "jpeg" => content_type = "image/jpeg",  
                "zip" => content_type = "application/zip",
                "mp3" => content_type = "application/map3", 
                "mp4" => content_type = "application/map4", 
                _ => {  
                    // 如果不支持的扩展名，则重定向到nosupport页面  
                    file_name = "nosupport.html".to_string();  
                    content_type = "text/html";  
                }  
            }  
        }  
        
        headers.insert("Content-Type", content_type);  

        match Self::load_file(&file_name) {
            None => HttpResponse::new("404", None, Self::load_file("404.html")),
            Some(body) => HttpResponse::new("200", Some(headers), Some(body)),
        }
    }
}



impl  WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "characters.json");
        // println!("full_path:{}", full_path);
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