use serde::Deserialize;
use serde_json::{Map, Value};
use std::{
  collections::HashMap,
  str::FromStr,
  sync::{Arc, LazyLock, OnceLock},
};
use tauri_plugin_http::reqwest::{
  cookie::{CookieStore, Jar},
  header::{HeaderMap, HeaderName, HeaderValue},
  Client, Method, Response, Url,
};

use crate::http::{config::HttpConfig, lib::KgCookies, server::BASE_URL};

/// HTTP 客户端实例
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
/// Cookie jar
static COOKIE_JAR: LazyLock<Arc<Jar>> = LazyLock::new(|| Arc::new(Jar::default()));

#[derive(Debug, Default)]
pub struct HttpRequestOptions {
  url: String,
  method: Method,
  header: HeaderMap,
  params: Map<String, Value>,
  data: Value,
}

impl HttpRequestOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn url(mut self, url: impl Into<String>) -> Self {
    self.url = url.into();
    self
  }

  pub fn method(mut self, method: Method) -> Self {
    self.method = method;
    self
  }

  pub fn header(mut self, header: HeaderMap) -> Self {
    self.header = header;
    self
  }

  pub fn add_header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    let Ok(name) = HeaderName::from_bytes(key.as_ref().as_bytes()) else {
      return self;
    };

    let Ok(value) = HeaderValue::from_str(value.as_ref()) else {
      return self;
    };

    self.header.insert(name, value);
    self
  }

  pub fn params(mut self, params: Map<String, Value>) -> Self {
    self.params = params;
    self
  }

  pub fn add_param(mut self, key: impl Into<String>, value: Value) -> Self {
    self.params.insert(key.into(), value);
    self
  }

  pub fn data(mut self, data: Value) -> Self {
    self.data = data;
    self
  }
}

pub struct HttpRequest;

impl HttpRequest {
  pub fn get_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
      let cookie_jar = COOKIE_JAR.clone();

      Client::builder()
        .cookie_provider(cookie_jar)
        .gzip(true)
        .build()
        .unwrap_or_default()
    })
  }

  pub async fn request(opts: HttpRequestOptions) -> anyhow::Result<Response> {
    let mut request_builder = Self::get_client().request(opts.method, &opts.url);

    if !opts.header.is_empty() {
      request_builder = request_builder.headers(opts.header);
    }
    if !opts.params.is_empty() {
      request_builder = request_builder.query(&opts.params);
    }
    if !opts.data.is_null() {
      request_builder = match opts.data {
        Value::String(s) => request_builder.body(s),
        Value::Object(_) => request_builder.json(&opts.data),
        _ => request_builder.body(opts.data.to_string()),
      }
    }

    let response = request_builder.send().await?;

    Ok(response)
  }

  pub async fn get(url: String) -> anyhow::Result<Response> {
    let resp = Self::get_client().get(url).send().await?;

    Ok(resp)
  }

  pub async fn get_json<T>(url: String) -> anyhow::Result<T>
  where
    T: for<'de> Deserialize<'de>,
  {
    let resp = Self::get_client().get(url).send().await?;
    let res_json = resp.json::<T>().await?;

    Ok(res_json)
  }

  pub fn get_cookies(url: &str) -> Option<HeaderValue> {
    let url = url
      .parse::<Url>()
      .unwrap_or_else(|_| Url::from_str(BASE_URL).unwrap());

    COOKIE_JAR.cookies(&url)
  }

  pub fn add_cookie(url: &str, key: &str, value: &str) {
    let url = url
      .parse::<Url>()
      .unwrap_or_else(|_| Url::from_str(BASE_URL).unwrap());

    let cookie_str = format!("{key}={value}; Path=/; Domain=.kugou.com");
    COOKIE_JAR.add_cookie_str(&cookie_str, &url);
  }

  pub fn set_cookies(url: &str, cookies: HashMap<String, String>) {
    let url = url
      .parse::<Url>()
      .unwrap_or_else(|_| Url::from_str(BASE_URL).unwrap());

    for (key, value) in cookies {
      let cookie_str = format!("{key}={value}; Path=/; Domain=.kugou.com");
      COOKIE_JAR.add_cookie_str(&cookie_str, &url);
    }
  }

  pub fn clear_cookies(url: &str) {
    let url = url
      .parse::<Url>()
      .unwrap_or_else(|_| Url::from_str(BASE_URL).unwrap());

    let default_cookies = KgCookies::default();
    for (key, value) in default_cookies.to_hashmap() {
      let cookie_str = format!("{key}={value}; Path=/; Domain=.kugou.com");
      COOKIE_JAR.add_cookie_str(&cookie_str, &url);
    }
  }
}
