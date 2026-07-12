use anyhow::anyhow;
use serde_json::json;
use std::sync::{LazyLock, RwLock};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::http::{
  client::HttpRequest,
  lib::{
    DynamicConfig, KgCookies, KgDynamicConfig, KgStaticConfig, KgTerminalConfig, StaticConfig,
  },
  mode::{HttpMode, Mode},
  server::BASE_URL,
};

// 配置文件存储路径
pub const STORE_PATH: &str = "config.json";
// 配置文件存储键
pub const CONFIG_KEY: &str = "http_config";
pub const MODE_KEY: &str = "http_mode";

// 动态配置
static DYNAMIC_CONFIG: LazyLock<RwLock<DynamicConfig>> =
  LazyLock::new(|| RwLock::new(DynamicConfig::default()));
// 静态配置
static STATIC_CONFIG: LazyLock<StaticConfig> = LazyLock::new(|| StaticConfig {
  kg: KgTerminalConfig {
    mobile: KgStaticConfig {
      api_ver: 20,
      src_appid: 2919,
      appid: 1005,
      client_ver: 20489,
      wx_appid: "wx79f2c4418704b4f8",
      wx_secret: "4efcab88b700769e376e3f6087b8abc9",
      rsa_pem: "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDIAG7QOELSYoIJvTFJhMpe1s/g
bjDJX51HBNnEl5HXqTW6lQ7LC8jr9fWZTwusknp+sVGzwd40MwP6U5yDE27M/X1+
UR4tvOGOqp94TJtQ1EPnWGWXngpeIW5GxoQGao1rmYWAu6oi1z9XkChrsUdC6DJE
5E221wf/4WLFxwAtRQIDAQAB
-----END PUBLIC KEY-----",
      params_padding: "R6snCXJgbCaj9WFRJKefTMIFp0ey6Gza",
      params_web_padding: "NVPh5oo715z5DIWAeQlhMDsWXXQV4hwt",
      params_android_padding: "OIlwieks28dk2k092lksi2UIkp",
      params_register_padding: "1014",
      key_padding: "57ae12eb6890223e355ccfcb74edf70d",
      key_cloud_padding: "ebd1ac3134c880bda6a2194537843caa0162e2e7",
      key_params_padding: "OIlwieks28dk2k092lksi2UIkp",
    },
    lite: KgStaticConfig {
      api_ver: 20,
      src_appid: 2919,
      appid: 3116,
      client_ver: 11440,
      wx_appid: "wx72b795aca60ad321",
      wx_secret: "33e486041e5e25729a4e3d2da7502f9a",
      rsa_pem: "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDECi0Np2UR87scwrvTr72L6oO0
1rBbbBPriSDFPxr3Z5syug0O24QyQO8bg27+0+4kBzTBTBOZ/WWU0WryL1JSXRTX
LgFVxtzIY41Pe7lPOgsfTCn5kZcvKhYKJesKnnJDNr5/abvTGf+rHG3YRwsCHcQ0
8/q6ifSioBszvb3QiwIDAQAB
-----END PUBLIC KEY-----",
      params_padding: "R6snCXJgbCaj9WFRJKefTMIFp0ey6Gza",
      params_web_padding: "NVPh5oo715z5DIWAeQlhMDsWXXQV4hwt",
      params_android_padding: "LnT6xpN3khm36zse0QzvmgTZ3waWdRSA",
      params_register_padding: "1014",
      key_padding: "185672dd44712f60bb1736df5a377e82",
      key_cloud_padding: "ebd1ac3134c880bda6a2194537843caa0162e2e7",
      key_params_padding: "LnT6xpN3khm36zse0QzvmgTZ3waWdRSA",
    },
  },
});

pub struct HttpConfig;

impl HttpConfig {
  pub fn init(app_handle: &AppHandle) {
    let config = Self::load_dynamic_config(app_handle);

    if let Ok(mut http_config) = DYNAMIC_CONFIG.write() {
      *http_config = config;
    }
  }

  /// 从 store 加载动态配置
  fn load_dynamic_config(app_handle: &AppHandle) -> DynamicConfig {
    // 尝试获取 store
    let Ok(store) = app_handle.store(STORE_PATH) else {
      return DynamicConfig::default();
    };

    // 尝试获取 store 配置
    if let Some(store_config) = store.get(CONFIG_KEY) {
      if let Ok(config) = serde_json::from_value::<DynamicConfig>(store_config) {
        // 从 store 中恢复 cookies
        let cookies = match HttpMode::get_mode() {
          Mode::KgMobile => &config.kg.mobile.cookies,
          Mode::KgLite => &config.kg.lite.cookies,
        };
        HttpRequest::set_cookies(BASE_URL, cookies.to_hashmap());

        return config;
      }
    }

    // store 配置不存在或解析失败，创建新配置并保存
    let config = DynamicConfig::default();

    if let Ok(value_config) = serde_json::to_value(&config) {
      store.set(CONFIG_KEY, value_config);
      let _ = store.save();
    }

    config
  }

  /// 设置 kg 的动态配置
  pub fn set_kg_cookies(
    app_handle: &AppHandle,
    url: &str,
    cookies: KgCookies,
  ) -> anyhow::Result<()> {
    HttpRequest::set_cookies(url, cookies.to_hashmap());

    let mut config = DYNAMIC_CONFIG.write().map_err(|e| anyhow!(e.to_string()))?;

    match HttpMode::get_mode() {
      Mode::KgMobile => config.kg.mobile.cookies = cookies,
      Mode::KgLite => config.kg.lite.cookies = cookies,
    }

    let store = app_handle.store(STORE_PATH)?;
    store.set(CONFIG_KEY, json!(*config));
    store.save()?;

    Ok(())
  }

  pub fn clear_kg_cookies(app_handle: &AppHandle, url: &str) -> anyhow::Result<()> {
    HttpRequest::clear_cookies(url);

    let mut config = DYNAMIC_CONFIG.write().map_err(|e| anyhow!(e.to_string()))?;

    let default_cookies = KgCookies::default();

    match HttpMode::get_mode() {
      Mode::KgMobile => config.kg.mobile.cookies = default_cookies,
      Mode::KgLite => config.kg.lite.cookies = default_cookies,
    }

    let store = app_handle.store(STORE_PATH)?;
    store.set(CONFIG_KEY, json!(*config));
    store.save()?;

    Ok(())
  }

  /// 获取 kg 的动态配置
  pub fn get_kg_dynamic_config() -> KgDynamicConfig {
    let Ok(config) = DYNAMIC_CONFIG.read() else {
      return KgDynamicConfig::default();
    };

    match HttpMode::get_mode() {
      Mode::KgMobile => config.kg.mobile.clone(),
      Mode::KgLite => config.kg.lite.clone(),
    }
  }

  /// 获取 kg 的静态配置
  pub fn get_kg_static_config() -> &'static KgStaticConfig {
    match HttpMode::get_mode() {
      Mode::KgMobile => &STATIC_CONFIG.kg.mobile,
      Mode::KgLite => &STATIC_CONFIG.kg.lite,
    }
  }
}
