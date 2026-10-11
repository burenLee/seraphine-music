/**
 * Tauri 打包封装脚本：自动加载项目根目录 .env 文件并校验签名私钥环境变量。
 *
 * 用法:
 *   pnpm tauri:build                 # 等价于 pnpm tauri build
 *   pnpm tauri:build --debug         # 透传额外参数给 tauri build
 *   pnpm tauri:build --target x86_64-pc-windows-msvc
 *
 * 说明:
 *   - 在项目根目录创建 .env 文件（已加入 .gitignore），内容示例：
 *       TAURI_SIGNING_PRIVATE_KEY="untrusted comment: rsign encrypted secret key
 *       RWT....="
 *       TAURI_SIGNING_PRIVATE_KEY_PASSWORD=your-password
 *   - 私钥由 tauri signer generate 生成，多行内容需用双引号包裹
 *   - TAURI_SIGNING_PRIVATE_KEY 支持密钥内容或文件路径两种形式
 *   - 已手动设置的环境变量优先，.env 不会覆盖已有值
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const ENV_FILE = path.join(ROOT, '.env');

// Node 内置 .env 加载（v20.12+）；不覆盖已存在的环境变量
if (fs.existsSync(ENV_FILE)) {
  process.loadEnvFile(ENV_FILE);
  console.log('📄 已加载 .env 文件');
}

if (!process.env.TAURI_SIGNING_PRIVATE_KEY) {
  console.error('❌ 未设置 TAURI_SIGNING_PRIVATE_KEY');
  console.error('   请在项目根目录 .env 文件中配置，或手动设置该环境变量');
  process.exit(1);
}

console.log('🔑 使用 TAURI_SIGNING_PRIVATE_KEY 中指定的私钥');

// 直接用 Node 运行 tauri CLI 的 JS 入口，跨平台无需 shell（Windows 下 spawn .cmd 需 shell 且有弃用警告）
const tauriJs = path.join(ROOT, 'node_modules', '@tauri-apps', 'cli', 'tauri.js');

if (!fs.existsSync(tauriJs)) {
  console.error(`❌ 未找到 tauri CLI: ${tauriJs}`);
  console.error('   请先执行 pnpm install');
  process.exit(1);
}

// 透传参数：pnpm tauri:build --debug --target xxx → tauri build --debug --target xxx
const args = ['build', ...process.argv.slice(2)];

console.log(`🚀 执行: tauri ${args.join(' ')}\n`);

const child = spawn(process.execPath, [tauriJs, ...args], {
  stdio: 'inherit',
  cwd: ROOT,
  env: process.env,
});

child.on('exit', (code) => process.exit(code ?? 0));
child.on('error', (err) => {
  console.error('启动 tauri 失败:', err.message);
  process.exit(1);
});
