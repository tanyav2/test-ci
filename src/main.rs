use reqwest::{
    header::{HeaderMap, ACCEPT, CACHE_CONTROL, CONTENT_TYPE},
    Client, Response,
};

fn main() {
    println!("Big if true");
    let _expected_configs = vec![
        0, 55, 255, 3, 0, 15, 0, 32, 51, 0, 68, 86, 0, 7, 1, 32, 4, 5, 7, 8, 9, 255, 2, 0, 13, 0,
        32, 51, 0, 68, 86, 0, 5, 1, 32, 4, 5, 7, 255, 3, 0, 15, 0, 32, 51, 0, 68, 86, 0, 7, 1, 32,
        4, 5, 7, 8, 9,
    ];
}
