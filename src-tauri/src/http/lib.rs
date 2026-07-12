use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::utils::{
  crypto::encrypt_md5,
  tools::{calculate_mid, random_string},
};

/// 动态配置
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DynamicConfig {
  pub kg: KgTerminalConfig<KgDynamicConfig>,
}

/// 静态配置
#[derive(Debug)]
pub struct StaticConfig {
  pub kg: KgTerminalConfig<KgStaticConfig>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KgTerminalConfig<T> {
  pub mobile: T,
  pub lite: T,
}

/// kg的详细动态配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgDynamicConfig {
  pub platform: String,
  pub mac: String,
  pub guid: String,
  pub mid: String,
  pub dev: String,
  pub cookies: KgCookies,
}

impl Default for KgDynamicConfig {
  fn default() -> Self {
    let guid = encrypt_md5(Uuid::new_v4());
    let mid = calculate_mid(&guid);

    KgDynamicConfig {
      platform: String::new(),
      mac: String::from("02:00:00:00:00:00"),
      guid: guid,
      mid: mid,
      dev: random_string(10),
      // TODO: cookies 未来可以去掉, 直接在HttpRequest中获取和保存
      cookies: KgCookies::default(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgCookies {
  pub dfid: String,
  pub userid: u64,
  pub token: String,
  pub t1: String,
  pub vip_type: u64,
  pub vip_token: String,
}

impl Default for KgCookies {
  fn default() -> Self {
    KgCookies {
      dfid: String::from("-"),
      userid: 0,
      token: String::from(""),
      t1: String::from(""),
      vip_type: 0,
      vip_token: String::from(""),
    }
  }
}

impl KgCookies {
  pub fn to_hashmap(&self) -> HashMap<String, String> {
    HashMap::from_iter([
      (String::from("dfid"), self.dfid.to_string()),
      (String::from("userid"), self.userid.to_string()),
      (String::from("token"), self.token.to_string()),
      (String::from("t1"), self.t1.to_string()),
      (String::from("vip_type"), self.vip_type.to_string()),
      (String::from("vip_token"), self.vip_token.to_string()),
    ])
  }
}

/// kg的详细静态配置
#[derive(Debug)]
pub struct KgStaticConfig {
  // 通用
  pub api_ver: u16,
  pub src_appid: u16,
  pub appid: u16,
  pub client_ver: u16,
  pub wx_appid: &'static str,
  pub wx_secret: &'static str,
  // 加密
  pub rsa_pem: &'static str,
  // sign
  pub params_padding: &'static str,
  pub params_web_padding: &'static str,
  pub params_android_padding: &'static str,
  pub params_register_padding: &'static str,
  pub key_padding: &'static str,
  pub key_cloud_padding: &'static str,
  pub key_params_padding: &'static str,
}
