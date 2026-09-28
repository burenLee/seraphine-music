use aes::{
  cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
  Aes128, Aes256,
};
use anyhow::anyhow;
use base64::{engine::general_purpose::STANDARD, Engine};
use cbc::{Decryptor, Encryptor};
use md5::{Digest, Md5};
use rsa::{
  pkcs8::DecodePublicKey, rand_core::OsRng, traits::PublicKeyParts, BigUint, Pkcs1v15Encrypt,
  RsaPublicKey,
};
use sha1::Sha1;

use crate::{
  http::config::{HttpConfig, KgStaticConfig, StaticConfigMode},
  utils::tools::gen_random_string,
};

/// ## md5 加密
///
/// 返回 32 长度的 16 进制字符串
///
/// ### 必选参数
/// * `data` - 待加密的数据
pub fn encrypt_md5(data: impl AsRef<[u8]>) -> String {
  let mut hasher = Md5::new();
  hasher.update(data);

  hex::encode(hasher.finalize())
}

/// ## sha1 加密
///
/// 返回 40 长度的 16 进制字符串
///
/// ### 必选参数
/// * `data` - 待加密的数据
pub fn encrypt_sha1(data: impl AsRef<[u8]>) -> String {
  let mut hasher = Sha1::new();
  hasher.update(data);

  hex::encode(hasher.finalize())
}

/// ## AES 加密
///
/// 使用 Aes256 加密, 返回 (加密后的 16 进制字符串, 密钥)
///
/// ### 必选参数
/// * `data` - 待加密的数据
///
/// ### 可选参数
/// * `key` - 密钥, 不传时随机生成
/// * `iv` - 偏移量, 不传时随机生成
pub fn encrypt_aes(
  data: impl AsRef<[u8]>,
  key: Option<&str>,
  iv: Option<&str>,
) -> anyhow::Result<(String, String)> {
  let (key, iv, temp_key) = match (key, iv) {
    (Some(key), Some(iv)) => (key.to_owned(), iv.to_owned(), key.to_owned()),
    _ => {
      let temp_key = gen_random_string(16).to_lowercase();
      let key = encrypt_md5(&temp_key);
      let iv = key[16..].to_owned();

      (key, iv, temp_key)
    }
  };

  let encryptor = Encryptor::<Aes256>::new_from_slices(key.as_ref(), iv.as_ref())?;
  let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data.as_ref());

  Ok((hex::encode(encrypted), temp_key))
}

/// ## AES 解密
///
/// 使用 Aes256 解密, 返回解密后的字符串
///
/// ### 必选参数
/// * `data` - 待解密的数据
/// * `key` - 密钥
///
/// ### 可选参数
/// * `iv` - 偏移量, 不传时由密钥推导
pub fn decrypt_aes(data: impl AsRef<[u8]>, key: &str, iv: Option<&str>) -> anyhow::Result<String> {
  let (key, iv) = match iv {
    Some(iv) => (key.to_owned(), iv.to_owned()),
    None => {
      let key = encrypt_md5(key);
      let iv = key[16..].to_owned();

      (key, iv)
    }
  };

  let mut data_buf = hex::decode(data)?;
  let decryptor = Decryptor::<Aes256>::new_from_slices(key.as_ref(), iv.as_ref())?;
  let decrypted = decryptor
    .decrypt_padded_vec_mut::<Pkcs7>(&mut data_buf)
    .map_err(|e| anyhow!(e))?;

  Ok(String::from_utf8_lossy(&decrypted).into_owned())
}

/// ## RSA 加密, 无 padding
///
/// ### 必选参数
/// * `data` - 待加密的数据
///
/// ### 可选参数
/// * `pem` - 公钥, 不传时使用当前模式的公钥
pub fn encrypt_rsa_unpad(data: impl AsRef<[u8]>, pem: Option<&str>) -> anyhow::Result<String> {
  let KgStaticConfig { rsa_pem, .. } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let key = RsaPublicKey::from_public_key_pem(pem.unwrap_or(rsa_pem))?;

  let mut data_buf = data.as_ref().to_owned();
  // 不够key长度补零
  data_buf.resize(key.size(), 0);

  let encrypted = BigUint::from_bytes_be(&data_buf).modpow(key.e(), key.n());

  Ok(encrypted.to_str_radix(16))
}

