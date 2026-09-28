use std::{
  fs::File,
  io::{Error, ErrorKind, Read, Result, Seek, SeekFrom},
  sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
  },
  thread,
  time::{Duration, Instant},
};

// 超时时间
const TIMEOUT: Duration = Duration::from_millis(5000);
// 第一次休眠时间
const SLEEP_DURATION: Duration = Duration::from_millis(10);
// 最大休眠时间
const MAX_SLEEP_DURATION: Duration = Duration::from_millis(100);

pub struct StreamFile {
  file: File,
  file_size: u64,
  downloaded_size: Arc<AtomicU64>,
}

impl StreamFile {
  /// ## 构建流式文件
  ///
  /// ### 必选参数
  /// * `file` - 本地缓存文件
  /// * `file_size` - 文件总大小
  /// * `downloaded_size` - 已下载的文件大小
  pub fn new(file: File, file_size: u64, downloaded_size: Arc<AtomicU64>) -> Self {
    Self {
      file,
      file_size,
      downloaded_size,
    }
  }

  /// ## 等待数据下载
  ///
  /// ### 必选参数
  /// * `target_position` - 需要读取到的文件位置
  fn wait_for_data(&self, target_position: u64) -> Result<()> {
    let start_time = Instant::now();
    let mut sleep_duration = SLEEP_DURATION;

    while target_position > self.downloaded_size.load(Ordering::Acquire) {
      if start_time.elapsed() > TIMEOUT {
        return Err(Error::new(ErrorKind::TimedOut, "等待下载超时".to_string()));
      }

      thread::sleep(sleep_duration);

      if sleep_duration < MAX_SLEEP_DURATION {
        sleep_duration = (sleep_duration * 2).min(MAX_SLEEP_DURATION);
      }
    }

    Ok(())
  }
}

impl Read for StreamFile {
  /// ## 读取流式数据
  ///
  /// ### 必选参数
  /// * `buf` - 读取数据的缓冲区
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    if buf.is_empty() {
      return Ok(0);
    }

    let current_pos = self.file.stream_position()?;
    if current_pos >= self.file_size {
      return Ok(0);
    }

    let bytes_to_read = (buf.len() as u64).min(self.file_size - current_pos);
    if bytes_to_read == 0 {
      return Ok(0);
    }

    let next_pos = current_pos + bytes_to_read;
    self.wait_for_data(next_pos)?;

    self.file.read(&mut buf[..bytes_to_read as usize])
  }
}

impl Seek for StreamFile {
  /// ## 跳转流式数据
  ///
  /// ### 必选参数
  /// * `pos` - 跳转的位置
  fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
    let target_pos = match pos {
      SeekFrom::Start(offset) => offset.min(self.file_size),
      SeekFrom::Current(offset) => {
        let current = self.file.stream_position()?;
        if offset >= 0 {
          (current + offset as u64).min(self.file_size)
        } else {
          current.saturating_sub(offset.unsigned_abs())
        }
      }
      SeekFrom::End(offset) => {
        if offset <= 0 {
          self
            .file_size
            .saturating_sub(offset.unsigned_abs().min(self.file_size))
        } else {
          self.file_size
        }
      }
    };

    if target_pos >= self.file_size {
      return self.file.seek(SeekFrom::Start(self.file_size));
    }

    self.wait_for_data(target_pos)?;
    self.file.seek(SeekFrom::Start(target_pos))
  }
}

#[cfg(test)]
mod tests {
  use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    sync::{
      atomic::{AtomicU64, Ordering},
      Arc,
    },
  };

  use super::*;

  fn setup(content: &[u8]) -> (StreamFile, tempfile::TempDir) {
    let temp = tempfile::tempdir().unwrap();
    let file_path = temp.path().join("stream.bin");
    fs::write(&file_path, content).unwrap();
    let file = File::open(&file_path).unwrap();
    let downloaded = Arc::new(AtomicU64::new(content.len() as u64));

    (StreamFile::new(file, content.len() as u64, downloaded), temp)
  }

  #[test]
  fn read_returns_whole_content() {
    let (mut stream, _temp) = setup(b"hello world");
    let mut buf = [0u8; 64];

    let n = stream.read(&mut buf).unwrap();

    assert_eq!(n, 11);
    assert_eq!(&buf[..n], b"hello world");
  }

  #[test]
  fn read_empty_buf_returns_zero() {
    let (mut stream, _temp) = setup(b"hello");
    let mut buf = [0u8; 0];

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
  }

  #[test]
  fn read_at_eof_returns_zero() {
    let (mut stream, _temp) = setup(b"hello");
    let mut buf = [0u8; 16];

    stream.read(&mut buf).unwrap();
    assert_eq!(stream.read(&mut buf).unwrap(), 0);
  }

  #[test]
  fn seek_start_clamps_beyond_end() {
    let (mut stream, _temp) = setup(b"hello");

    let pos = stream.seek(SeekFrom::Start(100)).unwrap();

    assert_eq!(pos, 5);
  }

  #[test]
  fn seek_end_negative_offset() {
    let (mut stream, _temp) = setup(b"hello");

    let pos = stream.seek(SeekFrom::End(-2)).unwrap();

    assert_eq!(pos, 3);
  }

  #[test]
  fn seek_current_forward() {
    let (mut stream, _temp) = setup(b"hello");

    let pos = stream.seek(SeekFrom::Current(2)).unwrap();

    assert_eq!(pos, 2);
  }
}
