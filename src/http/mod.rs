pub enum Route {
    PostChats,
    GetChats,
    PostMessage,
    GetMessages
}



pub struct JsonHttpRequest {
    body: String,
    route: Route
}

pub struct JsonHttpResponse {
    body: String,
    status: u8
}

impl JsonHttpResponse {
    pub fn to_raw_output(&self) -> String {
        format!("
            {}
            Content-Type: application/json
            
            {}", 
        self.status_code_to_http_response_string(),
        self.body)
    }

    fn status_code_to_http_response_string(&self) -> String {
        let status_code_message = match self.status {
            200 => "200 OK",
            _ => "500 Internal Server Error"
        };
        format!("HTTP/1.1 {}", status_code_message)
    }
}