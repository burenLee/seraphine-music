/** 登录页模式 */
export const enum LoginMode {
  Code,
  Form,
}

/** 二维码状态 */
export const enum QrcodeStatus {
  Ready,
  Scan,
  Timeout,
  Confirm,
  Fail,
}

/** 二维码类型 */
export const enum QrcodeType {
  KG,
  QQ,
  WX,
}

/** 扫描状态 */
export const enum ScanStatus {
  Ready,
  Loading,
  Fail,
  Success,
}

/** 概念版vip等级 */
export const enum YouthVip {
  Not,
  Dvip,
  Qvip,
  Tvip,
  Svip,
}
