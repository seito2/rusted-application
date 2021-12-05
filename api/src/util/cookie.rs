use actix_web::{cookie::Cookie, HttpRequest};
use serde::Deserialize;

const KEY: &str = "cookie_test";

fn get_cookie_string_from_header(req: HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        let cookie_string = v.to_str().unwrap();
        return Some(String::from(cookie_string));
    }

    return None;
}

fn get_cookie_value(key: &str, cookie_string: String) -> Option<String> {
    let key_vector: Vec<&str> = cookie_string.split(';').collect();

    for cookie in key_vector {
        match Cookie::parse(cookie) {
            Ok(key_vector) => {
                if key == key_vector.name() {
                    return Some(String::from(key_vector.value()));
                }
            }

            Err(e) => {
                println!("cookie parse error. -> {}", e);
            }
        }
    }

    return None;
}

#[derive(Deserialize)]
pub struct CookieQuery {
    pub value: String,
}
