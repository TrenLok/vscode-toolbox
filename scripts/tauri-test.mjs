import { readFile, writeFile } from 'node:fs/promises';
import { spawn } from 'node:child_process';

const testConfigPath = 'src-tauri/tauri.test.conf.json';
const generatedConfigPath = 'src-tauri/tauri.test.generated.conf.json';
const tauriCommand = process.argv.at(2) ?? 'build';
const testConfig = JSON.parse(await readFile(testConfigPath, 'utf8'));
const timestamp = Math.floor(Date.now() / 1000);
const version = `${testConfig.version}.${timestamp}`;

testConfig.version = version;
testConfig.mainBinaryName = 'vscode-toolbox-test';
await writeFile(generatedConfigPath, `${JSON.stringify(testConfig, null, 2)}\n`);

console.log(`Running test app ${tauriCommand} version ${version}`);

const command = process.platform === 'win32'
  ? (process.env.ComSpec ?? 'cmd.exe')
  : 'pnpm';
const args = process.platform === 'win32'
  ? ['/d', '/s', '/c', `pnpm tauri ${tauriCommand} --config ${generatedConfigPath}`]
  : ['tauri', tauriCommand, '--config', generatedConfigPath];

const tauriBuild = spawn(command, args, { stdio: 'inherit' });

tauriBuild.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }

  // eslint-disable-next-line unicorn/no-process-exit
  process.exit(code ?? 1);
});
