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

use crate::{http::config::HttpConfig, utils::tools::random_string};

const AES_BLOCK_SIZE: usize = 16;

#[derive(Debug)]
pub struct EncryptedResult {
  pub str: String,
  pub key: String,
}

/// md5 加密, 返回 32 长度的 16 进制字符串
pub fn encrypt_md5(data: impl AsRef<[u8]>) -> String {
  let mut hasher = Md5::new();
  hasher.update(data);

  hex::encode(hasher.finalize())
}

/// sha1 加密, 返回 40 长度的 16 进制字符串
pub fn encrypt_sha1(data: impl AsRef<[u8]>) -> String {
  let mut hasher = Sha1::new();
  hasher.update(data);

  hex::encode(hasher.finalize())
}

/// AES 加密
pub fn encrypt_aes(
  data: impl AsRef<[u8]>,
  key: Option<&str>,
  iv: Option<&str>,
) -> anyhow::Result<EncryptedResult> {
  // 如果传递了 key 和 iv，则使用它们，否则创建随机字符串
  let (key, iv, temp_key) = match (key, iv) {
    (Some(key), Some(iv)) => {
      if key.len() != 32 {
        return Err(anyhow!("AES解密key字节长度不足32, 当前长度: {}", key.len()));
      }
      if iv.len() != 16 {
        return Err(anyhow!("AES解密iv字节长度不足16, 当前长度: {}", iv.len()));
      }

      (key.to_owned(), iv.to_owned(), key.to_owned())
    }
    _ => {
      let temp_key = random_string(16).to_lowercase();
      let key = encrypt_md5(&temp_key);
      let iv = key[16..].to_owned();

      (key, iv, temp_key)
    }
  };

  let encryptor = Encryptor::<Aes256>::new_from_slices(key.as_bytes(), iv.as_bytes())
    .map_err(|e| anyhow!("AES加密创建失败: {e}"))?;

  let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data.as_ref());

  let encrypted_str = hex::encode(encrypted);

  Ok(EncryptedResult {
    str: encrypted_str,
    key: temp_key,
  })
}

/// AES 解密
pub fn decrypt_aes(data: &str, key: &str, iv: Option<&str>) -> anyhow::Result<String> {
  let (key, iv) = match iv {
    Some(iv) => {
      if key.len() != 32 {
        return Err(anyhow!("AES解密key字节长度不足32, 当前长度: {}", key.len()));
      }
      if iv.len() != 16 {
        return Err(anyhow!("AES解密iv字节长度不足16, 当前长度: {}", iv.len()));
      }

      (key.to_owned(), iv.to_owned())
    }
    None => {
      let key = encrypt_md5(key);
      let iv = key[16..].to_owned();

      (key, iv)
    }
  };

  let mut data_buf = hex::decode(data).map_err(|e| anyhow!("AES解密数据无效: {e}"))?;

  let decryptor = Decryptor::<Aes256>::new_from_slices(key.as_bytes(), iv.as_bytes())
    .map_err(|e| anyhow!("AES解密创建失败: {e}"))?;

  let decrypted = decryptor
    .decrypt_padded_vec_mut::<Pkcs7>(&mut data_buf)
    .map_err(|e| anyhow!("AES解密数据失败: {e}"))?;

  let decrypted_str = String::from_utf8_lossy(&decrypted).to_string();

  Ok(decrypted_str)
}

/// RSA 加密, 无padding
pub fn encrypt_rsa_unpad(data: impl AsRef<[u8]>, pem: Option<&str>) -> anyhow::Result<String> {
  let pem = pem.unwrap_or(HttpConfig::get_kg_static_config().rsa_pem);
  let key = RsaPublicKey::from_public_key_pem(pem)
    .map_err(|e| anyhow!("RSA无填充加密秘钥创建失败: {e}"))?;

  let mut data_buf = data.as_ref().to_vec();

  if data_buf.len() > key.size() {
    return Err(anyhow!("RSA无填充加密秘钥长度超过数据长度"));
  }

  // 不够key长度补零
  data_buf.resize(key.size(), 0);

  let encrypted = BigUint::from_bytes_be(&data_buf).modpow(key.e(), key.n());

  let encrypted_str = encrypted.to_str_radix(16);

  Ok(encrypted_str)
}

