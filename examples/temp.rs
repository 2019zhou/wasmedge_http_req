use std::convert::TryFrom;
use wasmedge_http_req::{request::{Request, Method, HttpVersion}, uri::Uri};


fn main() {
    let uri = Uri::try_from("http://localhost:3500/v1.0/state/statestore").unwrap();
    let mut writer = Vec::new(); //container for body of a response
    const body: &[u8] = br#"[{ "key": "name", "value": "Bruce"}]"#;
    // println!("{:?}", String::from_utf8_lossy(body));

    let response = Request::new(&uri)
        .method(Method::POST)
        .version(HttpVersion::Http11)
        .header("Content-Length", &body.len())
        .header("Content-Type", "application/json")
        .body(body)
        .send(&mut writer)
        .unwrap();

}
