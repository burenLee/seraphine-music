/**
 * Tauri 打包封装脚本：自动从 .tauri/ 目录读取签名私钥并注入 TAURI_SIGNING_PRIVATE_KEY，
 * 避免每次手动 export 环境变量。
 *
 * 用法:
 *   pnpm tauri:build                 # 等价于 pnpm tauri build
 *   pnpm tauri:build --debug         # 透传额外参数给 tauri build
 *   pnpm tauri:build --target x86_64-pc-windows-msvc
 *
 * 说明:
 *   - 私钥查找规则：.tauri/ 目录下第一个 *.key 且不以 .pub 结尾的文件
 *   - 若已手动设置 TAURI_SIGNING_PRIVATE_KEY 环境变量，则优先使用该值（不覆盖）
 *   - TAURI_SIGNING_PRIVATE_KEY 支持传入密钥内容或文件路径，此处传入绝对路径
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const TAURI_DIR = path.join(ROOT, '.tauri');

// 在 .tauri/ 下查找私钥文件（*.key 且非 .pub）
function findPrivateKey() {
  if (!fs.existsSync(TAURI_DIR)) return null;
  const files = fs.readdirSync(TAURI_DIR);
  const keyFile = files.find((f) => f.endsWith('.key') && !f.endsWith('.pub'));
  return keyFile ? path.join(TAURI_DIR, keyFile) : null;
}

const privateKeyPath = findPrivateKey();

if (!privateKeyPath) {
  console.error('❌ 未在 .tauri/ 目录找到签名私钥文件（*.key）');
  console.error('   请将私钥放入 .tauri/ 目录，或手动设置 TAURI_SIGNING_PRIVATE_KEY 环境变量');
  process.exit(1);
}

// 仅当环境变量未设置时才注入，允许手动覆盖
if (!process.env.TAURI_SIGNING_PRIVATE_KEY) {
  process.env.TAURI_SIGNING_PRIVATE_KEY = privateKeyPath;
  console.log(`🔑 使用签名私钥: ${path.relative(ROOT, privateKeyPath)}`);
} else {
  console.log('🔑 使用环境变量 TAURI_SIGNING_PRIVATE_KEY 中指定的私钥');
}

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
