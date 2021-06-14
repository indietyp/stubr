use asserhttp::*;
use serde_json::json;

use stubr::Stubr;

use crate::utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_response_headers() {
    let srv = given("record/resp-headers/one");
    isahc::get(srv.path("/headers/resp/one"))
        .expect_status_ok()
        .expect_header("x-a", "a");
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/headers/resp/one"))
        .expect_status_ok()
        .expect_header("x-a", "a");
    assert_recorded_stub_eq("headers-resp-one-357454623928053573", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/resp/one"
        },
        "response": {
            "status": 200,
            "headers": {
                "x-a": "a"
            }
        }
    }))
}

#[tokio::test(flavor = "multi_thread")]
async fn proxy_should_forward_many_response_headers() {
    let srv = given("record/resp-headers/many");
    isahc::get(srv.path("/headers/resp/many"))
        .expect_status_ok()
        .expect_header("x-a", "a")
        .expect_header("x-b", "b");
    Stubr::record_with(record_cfg()).isahc_client().get(srv.path("/headers/resp/many"))
        .expect_status_ok()
        .expect_header("x-a", "a")
        .expect_header("x-b", "b");
    assert_recorded_stub_eq("headers-resp-many-12494426098399125677", json!({
        "request": {
            "method": "GET",
            "urlPath": "/headers/resp/many"
        },
        "response": {
            "status": 200,
            "headers": {
                "x-a": "a",
                "x-b": "b"
            }
        }
    }))
}
