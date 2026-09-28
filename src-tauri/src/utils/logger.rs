/// ## 记录错误(error)日志
#[macro_export]
macro_rules! log_err {
  ($target:expr, $err:expr, $msg:expr) => {
    ::log::error!(target: $target, "{}: {}", $msg, $err)
  };
}

/// ## 记录预期内的中断/降级(warn)日志
#[macro_export]
macro_rules! log_warn {
  ($target:expr, $err:expr, $msg:expr) => {
    ::log::warn!(target: $target, "{}: {}", $msg, $err)
  };
}

/// ## 记录关键流程节点(info)日志
#[macro_export]
macro_rules! log_info {
  ($target:expr, $err:expr, $msg:expr) => {
    ::log::info!(target: $target, "{}: {}", $msg, $err)
  };
}

pub trait LogErrExt<T, E> {
  /// ## 记录错误(error)并原样返回
  ///
  /// ### 必选参数
  /// * `target` - 日志目标
  /// * `msg` - 日志信息
  fn log_err(self, target: &str, msg: &str) -> Result<T, E>;

  /// ## 记录预期内的中断/降级(warn)并原样返回
  ///
  /// ### 必选参数
  /// * `target` - 日志目标
  /// * `msg` - 日志信息
  fn log_warn(self, target: &str, msg: &str) -> Result<T, E>;

  /// ## 记录关键流程节点(info)并原样返回
  ///
  /// ### 必选参数
  /// * `target` - 日志目标
  /// * `msg` - 日志信息
  fn log_info(self, target: &str, msg: &str) -> Result<T, E>;

  /// ## 用于 command 函数的错误记录和返回
  ///
  /// ### 必选参数
  /// * `target` - 日志目标
  /// * `msg` - 日志信息
  fn log_command(self, target: &str, msg: &str) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> LogErrExt<T, E> for Result<T, E> {
  fn log_err(self, target: &str, msg: &str) -> Result<T, E> {
    match self {
      Ok(value) => Ok(value),
      Err(e) => {
        log_err!(target, e, msg);

        Err(e)
      }
    }
  }

  fn log_warn(self, target: &str, msg: &str) -> Result<T, E> {
    match self {
      Ok(value) => Ok(value),
      Err(e) => {
        log_warn!(target, e, msg);

        Err(e)
      }
    }
  }

  fn log_info(self, target: &str, msg: &str) -> Result<T, E> {
    match self {
      Ok(value) => Ok(value),
      Err(e) => {
        log_info!(target, e, msg);

        Err(e)
      }
    }
  }

  fn log_command(self, target: &str, msg: &str) -> Result<T, String> {
    match self {
      Ok(value) => Ok(value),
      Err(e) => {
        log_err!(target, e, msg);

        Err(e.to_string())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::LogErrExt;

  #[test]
  fn log_err_ok_passthrough() {
    let result: Result<i32, &str> = Ok(42);
    assert_eq!(result.log_err("t", "msg"), Ok(42));
  }

  #[test]
  fn log_err_err_preserved() {
    let result: Result<i32, &str> = Err("boom");
    assert_eq!(result.log_err("t", "msg"), Err("boom"));
  }

  #[test]
  fn log_warn_err_preserved() {
    let result: Result<i32, &str> = Err("boom");
    assert_eq!(result.log_warn("t", "msg"), Err("boom"));
  }

  #[test]
  fn log_info_ok_passthrough() {
    let result: Result<i32, &str> = Ok(42);
    assert_eq!(result.log_info("t", "msg"), Ok(42));
  }

  #[test]
  fn log_command_ok_passthrough() {
    let result: Result<i32, &str> = Ok(42);
    assert_eq!(result.log_command("t", "msg"), Ok(42));
  }

  #[test]
  fn log_command_err_converted_to_string() {
    let result: Result<i32, &str> = Err("boom");
    assert_eq!(result.log_command("t", "msg"), Err("boom".to_string()));
  }
}
