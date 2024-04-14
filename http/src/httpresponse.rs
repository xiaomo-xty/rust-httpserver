use std::collections::HashMap;
use std::io::{Result, Write};


/**
# ResponseBody
RespnseBody 用于封装需要用不同方式处理的http body
 * ## Binary(Vec<u8>)
 * 用于二进制数据的封装。例如图片等用文本格式会损失信息的数据
```rust
if full_path.ends_with(".jpg") { 
    contents = match fs::read(full_path) {
            Ok(data) => Some(ResponseBody::Binary(data)),
            _ => None,
    };
}
```
 * 
## Text(String)
 * 用于文本数据的封装。例如js,html,css等文本数据
```rust
if full_path.ends_with(".html") { 
    contents = match fs::read_to_string(full_path) {
            Ok(txt) => Some(ResponseBody::Text(txt)),
            _ => None,
    };
}
```
 */
#[derive(Debug, PartialEq, Clone)]
pub enum ResponseBody {
    Binary(Vec<u8>),
    Text(String),
}

impl ResponseBody {
    /**
     * 将ResponseBody转成二进制数据，方便写入网络流
     */
    fn to_bytes(&self) -> Option<&[u8]> {
        match &self {
            ResponseBody::Text(txt) => Some(txt.as_bytes()),
            ResponseBody::Binary(data) => Some(&data)
        }
    }
}

/**
 * # HttpResponse
 * body 由[`ResponseBody`]封装
 */

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<ResponseBody>,
}



impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}




impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code : &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<ResponseBody>
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;
        response
    }

    /**
     * # Send reponse
     * Support:
     * - text data
     * - binary data
     */
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        // let response_string: String = String::from(res);
        let _ = write_stream.write_all(&res.to_bytes());
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }


    fn bodylen(&self) -> usize{
        let len = &self.body.clone().map(|b| {
            match b {
                ResponseBody::Text(txt) => txt.len(),
                ResponseBody::Binary(data) => data.len(),
            }
        });

        len.unwrap()
    }


    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let headers = format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n",
            &self.version(),
            &self.status_code(),
            &self.status_text(),
            &self.headers(),
            &self.bodylen(),
        );

        let _ = buffer.write_all(headers.as_bytes());
        let _ = buffer.write_all(self.body.as_ref().unwrap()
            .to_bytes().unwrap()
        );
        buffer
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200", 
            None, 
            Some(ResponseBody::Text("xxxx".into()))
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some(ResponseBody::Text("xxxx".into()))
        };
        assert_eq!(response_actual, response_expected);

    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404", 
            None, 
            Some(ResponseBody::Text("xxxx".into()))
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some(ResponseBody::Text("xxxx".into()))
        };
        assert_eq!(response_actual, response_expected);

    }

    // #[test]
    // fn test_http_response_creation() {
    //     let response_expected = HttpResponse {
    //         version: "HTTP/1.1",
    //         status_code: "404",
    //         status_text: "Not Found",
    //         headers: {
    //             let mut h = HashMap::new();
    //             h.insert("Content-Type", "text/html");
    //             Some(h)
    //         },
    //         body: Some(ResponseBody::Text("xxxx".into()))
    //     };
    //     let http_string: String = response_expected.into();
    //     let actual_string = 
    //         "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx";
    //     assert_eq!(http_string, actual_string);
    // }   
}