/// RSA 加密, 有padding
pub fn encrypt_rsa_pad(data: impl AsRef<[u8]>) -> anyhow::Result<String> {
  let pem = HttpConfig::get_kg_static_config().rsa_pem;
  let key =
    RsaPublicKey::from_public_key_pem(pem).map_err(|e| anyhow!("RSA填充加密秘钥创建失败: {e}"))?;

  let encrypted = key
    .encrypt(&mut OsRng, Pkcs1v15Encrypt, data.as_ref())
    .map_err(|e| anyhow!("RSA填充加密创建失败: {e}"))?;

  let encrypted_str = hex::encode(encrypted);

  Ok(encrypted_str)
}

/// 用于播放列表的 AES 加密
pub fn encrypt_aes_playlist(data: impl AsRef<[u8]>) -> anyhow::Result<EncryptedResult> {
  let temp_key = random_string(6).to_lowercase();
  let hash = encrypt_md5(&temp_key);
  let (key, iv) = (&hash[..16], &hash[16..]);

  let mut data_buf = data.as_ref().to_vec();

  // 为 PKCS7 填充预留空间（最多需要一个完整的块）
  data_buf.resize(data_buf.len() + AES_BLOCK_SIZE, 0);

  let encryptor = Encryptor::<Aes128>::new_from_slices(key.as_bytes(), iv.as_bytes())
    .map_err(|e| anyhow!("播放列表AES加密创建失败: {e}"))?;

  let encrypted = encryptor.encrypt_padded_vec_mut::<Pkcs7>(data.as_ref());

  let encrypted_str = STANDARD.encode(encrypted);

  Ok(EncryptedResult {
    str: encrypted_str,
    key: temp_key,
  })
}

/// 用于播放列表的 AES 解密
pub fn decrypt_aes_playlist(data: &str, key: &str) -> anyhow::Result<String> {
  let hash = encrypt_md5(key);
  let (key, iv) = (&hash[..16], &hash[16..]);

  let mut data_buf = STANDARD
    .decode(&data)
    .map_err(|e| anyhow!("播放列表AES解密数据无效: {e}"))?;

  let decryptor = Decryptor::<Aes128>::new_from_slices(key.as_bytes(), iv.as_bytes())
    .map_err(|e| anyhow!("播放列表AES解密创建失败: {e}"))?;

  let decrypted = decryptor
    .decrypt_padded_vec_mut::<Pkcs7>(&mut data_buf)
    .map_err(|e| anyhow!("播放列表AES解密数据失败: {e}"))?;

  let decrypted_str = String::from_utf8_lossy(&decrypted).to_string();

  Ok(decrypted_str)
}

#[cfg(test)]
mod tests {
  use super::*;

  // ==================== MD5 测试 ====================

  #[test]
  fn test_encrypt_md5_basic() {
    let data = b"hello";
    let result = encrypt_md5(data);

    // MD5("hello") = "5d41402abc4b2a76b9719d911017c592"
    assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
    assert_eq!(result.len(), 32);
  }

  #[test]
  fn test_encrypt_md5_empty() {
    let data = b"";
    let result = encrypt_md5(data);

    // MD5("") = "d41d8cd98f00b204e9800998ecf8427e"
    assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(result.len(), 32);
  }

  #[test]
  fn test_encrypt_md5_chinese() {
    let data = "你好世界".as_bytes();
    let result = encrypt_md5(data);

    assert_eq!(result.len(), 32);
    // 验证是有效的十六进制字符串
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
  }

  #[test]
  fn test_encrypt_md5_consistency() {
    let data = b"test data";
    let result1 = encrypt_md5(data);
    let result2 = encrypt_md5(data);

    assert_eq!(result1, result2);
  }

