use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    url: String,
    headers: HeaderMap,
    timeout: Option<u64>,
}

impl HttpClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
            headers: HeaderMap::new(),
            timeout: None,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
    let name = HeaderName::from_bytes(key.as_bytes()).unwrap();
    let val = HeaderValue::from_str(value).unwrap();

    self.headers.insert(name, val);
    self
}

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(secs);
        self
    }

    pub async fn text(self) -> Result<String, reqwest::Error> {
        let mut req = self.client.get(&self.url).headers(self.headers);

        if let Some(t) = self.timeout {
            req = req.timeout(Duration::from_secs(t));
        }

        let res = req.send().await?;
        res.text().await
    }

    pub async fn json<T: Serialize>(self, body: &T) -> Result<String, reqwest::Error> {
        let mut req = self.client.post(&self.url).headers(self.headers);

        if let Some(t) = self.timeout {
            req = req.timeout(Duration::from_secs(t));
        }

        let res = req.json(body).send().await?;
        res.text().await
    }

    pub async fn fetch_json<T: serde::de::DeserializeOwned>(
        self
    ) -> Result<T, reqwest::Error> {
        let mut req = self.client.get(&self.url).headers(self.headers);

        if let Some(t) = self.timeout {
            req = req.timeout(Duration::from_secs(t));
        }

        let res = req.send().await?;
        res.json::<T>().await
    }
}

pub fn get(url: &str) -> HttpClient {
    HttpClient::new(url)
}

pub fn post(url: &str) -> HttpClient {
    HttpClient::new(url)
  }
