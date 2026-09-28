use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::{sync::LazyLock, time::Duration};
use tauri_plugin_http::reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Client, Error, IntoUrl, Method, RequestBuilder, Response,
};

use crate::http::cookie::HttpCookie;

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .cookie_provider(HttpCookie::get_jar())
    .gzip(true)
    .connect_timeout(Duration::from_secs(10))
    .read_timeout(Duration::from_secs(30))
    .build()
    .expect("构建 HTTP CLIENT 失败")
});

#[derive(Debug)]
pub struct HttpClient {
  builder: RequestBuilder,
}

impl HttpClient {
  /// ## 构建客户端
  ///
  /// ### 必选参数
  /// * `url` - 请求地址
  /// * `method` - 请求方法
  pub fn new<U: IntoUrl>(url: U, method: Method) -> Self {
    Self {
      builder: HTTP_CLIENT.request(method, url),
    }
  }

  /// ## 添加请求头
  ///
  /// ### 必选参数
  /// * `key` - 请求头名称
  /// * `value` - 请求头的值
  pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
    let Ok(key) = HeaderName::from_bytes(key.as_ref().as_bytes()) else {
      return self;
    };
    let Ok(value) = HeaderValue::from_bytes(value.as_ref().as_bytes()) else {
      return self;
    };

    self.builder = self.builder.header(key, value);
    self
  }

  /// ## 覆盖请求头
  ///
  /// ### 必选参数
  /// * `headers` - 请求头集合
  pub fn headers(mut self, headers: HeaderMap) -> Self {
    if headers.is_empty() {
      return self;
    }

    self.builder = self.builder.headers(headers);
    self
  }

  /// ## 设置请求参数
  ///
  /// ### 必选参数
  /// * `params` - 请求参数, 会拼接到 url 上
  pub fn params(mut self, params: Map<String, Value>) -> Self {
    if params.is_empty() {
      return self;
    }

    self.builder = self.builder.query(&params);
    self
  }

  /// ## 设置请求体
  ///
  /// ### 必选参数
  /// * `data` - 请求体, 放到 body 中
  pub fn data(mut self, data: Value) -> Self {
    self.builder = match data {
      Value::Null => return self,
      Value::String(data_str) => self.builder.body(data_str),
      Value::Object(data_obj) => self.builder.json(&data_obj),
      data => self.builder.body(data.to_string()),
    };

    self
  }

  /// ## 发送请求, 返回响应
  pub async fn send(self) -> Result<Response, Error> {
    self.builder.send().await
  }

  /// ## 发送请求, 返回反序列化的 json
  pub async fn json<R: DeserializeOwned>(self) -> Result<R, Error> {
    self.send().await?.json().await
  }

  /// ## 发送请求, 返回文本
  pub async fn text(self) -> Result<String, Error> {
    self.send().await?.text().await
  }

  /// ## 发送请求, 返回字节流
  pub async fn bytes(self) -> Result<Bytes, Error> {
    self.send().await?.bytes().await
  }
}
