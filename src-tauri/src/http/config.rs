use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{LazyLock, RwLock};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

use crate::{
  app::{
    config::Config as AppConfig,
    mode::{AppMode, Mode},
  },
  http::{CONFIG_KEY, HTTP_STORE},
  utils::{crypto::encrypt_md5, tools::gen_random_string},
};

/// 静态配置
pub struct StaticConfig {
  pub kg_mobile: StaticKgConfig,
  pub kg_lite: StaticKgConfig,
}

pub struct StaticKgConfig {
  pub api_ver: u16,
  pub src_appid: u16,
  pub appid: u16,
  pub client_ver: u16,
  pub wx_appid: &'static str,
  pub wx_secret: &'static str,
  pub qq_appid: &'static str,
  pub rsa_pem: &'static str,

  pub params_padding: &'static str,
  pub params_web_padding: &'static str,
  pub params_android_padding: &'static str,
  pub params_register_padding: &'static str,
  pub key_padding: &'static str,
  pub key_cloud_padding: &'static str,
  pub key_params_padding: &'static str,
}

pub enum StaticModeConfig {
  KgMobile(StaticKgConfig),
  KgLite(StaticKgConfig),
}

const STATIC_CONFIG: StaticConfig = StaticConfig {
  kg_mobile: StaticKgConfig {
    api_ver: 20,
    src_appid: 2919,
    appid: 1005,
    client_ver: 20489,
    wx_appid: "wx79f2c4418704b4f8",
    wx_secret: "4efcab88b700769e376e3f6087b8abc9",
    qq_appid: "205141",
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
  kg_lite: StaticKgConfig {
    api_ver: 20,
    src_appid: 2919,
    appid: 3116,
    client_ver: 11440,
    wx_appid: "wx72b795aca60ad321",
    wx_secret: "33e486041e5e25729a4e3d2da7502f9a",
    qq_appid: "101706348",
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
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DynamicConfig {
  pub kg_mobile: DynamicKgConfig,
  pub kg_lite: DynamicKgConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicKgConfig {
  pub mid: String,   // 设备 MID
  pub guid: String,  // 设备全局唯一标识符
  pub dev: String,   // 开发设备标识符
  pub mac: String,   // 设备 MAC 地址，默认为 '02:00:00:00:00:00'
  pub webgl: String, // WebGL 指纹哈希值
}

impl Default for DynamicKgConfig {
  fn default() -> Self {
    let guid = encrypt_md5(Uuid::new_v4());
    let mid = u128::from_str_radix(&guid, 16).unwrap_or(0).to_string();
    let dev = gen_random_string(10);

    DynamicKgConfig {
      mid,
      guid,
      dev,
      mac: "02:00:00:00:00:00".to_string(),
      webgl: String::new(),
    }
  }
}

pub enum DynamicModeConfig {
  KgMobile(DynamicKgConfig),
  KgLite(DynamicKgConfig),
}

// 动态配置
static DYNAMIC_CONFIG: LazyLock<RwLock<DynamicConfig>> =
  LazyLock::new(|| RwLock::new(DynamicConfig::default()));

pub struct Config;

impl Config {
  /// ## 初始化配置
  ///
  /// 从 store 中加载已保存的动态配置
  pub fn init() {
    let config = Self::load_config();

    let mut dynamic_config = DYNAMIC_CONFIG.write().unwrap_or_else(|e| e.into_inner());
    *dynamic_config = config;
  }

  /// ## 从 store 加载动态配置
  fn load_config() -> DynamicConfig {
    AppConfig::app()
      .store(HTTP_STORE)
      .ok()
      .and_then(|store| store.get(CONFIG_KEY))
      .and_then(|value| serde_json::from_value::<DynamicConfig>(value).ok())
      .unwrap_or_default()
  }

  /// ## 保存动态配置到 store
  ///
  /// ### 必选参数
  /// * `config` - 动态配置
  fn save_config(config: &DynamicConfig) -> anyhow::Result<()> {
    let store = AppConfig::app().store(HTTP_STORE)?;
    store.set(CONFIG_KEY, json!(config));
    store.save()?;

    Ok(())
  }

  /// ## 获取当前模式的静态配置
  pub fn get_static_config() -> &'static StaticModeConfig {
    match Mode::get_mode() {
      AppMode::KgMobile => &StaticModeConfig::KgMobile(STATIC_CONFIG.kg_mobile),
      AppMode::KgLite => &StaticModeConfig::KgLite(STATIC_CONFIG.kg_lite),
    }
  }

  /// ## 获取当前模式的动态配置
  pub fn get_dynamic_config() -> DynamicModeConfig {
    let dynamic_config = DYNAMIC_CONFIG.read().unwrap_or_else(|e| e.into_inner());

    match Mode::get_mode() {
      AppMode::KgMobile => DynamicModeConfig::KgMobile(dynamic_config.kg_mobile.clone()),
      AppMode::KgLite => DynamicModeConfig::KgLite(dynamic_config.kg_lite.clone()),
    }
  }

  /// ## 设置当前模式的动态配置
  ///
  /// ### 必选参数
  /// * `config` - 动态配置
  pub fn set_dynamic_config(config: DynamicModeConfig) -> anyhow::Result<()> {
    let dynamic_config = {
      let mut dynamic_config = DYNAMIC_CONFIG.write().unwrap_or_else(|e| e.into_inner());

      match config {
        DynamicModeConfig::KgMobile(config) => dynamic_config.kg_mobile = config,
        DynamicModeConfig::KgLite(config) => dynamic_config.kg_lite = config,
      };

      dynamic_config.clone()
    };

    Self::save_config(&dynamic_config)
  }

  /// ## 重置当前模式的动态配置
  pub fn reset_dynamic_config() -> anyhow::Result<()> {
    let dynamic_config = {
      let mut dynamic_config = DYNAMIC_CONFIG.write().unwrap_or_else(|e| e.into_inner());

      match Mode::get_mode() {
        AppMode::KgMobile => dynamic_config.kg_mobile = DynamicKgConfig::default(),
        AppMode::KgLite => dynamic_config.kg_lite = DynamicKgConfig::default(),
      };

      dynamic_config.clone()
    };

    Self::save_config(&dynamic_config)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn kg_dynamic_config_default_fields() {
    let config = DynamicKgConfig::default();

    // guid 为 32 位 16 进制字符串, mid 为其十进制表示
    assert_eq!(config.guid.len(), 32);
    assert!(config.guid.chars().all(|c| c.is_ascii_hexdigit()));
    let mid_expected = u128::from_str_radix(&config.guid, 16)
      .unwrap_or(0)
      .to_string();
    assert_eq!(config.mid, mid_expected);

    assert_eq!(config.dev.len(), 10);
    assert_eq!(config.mac, "02:00:00:00:00:00");
    assert!(config.webgl.is_empty());
  }

  #[test]
  fn get_static_config_defaults_to_kg_lite() {
    match Config::get_static_config() {
      StaticModeConfig::KgLite(config) => assert_eq!(config.appid, 3116),
      StaticModeConfig::KgMobile(_) => panic!("默认模式应为 KgLite"),
    }
  }
}
