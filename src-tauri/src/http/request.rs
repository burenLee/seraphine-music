use chrono::Utc;
use serde_json::{json, Map, Value};
use std::fmt::Debug;
use tauri_plugin_http::reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Method,
};

use crate::{
  http::{
    client::HttpClient,
    config::{DynamicModeConfig, HttpConfig, StaticConfigMode, KG_BASE_URL},
    cookie::{HttpCookie, ModeCookies},
  },
  utils::helper::{sign_key, sign_params_android, sign_params_register, sign_params_web},
};

#[derive(Debug, Clone, Copy)]
pub enum EncryptType {
  Web,
  Android,
  Register,
}

#[derive(Debug)]
pub struct HttpRequest {
  base_url: String,           // 基础URL
  url: String,                // 请求URL
  method: Method,             // 请求方法, 默认 GET
  headers: HeaderMap,         // 请求头
  params: Map<String, Value>, // 请求参数, 拼接到URL上
  data: Value,                // 请求体, 放到body中
  should_clear_params: bool,  // 是否清除默认参数, 默认 false
  should_signature: bool,     // 是否进行签名, 默认 true
  should_encrypt: bool,       // 是否进行 key 加密, 默认 false
  encrypt_type: EncryptType,  // 加密类型, 默认 Android
}

impl Default for HttpRequest {
  fn default() -> Self {
    Self {
      base_url: KG_BASE_URL.to_string(),
      url: String::new(),
      method: Method::GET,
      headers: HeaderMap::new(),
      params: Map::new(),
      data: Value::default(),
      should_clear_params: false,
      should_signature: true,
      should_encrypt: false,
      encrypt_type: EncryptType::Android,
    }
  }
}

impl HttpRequest {
  /// ## 构建请求实例
  pub fn new() -> Self {
    Self::default()
  }

