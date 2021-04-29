use super::method::Method;

    pub struct Request {
        path: String,
        // can be None or Some(String)
        query_string: Option<String>,
        method: Method,
    }