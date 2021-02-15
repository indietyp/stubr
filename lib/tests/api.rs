use std::path::{Path, PathBuf};

use surf::get;

use stubr::Config;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_start_server_from_relative_pathbuf() {
    let srv = Stubr::start(PathBuf::from("tests/stubs/ping.json")).await;
    get(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_start_server_from_relative_path() {
    let srv = Stubr::start(Path::new("tests/stubs/ping.json")).await;
    get(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_start_server_from_relative_path_as_str() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn should_start_server_on_dedicated_port() {
    let cfg = Config { port: Some(60_000) };
    Stubr::start_with("tests/stubs/ping.json", cfg).await;
    get("http://127.0.0.1:60000").await.unwrap().assert_ok();
}