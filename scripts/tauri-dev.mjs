import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

const args = process.argv.slice(2);

const localTauriBin = join(
  process.cwd(),
  'node_modules',
  '.bin',
  process.platform === 'win32' ? 'tauri.cmd' : 'tauri',
);

const tauriCmd = existsSync(localTauriBin) ? localTauriBin : 'tauri';

const child = spawn(tauriCmd, ['dev', ...args], {
  stdio: 'inherit',
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
