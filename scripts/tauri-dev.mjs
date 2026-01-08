import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

const args = process.argv.slice(2);

const env = { ...process.env };

// Fix for WebKitGTK DMA-BUF rendering issues on Wayland (Linux cross-platform compatibility).
// See: https://github.com/tauri-apps/tauri/issues/9014
// Can be disabled via: NO_WEBKIT_DMABUF_FIX=1 npm run tauri:dev
if (
  process.platform === 'linux' &&
  env.WAYLAND_DISPLAY &&
  !env.NO_WEBKIT_DMABUF_FIX &&
  !env.WEBKIT_DISABLE_DMABUF_RENDERER
) {
  env.WEBKIT_DISABLE_DMABUF_RENDERER = '1';
}

const localTauriBin = join(
  process.cwd(),
  'node_modules',
  '.bin',
  process.platform === 'win32' ? 'tauri.cmd' : 'tauri',
);

const tauriCmd = existsSync(localTauriBin) ? localTauriBin : 'tauri';

const child = spawn(tauriCmd, ['dev', ...args], {
  stdio: 'inherit',
  env,
  shell: process.platform === 'win32',
});

child.on('error', (err) => {
  console.error(err);
  process.exit(1);
});

child.on('exit', (code, signal) => {
  if (signal) process.kill(process.pid, signal);
  process.exit(code ?? 1);
});
