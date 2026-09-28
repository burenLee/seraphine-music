/* oxlint-disable typescript/no-require-imports */
/**
 * 统一版本号管理脚本
 *
 * 用法:
 *   pn set-ver 0.1.3     # 将所有文件版本号设为 0.1.3
 *   pn check-ver         # CI 检查，不一致则报错退出
 */

const fs = require('fs');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');

const FILES = [
  {
    path: path.join('src-tauri', 'tauri.conf.json'),
    read: (c) => JSON.parse(c).version,
    write: (c, v) => {
      const conf = JSON.parse(c);
      conf.version = v;
      return JSON.stringify(conf, null, 2) + '\n';
    },
  },
  {
    path: 'package.json',
    read: (c) => JSON.parse(c).version,
    write: (c, v) => {
      const pkg = JSON.parse(c);
      pkg.version = v;
      return JSON.stringify(pkg, null, 2) + '\n';
    },
  },
  {
    path: path.join('src-tauri', 'Cargo.toml'),
    read: (c) => {
      const m = c.match(/^version\s*=\s*"([^"]+)"/m);
      return m ? m[1] : null;
    },
    write: (c, v) => c.replace(/^version\s*=\s*"[^"]+"/m, `version = "${v}"`),
  },
];

const args = process.argv.slice(2);
const targetVersion = args[0];

// ====== set 模式 ======
if (!targetVersion) {
  console.error('用法: pnpm set-version <版本号>');
  console.error('示例: pnpm set-version 0.1.3');
  process.exit(1);
}

// 校验 semver 格式（允许 v 前缀）
if (!/^v?\d+\.\d+\.\d+/.test(targetVersion)) {
  console.error(`❌ 无效的版本号: ${targetVersion}（期望 semver 格式，如 0.1.3）`);
  process.exit(1);
}

const cleanVersion = targetVersion.replace(/^v/, '');

console.log(`📌 统一设置版本号: ${cleanVersion}\n`);

for (const { path: file, read, write } of FILES) {
  const filePath = path.join(ROOT, file);
  const content = fs.readFileSync(filePath, 'utf-8');
  const current = read(content);

  if (current === cleanVersion) {
    console.log(`⏭️ ${file}: 已是 ${cleanVersion}`);
    continue;
  }

  const updated = write(content, cleanVersion);
  fs.writeFileSync(filePath, updated, 'utf-8');
  console.log(`✅ ${file}: ${current || '(未找到)'} → ${cleanVersion}`);
}

console.log('\n✨ 完成。');
