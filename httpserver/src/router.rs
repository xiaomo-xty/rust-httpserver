use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler,};
use http::{httprequest, httprequest::HttpRequst};
use std::io::prelude::*;

/**
 * 我想应该可以通过读取一些配置文件来达到路由设置的目的，现在先简单硬编码路由
 */
pub struct Router;


impl Router {
    /**
     * Router: 对不同的请求进行不同的相应
     */
    pub fn route(req: HttpRequst, stream :&mut impl Write) -> () {
        let resp =
        match req.method {
            httprequest::Method::Get => 
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
            _ => PageNotFoundHandler::handle(&req),
        };


        let _ = resp.send_response(stream);
    }
}