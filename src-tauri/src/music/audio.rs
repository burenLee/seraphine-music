use anyhow::anyhow;
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
  output_device: Device, // 输出设备
  _sink: MixerDeviceSink,
  player: Player,
}

impl Audio {
  /// ## 构建音频实例
  ///
  /// 使用默认输出设备, 构建后为暂停状态
  pub fn new() -> anyhow::Result<Self> {
    let output_device = default_host()
      .default_output_device()
      .ok_or(anyhow!("未找到任何输出设备"))?;
    let sink_builder = DeviceSinkBuilder::from_device(output_device.clone())?;
    let sink = sink_builder.open_stream()?;
    let player = Player::connect_new(&sink.mixer());

    // 默认是play状态, 手动暂停
    player.pause();

    Ok(Self {
      output_device,
      _sink: sink,
      player,
    })
  }

  /// ## 获取全部输出设备
  pub fn output_devices(&self) -> anyhow::Result<Vec<Device>> {
    let devices = default_host().output_devices()?;

    Ok(devices.collect())
  }

  /// ## 获取当前输出设备
  pub fn output_device(&self) -> &Device {
    &self.output_device
  }

  /// ## 重载输出设备
  ///
  /// 会保留原有的音量与播放状态
  ///
  /// ### 必选参数
  /// * `output_device` - 输出设备
  pub fn reload_device(&mut self, output_device: Device) -> anyhow::Result<()> {
    let sink_builder = DeviceSinkBuilder::from_device(output_device.clone())?;
    let sink = sink_builder.open_stream()?;
    let player = Player::connect_new(&sink.mixer());

    // 从旧player恢复状态
    player.set_volume(self.player.volume());
    if self.player.is_paused() {
      player.pause();
    }

    self.output_device = output_device;
    self._sink = sink;
    self.player = player;

    Ok(())
  }

  /// ## 加载音频文件
  ///
  /// ### 必选参数
  /// * `path` - 音频文件路径
  pub fn load_from_file(&mut self, path: &str) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let source = Decoder::try_from(file)?;

    self.player.append(source);

    Ok(())
  }

  /// ## 加载音频流
  ///
  /// ### 必选参数
  /// * `stream` - 音频流
  /// * `file_size` - 音频文件总大小
  pub fn load_from_stream<T>(&mut self, stream: T, file_size: u64) -> anyhow::Result<()>
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

  /// ## 播放音频
  pub fn play(&self) {
    self.player.play();
  }

  /// ## 暂停音频
  pub fn pause(&self) {
    self.player.pause();
  }

  /// ## 音频是否暂停
  pub fn paused(&self) -> bool {
    self.player.is_paused()
  }

  /// ## 停止音频
  pub fn stop(&mut self) {
    let _ = self.player.try_seek(Duration::ZERO);
    self.player.pause();
    self.player.stop();
  }

  /// ## 获取当前音量
  pub fn volume(&self) -> f32 {
    self.player.volume()
  }

  /// ## 设置音频音量
  ///
  /// ### 必选参数
  /// * `volume` - 音量, 取值范围为 0 ~ 1
  pub fn set_volume(&self, volume: f32) {
    self.player.set_volume(volume);
  }

  /// ## 获取音频进度
  pub fn get_pos(&self) -> Duration {
    self.player.get_pos()
  }

  /// ## 音频跳转
  ///
  /// ### 必选参数
  /// * `pos` - 跳转的播放位置
  pub fn try_seek(&self, pos: Duration) -> anyhow::Result<(), SeekError> {
    self.player.try_seek(pos)
  }
}
