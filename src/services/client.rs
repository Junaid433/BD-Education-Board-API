use crate::config::{AppConfig, BASE_URL, RESULT_ENDPOINT};
use crate::exceptions::{AppError, AppResult};
use reqwest::cookie::Jar;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Url};
use std::sync::Arc;
use std::time::Duration;

pub struct HttpClient {
    config: AppConfig,
    default_headers: HeaderMap,
}

impl HttpClient {
    pub fn new(config: AppConfig) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
        );
        default_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        default_headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) EduBoardAPI/1.1"),
        );

        Self { config, default_headers }
    }

    fn build_session_client(&self) -> AppResult<Client> {
        let cookie_jar = Jar::default();
        Client::builder()
            .cookie_provider(Arc::new(cookie_jar))
            .default_headers(self.default_headers.clone())
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .pool_idle_timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))
    }

    pub async fn get_homepage(&self) -> AppResult<(Client, String)> {
        let client = self.build_session_client()?;
        let url = Url::parse(BASE_URL).expect("Invalid BASE_URL");
        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;
        let body = resp
            .text()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;
        Ok((client, body))
    }

    pub async fn submit_result_form<T: serde::Serialize>(&self, client: &Client, form_data: &T) -> AppResult<String> {
        let url =
            Url::parse(&format!("{BASE_URL}/{RESULT_ENDPOINT}")).expect("Invalid RESULT_ENDPOINT");
        let resp = client
            .post(url)
            .form(form_data)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;
        resp.text()
            .await
            .map_err(|e| AppError::Network(e.to_string()))
    }
}