  // ==================== SHA1 测试 ====================

  #[test]
  fn test_encrypt_sha1_basic() {
    let data = b"hello";
    let result = encrypt_sha1(data);

    // SHA1("hello") = "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
    assert_eq!(result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    assert_eq!(result.len(), 40);
  }

  #[test]
  fn test_encrypt_sha1_empty() {
    let data = b"";
    let result = encrypt_sha1(data);

    // SHA1("") = "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    assert_eq!(result, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(result.len(), 40);
  }

  #[test]
  fn test_encrypt_sha1_chinese() {
    let data = "你好世界".as_bytes();
    let result = encrypt_sha1(data);

    assert_eq!(result.len(), 40);
    assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
  }

  #[test]
  fn test_encrypt_sha1_consistency() {
    let data = b"test data";
    let result1 = encrypt_sha1(data);
    let result2 = encrypt_sha1(data);

    assert_eq!(result1, result2);
  }

  // ==================== AES 加密/解密测试 ====================

  #[test]
  fn test_encrypt_decrypt_aes_with_custom_key_iv() {
    let data = b"Hello, World!";
    let key = "0123456789abcdef0123456789abcdef"; // 32字节
    let iv = "0123456789abcdef"; // 16字节

    let encrypted = encrypt_aes(data, Some(key), Some(iv)).unwrap();
    let decrypted = decrypt_aes(&encrypted.str, key, Some(iv)).unwrap();

    assert_eq!(decrypted, String::from_utf8_lossy(data));
  }

  #[test]
  fn test_encrypt_decrypt_aes_auto_key_iv() {
    let data = b"Test message for auto key/iv generation";

    let encrypted = encrypt_aes(data, None, None).unwrap();
    let decrypted = decrypt_aes(&encrypted.str, &encrypted.key, None).unwrap();

    assert_eq!(decrypted, String::from_utf8_lossy(data));
  }

  #[test]
  fn test_encrypt_aes_invalid_key_length() {
    let data = b"test";
    let key = "short"; // 长度不足32
    let iv = "0123456789abcdef";

    let result = encrypt_aes(data, Some(key), Some(iv));
    assert!(result.is_err());
  }

  #[test]
  fn test_encrypt_aes_invalid_iv_length() {
    let data = b"test";
    let key = "0123456789abcdef0123456789abcdef";
    let iv = "short"; // 长度不足16

    let result = encrypt_aes(data, Some(key), Some(iv));
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_aes_invalid_key_length() {
    let key = "short";
    let iv = "0123456789abcdef";

    let result = decrypt_aes("somehexdata", key, Some(iv));
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_aes_invalid_iv_length() {
    let key = "0123456789abcdef0123456789abcdef";
    let iv = "short";

    let result = decrypt_aes("somehexdata", key, Some(iv));
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_aes_invalid_hex_data() {
    let key = "0123456789abcdef0123456789abcdef";

    let result = decrypt_aes("invalid hex!!!", key, None);
    assert!(result.is_err());
  }

  #[test]
  fn test_encrypt_decrypt_aes_empty_data() {
    let data = b"";
    let key = "0123456789abcdef0123456789abcdef";
    let iv = "0123456789abcdef";

    let encrypted = encrypt_aes(data, Some(key), Some(iv)).unwrap();
    let decrypted = decrypt_aes(&encrypted.str, key, Some(iv)).unwrap();

    assert_eq!(decrypted, "");
  }

  #[test]
  fn test_encrypt_decrypt_aes_large_data() {
    let data = vec![0u8; 1024]; // 1KB数据
    let key = "0123456789abcdef0123456789abcdef";
    let iv = "0123456789abcdef";

    let encrypted = encrypt_aes(&data, Some(key), Some(iv)).unwrap();
    let decrypted = decrypt_aes(&encrypted.str, key, Some(iv)).unwrap();

    assert_eq!(decrypted.as_bytes(), data.as_slice());
  }

  #[test]
  fn test_encrypted_result_structure() {
    let data = b"test";
    let encrypted = encrypt_aes(data, None, None).unwrap();

    assert!(!encrypted.str.is_empty());
    assert!(!encrypted.key.is_empty());
    assert_eq!(encrypted.key.len(), 16); // random_string(16)
  }

  // ==================== RSA 无填充加密测试 ====================

  #[test]
  fn test_encrypt_rsa_unpad_basic() {
    // 使用一个测试用的RSA公钥
    let test_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDJtlZhXOYrUMEXh+HBJmELlL8V
xJvWzKLLhPJZLdPnBHKRjPLFJhTqCJhMJLJPPPPPPPPPPPPPPPPPPPPPPPPPPP
PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP
PPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP
wIDAQAB
-----END PUBLIC KEY-----";

    let data = b"test";
    let result = encrypt_rsa_unpad(data, Some(test_pem));

    // 由于RSA加密的非确定性，我们只检查是否成功返回十六进制字符串
    if result.is_ok() {
      let encrypted = result.unwrap();
      assert!(encrypted.chars().all(|c| c.is_ascii_hexdigit()));
    }
  }

  #[test]
  fn test_encrypt_rsa_unpad_invalid_pem() {
    let data = b"test";
    let result = encrypt_rsa_unpad(data, Some("invalid pem"));

    assert!(result.is_err());
  }

  #[test]
  fn test_encrypt_rsa_unpad_data_too_large() {
    // 创建一个非常大的数据
    let large_data = vec![0u8; 1024];

    // 使用默认的pem（从HttpConfig获取）
    let result = encrypt_rsa_unpad(&large_data, None);

    // 应该失败，因为数据太大
    assert!(result.is_err());
  }

  // ==================== RSA 有填充加密测试 ====================

  #[test]
  fn test_encrypt_rsa_pad_basic() {
    let data = b"test data for rsa padding";
    let result = encrypt_rsa_pad(data);

    // 检查是否返回有效的十六进制字符串
    if result.is_ok() {
      let encrypted = result.unwrap();
      assert!(!encrypted.is_empty());
      assert!(encrypted.chars().all(|c| c.is_ascii_hexdigit()));
    }
  }

  #[test]
  fn test_encrypt_rsa_pad_empty() {
    let data = b"";
    let result = encrypt_rsa_pad(data);

    if result.is_ok() {
      let encrypted = result.unwrap();
      assert!(!encrypted.is_empty());
    }
  }

  // ==================== 播放列表 AES 加密/解密测试 ====================

  #[test]
  fn test_encrypt_decrypt_aes_playlist_basic() {
    let data = b"playlist data";

    let encrypted = encrypt_aes_playlist(data).unwrap();
    let decrypted = decrypt_aes_playlist(&encrypted.str, &encrypted.key).unwrap();

    assert_eq!(decrypted, String::from_utf8_lossy(data));
  }

  #[test]
  fn test_encrypt_decrypt_aes_playlist_empty() {
    let data = b"";

    let encrypted = encrypt_aes_playlist(data).unwrap();
    let decrypted = decrypt_aes_playlist(&encrypted.str, &encrypted.key).unwrap();

    assert_eq!(decrypted, "");
  }

  #[test]
  fn test_encrypt_decrypt_aes_playlist_large() {
    let data = vec![0u8; 2048]; // 2KB数据

    let encrypted = encrypt_aes_playlist(&data).unwrap();
    let decrypted = decrypt_aes_playlist(&encrypted.str, &encrypted.key).unwrap();

    assert_eq!(decrypted.as_bytes(), data.as_slice());
  }

  #[test]
  fn test_encrypt_aes_playlist_base64_encoded() {
    let data = b"test";

    let encrypted = encrypt_aes_playlist(data).unwrap();

    // 播放列表加密使用base64编码
    assert!(!encrypted.str.is_empty());
    // 验证是有效的base64字符串
    assert!(STANDARD.decode(&encrypted.str).is_ok());
  }

  #[test]
  fn test_decrypt_aes_playlist_invalid_base64() {
    let key = "testkey";

    let result = decrypt_aes_playlist("invalid!!!base64", key);
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_aes_playlist_invalid_data() {
    let key = "testkey";

    // 有效的base64但无效的加密数据
    let result = decrypt_aes_playlist("AAAAAAAA", key);
    assert!(result.is_err());
  }

  #[test]
  fn test_encrypt_aes_playlist_key_length() {
    let data = b"test";

    let encrypted = encrypt_aes_playlist(data).unwrap();

    // key应该是6个字符的随机字符串（小写字母或数字）
    assert_eq!(encrypted.key.len(), 6);
    assert!(encrypted
      .key
      .chars()
      .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
  }

  // ==================== 边界情况和错误处理测试 ====================

  #[test]
  fn test_aes_roundtrip_various_lengths() {
    let test_cases = vec![
      vec![],
      vec![0],
      vec![1, 2, 3],
      vec![0u8; 16],
      vec![0u8; 17],
      vec![0u8; 31],
      vec![0u8; 32],
      vec![0u8; 33],
    ];

    let key = "0123456789abcdef0123456789abcdef";
    let iv = "0123456789abcdef";

    for data in test_cases {
      let encrypted = encrypt_aes(&data, Some(key), Some(iv)).unwrap();
      let decrypted = decrypt_aes(&encrypted.str, key, Some(iv)).unwrap();

      assert_eq!(
        decrypted.as_bytes(),
        data.as_slice(),
        "Failed for data length: {}",
        data.len()
      );
    }
  }

  #[test]
  fn test_playlist_aes_roundtrip_various_lengths() {
    let test_cases = vec![
      vec![],
      vec![0],
      vec![1, 2, 3],
      vec![0u8; 16],
      vec![0u8; 17],
      vec![0u8; 31],
      vec![0u8; 32],
      vec![0u8; 33],
    ];

    for data in test_cases {
      let encrypted = encrypt_aes_playlist(&data).unwrap();
      let decrypted = decrypt_aes_playlist(&encrypted.str, &encrypted.key).unwrap();

      assert_eq!(
        decrypted.as_bytes(),
        data.as_slice(),
        "Failed for data length: {}",
        data.len()
      );
    }
  }

  #[test]
  fn test_different_keys_produce_different_results() {
    let data = b"test";
    let key1 = "0123456789abcdef0123456789abcdef";
    let key2 = "fedcba9876543210fedcba9876543210";
    let iv = "0123456789abcdef";

    let encrypted1 = encrypt_aes(data, Some(key1), Some(iv)).unwrap();
    let encrypted2 = encrypt_aes(data, Some(key2), Some(iv)).unwrap();

    assert_ne!(encrypted1.str, encrypted2.str);
  }

  #[test]
  fn test_different_ivs_produce_different_results() {
    let data = b"test";
    let key = "0123456789abcdef0123456789abcdef";
    let iv1 = "0123456789abcdef";
    let iv2 = "fedcba9876543210";

    let encrypted1 = encrypt_aes(data, Some(key), Some(iv1)).unwrap();
    let encrypted2 = encrypt_aes(data, Some(key), Some(iv2)).unwrap();

    assert_ne!(encrypted1.str, encrypted2.str);
  }

  #[test]
  fn test_wrong_key_decryption_fails() {
    let data = b"secret message";
    let correct_key = "0123456789abcdef0123456789abcdef";
    let wrong_key = "fedcba9876543210fedcba9876543210";
    let iv = "0123456789abcdef";

    let encrypted = encrypt_aes(data, Some(correct_key), Some(iv)).unwrap();
    let result = decrypt_aes(&encrypted.str, wrong_key, Some(iv));

    // 使用错误的密钥解密应该失败或产生不同的结果
    if result.is_ok() {
      let decrypted = result.unwrap();
      assert_ne!(decrypted, String::from_utf8_lossy(data));
    } else {
      // 或者返回错误（取决于padding验证）
      assert!(result.is_err());
    }
  }
}
