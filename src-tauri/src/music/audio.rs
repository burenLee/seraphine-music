use anyhow::{anyhow, Result};
use rodio::{
  cpal::{default_host, traits::HostTrait},
  decoder::DecoderBuilder,
  source::SeekError,
  Decoder, Device, DeviceSinkBuilder, MixerDeviceSink, Player,
};
use std::{
  fs::File,
  io::{Read, Seek},
  time::Duration,
};

pub struct Audio {
  _sink: MixerDeviceSink,
  player: Player,
  device: Option<Device>,
}

impl Audio {
  pub fn new() -> Result<Self> {
    let device = default_host()
      .default_output_device()
      .ok_or_else(|| anyhow!("未识别到输出设备"))?;
    let sink = DeviceSinkBuilder::from_device(device.clone())?.open_stream()?;
    let player = Player::connect_new(&sink.mixer());

    // 默认是play状态,手动暂停
    player.pause();

    Ok(Self {
      _sink: sink,
      player,
      device: Some(device),
    })
  }

  pub fn current_device(&self) -> Option<&Device> {
    self.device.as_ref()
  }

  /// 获取全部音频输出设备
  pub fn all_devices(&self) -> Vec<Device> {
    let Ok(devices) = default_host().output_devices() else {
      return Vec::new();
    };

    devices.collect()
  }

  pub fn reload_device(&mut self, device: Device) -> Result<()> {
    let new_sink = DeviceSinkBuilder::from_device(device.clone())?.open_stream()?;
    let new_player = Player::connect_new(&new_sink.mixer());

    let volume = self.player.volume();
    let paused = self.player.is_paused();

    self._sink = new_sink;
    self.player = new_player;
    self.device = Some(device);

    self.player.set_volume(volume);
    if paused {
      self.player.pause();
    }

    Ok(())
  }

  /// 加载音频文件
  pub fn load_from_file(&mut self, path: &str) -> Result<()> {
    let file = File::open(path)?;
    let source = Decoder::try_from(file)?;

    self.player.append(source);

    Ok(())
  }

  /// 加载音频流
  pub fn load_from_stream<T>(&mut self, stream: T, file_size: u64) -> Result<()>
  where
    T: Read + Seek + Send + Sync + 'static,
  {
    let decoder = DecoderBuilder::new()
      .with_data(stream)
      .with_byte_len(file_size)
      .build()?;

    self.player.append(decoder);

    Ok(())
  }

  /// 播放音频
  pub fn play(&self) {
    self.player.play();
  }

  /// 暂停音频
  pub fn pause(&self) {
    self.player.pause();
  }

  /// 音频是否暂停
  pub fn paused(&self) -> bool {
    self.player.is_paused()
  }

  /// 停止音频
  pub fn stop(&mut self) {
    self.player.pause();
    let _ = self.player.try_seek(Duration::ZERO);
    self.player.stop();
  }

  /// 设置音频音量
  pub fn set_volume(&self, volume: f32) {
    self.player.set_volume(volume);
  }

  /// 获取音频进度
  pub fn get_pos(&self) -> Duration {
    self.player.get_pos()
  }

  /// 音频跳转
  pub fn try_seek(&self, pos: Duration) -> Result<(), SeekError> {
    self.player.try_seek(pos)
  }
}
