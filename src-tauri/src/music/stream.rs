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
  pub fn new(file: File, file_size: u64, downloaded_size: Arc<AtomicU64>) -> Self {
    Self {
      file,
      file_size,
      downloaded_size,
    }
  }

  fn wait_for_data(&self, target_position: u64) -> Result<()> {
    let start_time = Instant::now();
    let mut sleep_duration = SLEEP_DURATION;

    while target_position > self.downloaded_size.load(Ordering::Acquire) {
      if start_time.elapsed() > TIMEOUT {
        return Err(Error::new(
          ErrorKind::TimedOut,
          String::from("等待下载超时"),
        ));
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
