use crate::config::{AppConfig, BASE_URL, RESULT_ENDPOINT};
use crate::exceptions::{AppError, AppResult};
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use std::sync::Arc;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    _config: AppConfig,
}

impl HttpClient {
    pub fn new(config: AppConfig) -> Self {
        let cookie_jar = Jar::default();
        let client = Client::builder()
            .cookie_provider(Arc::new(cookie_jar))
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            _config: config,
        }
    }

    pub async fn get_homepage(&self) -> AppResult<String> {
        let url = Url::parse(BASE_URL).expect("Invalid BASE_URL");
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;
        resp.text()
            .await
            .map_err(|e| AppError::Network(e.to_string()))
    }

    pub async fn submit_result_form(
        &self,
        form_data: &std::collections::HashMap<&str, &str>,
    ) -> AppResult<String> {
        let url =
            Url::parse(&format!("{BASE_URL}/{RESULT_ENDPOINT}")).expect("Invalid RESULT_ENDPOINT");
        let resp = self
            .client
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
