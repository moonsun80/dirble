use std::sync::Arc;
use url::Url;

use super::mock_server;
use crate::arg_parse::GlobalOpts;
use crate::request::{self, RequestResponse};

#[test]
fn test_200_response() {
    mock_server();


    let global_opts: GlobalOpts = Default::default();
    let global_opts = Arc::new(global_opts);
    let mut easy = request::generate_easy(&global_opts);

    let url = Url::parse(super::URL).unwrap();

    let rr = request::make_request(&mut easy, url);

    println!("{:?}", rr);

    assert_eq!(
        rr,
        RequestResponse {
            url: Url::parse("http://[::1]:3000/").unwrap(),
            code: 200,
            content_len: 2,
            is_directory: false,
            is_listable: false,
            redirect_url: "".into(),
            found_from_listable: false,
            parent_index: 0,
            parent_depth: 0,
        },
        "Response not recognised :(",
    );
}

#[test]
fn test_301_response() {
    mock_server();

    let global_opts: GlobalOpts = Default::default();
    let global_opts = Arc::new(global_opts);
    let mut easy = request::generate_easy(&global_opts);

    let url = Url::parse(super::URL)
        .unwrap()
        .join("301.html")
        .unwrap();

    let rr = request::make_request(&mut easy, url);

    println!("{:?}", rr);

    assert_eq!(
        rr,
        RequestResponse {
            url: Url::parse("http://[::1]:3000/301.html").unwrap(),
            code: 301,
            content_len: 8,
            is_directory: false,
            is_listable: false,
            redirect_url: "http://[::1]:3000/301-target.html".into(),
            found_from_listable: false,
            parent_index: 0,
            parent_depth: 0,
        },
        "Response not recognised :(",
    );

}
