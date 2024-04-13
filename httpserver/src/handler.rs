use http::{httprequest::HttpRequst, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::default;
use std::env;
use std::fs;
// use std::hash::Hash;


pub trait Handler {
    fn handle(req: &HttpRequst) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
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
     * littlesun.cloud: index page
     * littlesun.cloud/health: 
     * littlesun.cloud/name.suffix:
     */
    fn handle(req: &HttpRequst) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css"){
                        map.insert("Content-Type", "text/css");
                    }
                    else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
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
    fn handle(req: &HttpRequst) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "characters" => {
                let body = Some(serde_json::to_string_pretty(&Self::load_json()).unwrap());
                println!("{}", body.as_ref().unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html"))
        }
    }
}