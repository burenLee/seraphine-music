use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
  collections::HashMap,
  sync::{Arc, LazyLock},
};
use tauri_plugin_http::reqwest::{
  cookie::{CookieStore, Jar},
  header::HeaderValue,
};
use tauri_plugin_store::StoreExt;

use crate::{
  app::{
    config::APP_HANDLE,
    mode::{AppMode, Mode},
  },
  http::config::{COOKIE_KEY, HTTP_STORE, KG_BASE_DOMAIN, KG_BASE_URL},
  utils::{
    logger::LogErrExt,
    tools::{gen_url, gen_valid_cookie},
  },
};

static HTTP_COOKIE_JAR: LazyLock<Arc<Jar>> = LazyLock::new(|| Arc::new(Jar::default()));

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HttpCookie {
  pub kg_mobile: KgCookie,
  pub kg_lite: KgCookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgCookie {
  pub dfid: String,
  pub userid: u64,
  pub token: String,
  pub t1: String,
  pub vip_type: u64,
  pub vip_token: String,
}

impl Default for KgCookie {
  fn default() -> Self {
    KgCookie {
      dfid: "-".to_string(),
      userid: 0,
      token: String::new(),
      t1: String::new(),
      vip_type: 0,
      vip_token: String::new(),
    }
  }
}

impl KgCookie {
  pub fn to_hashmap(&self) -> HashMap<String, String> {
    HashMap::from_iter([
      ("dfid".to_string(), self.dfid.to_string()),
      ("userid".to_string(), self.userid.to_string()),
      ("token".to_string(), self.token.to_string()),
      ("t1".to_string(), self.t1.to_string()),
      ("vip_type".to_string(), self.vip_type.to_string()),
      ("vip_token".to_string(), self.vip_token.to_string()),
    ])
  }
}

impl From<&HeaderValue> for KgCookie {
  /// ## 从响应头中解析 cookie
  ///
  /// ### 必选参数
  /// * `value` - 响应头中的 cookie 值
  fn from(value: &HeaderValue) -> Self {
    let mut cookies = KgCookie::default();

    let Ok(raw) = value.to_str() else {
      return cookies;
    };

    for pair in raw.split(';') {
      let Some((key, val)) = pair.split_once('=') else {
        continue;
      };

      let (key, val) = (key.trim(), val.trim());
      match key {
        "dfid" => cookies.dfid = val.to_string(),
        "userid" => cookies.userid = val.parse().unwrap_or(0),
        "token" => cookies.token = val.to_string(),
        "t1" => cookies.t1 = val.to_string(),
        "vip_type" => cookies.vip_type = val.parse().unwrap_or(0),
        "vip_token" => cookies.vip_token = val.to_string(),
        _ => {}
      }
    }

    cookies
  }
}

impl From<HeaderValue> for KgCookie {
  /// ## 从响应头中解析 cookie
  ///
  /// ### 必选参数
  /// * `value` - 响应头中的 cookie 值
  fn from(value: HeaderValue) -> Self {
    KgCookie::from(&value)
  }
}

#[derive(Debug)]
pub enum ModeCookies {
  KgMobile(KgCookie),
  KgLite(KgCookie),
}

impl HttpCookie {
  /// ## 获取 cookie 管理器
  pub fn get_jar() -> Arc<Jar> {
    HTTP_COOKIE_JAR.clone()
  }

  /// ## 初始化 cookie
  ///
  /// 从 store 中加载已保存的 cookie
  pub fn init() {
    let http_cookies = Self::load_cookies();

    let cookies = match AppMode::get_mode() {
      Mode::KgMobile => ModeCookies::KgMobile(http_cookies.kg_mobile),
      Mode::KgLite => ModeCookies::KgLite(http_cookies.kg_lite),
    };

    let _ = Self::set_cookies(cookies).log_err("HttpCookie::init", "初始化失败");
  }

  /// ## 从 store 加载 cookie
  fn load_cookies() -> HttpCookie {
    APP_HANDLE
      .get()
      .and_then(|app_handle| app_handle.store(HTTP_STORE).ok())
      .and_then(|store| store.get(COOKIE_KEY))
      .and_then(|value| serde_json::from_value::<HttpCookie>(value).ok())
      .unwrap_or_default()
  }

  /// ## 保存 cookie 到 store
  ///
  /// ### 必选参数
  /// * `cookies` - cookie
  fn save_cookies(cookies: &HttpCookie) -> anyhow::Result<()> {
    let app_handle = APP_HANDLE.get().ok_or(anyhow!("未获取到APP_HANDLE"))?;
    let store = app_handle.store(HTTP_STORE)?;
    store.set(COOKIE_KEY, json!(cookies));
    store.save()?;

    Ok(())
  }

  /// ## 获取当前模式的 cookie
  pub fn get_cookies() -> ModeCookies {
    match AppMode::get_mode() {
      Mode::KgMobile => ModeCookies::KgMobile(
        HTTP_COOKIE_JAR
          .cookies(&gen_url(KG_BASE_URL))
          .map(|header| KgCookie::from(header))
          .unwrap_or_else(|| HttpCookie::default().kg_mobile),
      ),
      Mode::KgLite => ModeCookies::KgLite(
        HTTP_COOKIE_JAR
          .cookies(&gen_url(KG_BASE_URL))
          .map(|header| KgCookie::from(header))
          .unwrap_or_else(|| HttpCookie::default().kg_lite),
      ),
    }
  }

  /// ## 设置当前模式的 cookie
  ///
  /// ### 必选参数
  /// * `cookies` - cookie
  pub fn set_cookies(cookies: ModeCookies) -> anyhow::Result<()> {
    let mut http_cookies = Self::load_cookies();

    let (cookie_map, url) = match cookies {
      ModeCookies::KgMobile(cookies) => {
        let cookie_map = cookies.to_hashmap();
        http_cookies.kg_mobile = cookies;

        (cookie_map, gen_url(KG_BASE_URL))
      }
      ModeCookies::KgLite(cookies) => {
        let cookie_map = cookies.to_hashmap();
        http_cookies.kg_lite = cookies;

        (cookie_map, gen_url(KG_BASE_URL))
      }
    };

    for (k, v) in cookie_map {
      HTTP_COOKIE_JAR.add_cookie_str(
        &format!(
          "{}={}; Domain={KG_BASE_DOMAIN}; Path=/",
          gen_valid_cookie(k),
          gen_valid_cookie(v)
        ),
        &url,
      );
    }

    Self::save_cookies(&http_cookies)?;

    Ok(())
  }

  /// ## 清除当前模式的 cookie
  pub fn clear_cookies() -> anyhow::Result<()> {
    let mut http_cookies = Self::load_cookies();

    let (cookie_map, url) = match AppMode::get_mode() {
      Mode::KgMobile => {
        let cookies = KgCookie::default();
        let cookies_map = cookies.to_hashmap();
        http_cookies.kg_mobile = cookies;

        (cookies_map, gen_url(KG_BASE_URL))
      }
      Mode::KgLite => {
        let cookies = KgCookie::default();
        let cookies_map = cookies.to_hashmap();
        http_cookies.kg_lite = cookies;

        (cookies_map, gen_url(KG_BASE_URL))
      }
    };

    for (k, v) in cookie_map {
      HTTP_COOKIE_JAR.add_cookie_str(
        &format!(
          "{}={}; Domain={KG_BASE_DOMAIN}; Path=/",
          gen_valid_cookie(k),
          gen_valid_cookie(v)
        ),
        &url,
      );
    }

    Self::save_cookies(&http_cookies)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use tauri_plugin_http::reqwest::header::HeaderValue;

  use super::*;

  #[test]
  fn kg_cookie_default() {
    let cookie = KgCookie::default();

    assert_eq!(cookie.dfid, "-");
    assert_eq!(cookie.userid, 0);
    assert_eq!(cookie.token, "");
    assert_eq!(cookie.t1, "");
    assert_eq!(cookie.vip_type, 0);
    assert_eq!(cookie.vip_token, "");
  }

  #[test]
  fn kg_cookie_to_hashmap() {
    let cookie = KgCookie {
      dfid: "d".to_string(),
      userid: 1,
      token: "t".to_string(),
      t1: "t1".to_string(),
      vip_type: 2,
      vip_token: "vt".to_string(),
    };

    let map = cookie.to_hashmap();

    assert_eq!(map.len(), 6);
    assert_eq!(map.get("dfid").map(String::as_str), Some("d"));
    assert_eq!(map.get("userid").map(String::as_str), Some("1"));
    assert_eq!(map.get("token").map(String::as_str), Some("t"));
    assert_eq!(map.get("t1").map(String::as_str), Some("t1"));
    assert_eq!(map.get("vip_type").map(String::as_str), Some("2"));
    assert_eq!(map.get("vip_token").map(String::as_str), Some("vt"));
  }

  #[test]
  fn kg_cookie_from_header_parses_fields() {
    let header =
      HeaderValue::from_static("dfid=abc;userid=123;token=tok;t1=t1val;vip_type=5;vip_token=vtok");

    let cookie = KgCookie::from(&header);

    assert_eq!(cookie.dfid, "abc");
    assert_eq!(cookie.userid, 123);
    assert_eq!(cookie.token, "tok");
    assert_eq!(cookie.t1, "t1val");
    assert_eq!(cookie.vip_type, 5);
    assert_eq!(cookie.vip_token, "vtok");
  }

  #[test]
  fn kg_cookie_from_header_invalid_userid_falls_back_zero() {
    let header = HeaderValue::from_static("userid=abc;dfid=xyz");

    let cookie = KgCookie::from(&header);

    assert_eq!(cookie.userid, 0);
    assert_eq!(cookie.dfid, "xyz");
  }

  #[test]
  fn kg_cookie_from_header_ignores_unknown_keys() {
    let header = HeaderValue::from_static("unknown=1;dfid=xyz");

    let cookie = KgCookie::from(&header);

    assert_eq!(cookie.dfid, "xyz");
    assert_eq!(cookie.token, "");
  }
}
