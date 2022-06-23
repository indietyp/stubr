use std::path::PathBuf;
use reqwest::Client;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RecordConfig {
    /// Port number the recording proxy server is listening on.
    /// Defaults to a random one.
    pub port: Option<u16>,
    /// Directory where recorded stubs will be written.
    /// Defaults to 'target/stubs'
    pub output: Option<PathBuf>,
    /// Do not record those request headers
    pub except_request_headers: Option<Vec<&'static str>>,
    /// Do not record those response headers
    pub except_response_headers: Option<Vec<&'static str>>,
    /// Custom client for proxy requests, this is useful in cases
    /// where you would want to disable or enable certain features in the deployed reverse
    /// proxy making the actual requests to the remote server.
    pub client: Option<Client>
}

impl RecordConfig {
    const HOST_HEADER: &'static str = "host";
    const USER_AGENT_HEADER: &'static str = "user-agent";
}

impl Default for RecordConfig {
    fn default() -> Self {
        Self {
            port: None,
            output: None,
            except_request_headers: Some(vec![Self::HOST_HEADER, Self::USER_AGENT_HEADER]),
            except_response_headers: None,
            client: None
        }
    }
}