  /// ## 设置基础地址
  ///
  /// ### 必选参数
  /// * `base_url` - 基础地址, 默认为酷狗的网关地址
  pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
    self.base_url = base_url.into();
    self
  }

  /// ## 设置请求地址
  ///
  /// ### 必选参数
  /// * `url` - 请求地址, 会拼接在基础地址之后
  pub fn url(mut self, url: impl Into<String>) -> Self {
    self.url = url.into();
    self
  }

  /// ## 设置请求方法为 GET
  pub fn get(mut self) -> Self {
    self.method = Method::GET;
    self
  }

  /// ## 设置请求方法为 POST
  pub fn post(mut self) -> Self {
    self.method = Method::POST;
    self
  }

  /// ## 添加请求头
  ///
  /// ### 必选参数
  /// * `key` - 请求头名称
  /// * `val` - 请求头的值
  pub fn header(mut self, key: impl AsRef<str>, val: impl AsRef<str>) -> Self {
    let Ok(key) = HeaderName::from_bytes(key.as_ref().as_bytes()) else {
      return self;
    };
    let Ok(val) = HeaderValue::from_bytes(val.as_ref().as_bytes()) else {
      return self;
    };

    self.headers.insert(key, val);
    self
  }

  /// ## 覆盖请求头
  ///
  /// ### 必选参数
  /// * `headers` - 请求头集合
  pub fn headers(mut self, headers: HeaderMap) -> Self {
    self.headers = headers;
    self
  }

  /// ## 设置请求参数
  ///
  /// ### 必选参数
  /// * `params` - 请求参数, 会拼接到 url 上
  pub fn params(mut self, params: Map<String, Value>) -> Self {
    self.params = params;
    self
  }

  /// ## 设置请求体
  ///
  /// ### 必选参数
  /// * `data` - 请求体, 放到 body 中
  pub fn data(mut self, data: Value) -> Self {
    self.data = data;
    self
  }

  /// ## 设置是否清除默认参数
  ///
  /// ### 必选参数
  /// * `should_clear` - 是否清除默认参数, 默认为 `false`
  pub fn clear_params(mut self, should_clear: bool) -> Self {
    self.should_clear_params = should_clear;
    self
  }

  /// ## 设置是否签名
  ///
  /// ### 必选参数
  /// * `should_sign` - 是否添加 signature 字段, 默认为 `true`
  pub fn signature(mut self, should_sign: bool) -> Self {
    self.should_signature = should_sign;
    self
  }

  /// ## 设置是否加密
  ///
  /// ### 必选参数
  /// * `should_encrypt` - 是否添加 key 字段, 默认为 `false`
  pub fn encrypt(mut self, should_encrypt: bool) -> Self {
    self.should_encrypt = should_encrypt;
    self
  }

  /// ## 设置加密类型
  ///
  /// ### 必选参数
  /// * `encrypt_type` - 加密类型, 默认为 `EncryptType::Android`
  pub fn encrypt_type(mut self, encrypt_type: EncryptType) -> Self {
    self.encrypt_type = encrypt_type;
    self
  }

  /// ## 构建客户端
  ///
  /// 合并默认参数与请求头, 按需添加 key 与 signature 字段
  pub fn builder(self) -> HttpClient {
    let static_config = match HttpConfig::get_static_config() {
      StaticConfigMode::KgMobile(config) => config,
      StaticConfigMode::KgLite(config) => config,
    };
    let dynamic_config = match HttpConfig::get_dynamic_config() {
      DynamicModeConfig::KgMobile(config) => config,
      DynamicModeConfig::KgLite(config) => config,
    };
    let cookies = match HttpCookie::get_cookies() {
      ModeCookies::KgMobile(cookies) => cookies,
      ModeCookies::KgLite(cookies) => cookies,
    };

    let uuid = "-";
    let client_time = Utc::now().timestamp().to_string();

    // 构建 url
    let mut url = format!("{}{}", self.base_url, self.url);

    // 构建请求头
    let mut headers = HeaderMap::from_iter([
      (
        HeaderName::from_static("dfid"),
        HeaderValue::from_str(&cookies.dfid).unwrap_or(HeaderValue::from_static("-")),
      ),
      (
        HeaderName::from_static("clienttime"),
        HeaderValue::from_str(&client_time).unwrap_or(HeaderValue::from_static("")),
      ),
      (
        HeaderName::from_static("mid"),
        HeaderValue::from_str(&dynamic_config.mid).unwrap_or(HeaderValue::from_static("")),
      ),
      (
        HeaderName::from_static("kg-rc"),
        HeaderValue::from_static("1"),
      ),
      (
        HeaderName::from_static("kg-thash"),
        HeaderValue::from_static("5d816a0"),
      ),
      (
        HeaderName::from_static("kg-rec"),
        HeaderValue::from_static("1"),
      ),
      (
        HeaderName::from_static("kg-rf"),
        HeaderValue::from_static("B9EDA08A64250DEFFBCADDEE00F8F25F"),
      ),
      (
        HeaderName::from_static("user-agent"),
        HeaderValue::from_static("Android15-1070-11083-46-0-DiscoveryDRADProtocol-wifi"),
      ),
    ]);
    headers.extend(self.headers);

    // 构建请求参数
    let mut params = if self.should_clear_params {
      self.params
    } else {
      let default_params = json!({
        "dfid": cookies.dfid,
        "mid": dynamic_config.mid,
        "uuid": uuid,
        "appid": static_config.appid,
        "clientver": static_config.client_ver,
        "clienttime": client_time
      });
      let Value::Object(mut default_params) = default_params else { unreachable!() };

      if cookies.token != "" {
        default_params.insert("token".to_string(), json!(cookies.token));
      }

      if cookies.userid != 0 {
        default_params.insert("userid".to_string(), json!(cookies.userid));
      }

      default_params.extend(self.params);
      default_params
    };

    // 添加 key 字段
    if self.should_encrypt {
      let hash = params.get("hash").map_or("".to_string(), |v| match v {
        Value::Null => String::new(),
        Value::String(v) => v.to_owned(),
        _ => v.to_string(),
      });
      let mid = params.get("mid").map_or("".to_string(), |v| match v {
        Value::Null => String::new(),
        Value::String(v) => v.to_owned(),
        _ => v.to_string(),
      });
      let userid = params.get("userid").map_or("0".to_string(), |v| match v {
        Value::Null => String::new(),
        Value::String(v) => v.to_owned(),
        _ => v.to_string(),
      });
      let appid = params
        .get("appid")
        .map_or(static_config.appid.to_string(), |v| match v {
          Value::Null => String::new(),
          Value::String(v) => v.to_owned(),
          _ => v.to_string(),
        });

      let key = sign_key(&hash, &mid, Some(&userid), Some(&appid));
      params.insert("key".to_string(), json!(key));
    }

    // 添加 signature 字段
    if self.should_signature && params.get("signature").is_none() {
      let data_str = match &self.data {
        Value::Null => String::new(),
        Value::String(str) => str.to_owned(),
        data => data.to_string(),
      };

      let signature = match self.encrypt_type {
        EncryptType::Web => sign_params_web(&params, Some(&data_str)),
        EncryptType::Android => sign_params_android(&params, Some(&data_str)),
        EncryptType::Register => sign_params_register(&params),
      };

      params.insert("signature".to_string(), json!(signature));
    }

    // TODO: 不清楚为什么手动拼接
    if self.base_url.contains("openapicdn") {
      let params_str = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

      url = format!("{url}?{params_str}");
      params.clear();
    }

    HttpClient::new(url, self.method)
      .headers(headers)
      .params(params)
      .data(self.data)
  }
}