/// ## RSA 加密, 有 padding
///
/// 使用当前模式的公钥加密
///
/// ### 必选参数
/// * `data` - 待加密的数据
pub fn encrypt_rsa_pad(data: impl AsRef<[u8]>) -> anyhow::Result<String> {
  let KgStaticConfig { rsa_pem, .. } = match HttpConfig::get_static_config() {
    StaticConfigMode::KgMobile(config) => config,
    StaticConfigMode::KgLite(config) => config,
  };

  let key = RsaPublicKey::from_public_key_pem(rsa_pem)?;

  let encrypted = key.encrypt(&mut OsRng, Pkcs1v15Encrypt, data.as_ref())?;

  Ok(hex::encode(encrypted))
}

/// ## 用于播放列表的 AES 加密
///
/// 使用 Aes128 加密, 返回 (加密后的 base64 字符串, 密钥)
///
/// ### 必选参数
/// * `data` - 待加密的数据
pub fn encrypt_aes_playlist(data: impl AsRef<[u8]>) -> anyhow::Result<(String, String)> {
  let temp_key = gen_random_string(6).to_lowercase();
  let hash = encrypt_md5(&temp_key);
  let (key, iv) = (&hash[..16], &hash[16..]);

  let encryptor = Encryptor::<Aes128>::new_from_slices(key.as_bytes(), iv.as_bytes())?;
  let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data.as_ref());

  Ok((STANDARD.encode(encrypted), temp_key))
}

/// ## 用于播放列表的 AES 解密
///
/// 使用 Aes128 解密, 返回解密后的字符串
///
/// ### 必选参数
/// * `data` - 待解密的数据
/// * `key` - 密钥
pub fn decrypt_aes_playlist(data: impl AsRef<[u8]>, key: &str) -> anyhow::Result<String> {
  let hash = encrypt_md5(key);
  let (key, iv) = (&hash[..16], &hash[16..]);

  let mut data_buf = STANDARD.decode(&data)?;

  let decryptor = Decryptor::<Aes128>::new_from_slices(key.as_ref(), iv.as_ref())?;
  let decrypted = decryptor
    .decrypt_padded_vec_mut::<Pkcs7>(&mut data_buf)
    .map_err(|e| anyhow!(e))?;

  Ok(String::from_utf8_lossy(&decrypted).into_owned())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn md5_known_vectors() {
    assert_eq!(encrypt_md5(""), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(encrypt_md5("abc"), "900150983cd24fb0d6963f7d28e17f72");
  }

  #[test]
  fn sha1_known_vector() {
    assert_eq!(encrypt_sha1("abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
  }

  #[test]
  fn aes_roundtrip_with_derived_key() {
    let (encrypted, temp_key) = encrypt_aes("hello aes", None, None).unwrap();
    let decrypted = decrypt_aes(&encrypted, &temp_key, None).unwrap();

    assert_eq!(decrypted, "hello aes");
  }

  #[test]
  fn aes_roundtrip_with_explicit_key_iv() {
    let key = "0123456789abcdef0123456789abcdef";
    let iv = "0123456789abcdef";
    let (encrypted, _) = encrypt_aes("hello aes", Some(key), Some(iv)).unwrap();
    let decrypted = decrypt_aes(&encrypted, key, Some(iv)).unwrap();

    assert_eq!(decrypted, "hello aes");
  }

  #[test]
  fn aes_playlist_roundtrip() {
    let (encrypted, temp_key) = encrypt_aes_playlist("playlist payload").unwrap();
    let decrypted = decrypt_aes_playlist(&encrypted, &temp_key).unwrap();

    assert_eq!(decrypted, "playlist payload");
  }
}
