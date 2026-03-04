import * as tauriApp from '@tauri-apps/api/app';
import * as tauriPath from '@tauri-apps/api/path';
import * as tauriWebviewWindow from '@tauri-apps/api/webviewWindow';
import * as tauriDialog from '@tauri-apps/plugin-dialog';
import * as tauriFs from '@tauri-apps/plugin-fs';
import * as tauriNotification from '@tauri-apps/plugin-notification';
import * as tauriOs from '@tauri-apps/plugin-os';
import * as tauriShell from '@tauri-apps/plugin-shell';
import * as tauriStore from '@tauri-apps/plugin-store';
import * as tauriAutostart from '@tauri-apps/plugin-autostart';
import * as tauriUpdater from '@tauri-apps/plugin-updater';
import * as tauriProcess from '@tauri-apps/plugin-process';
import * as tauriOpener from '@tauri-apps/plugin-opener';
import * as tauriLog from '@tauri-apps/plugin-log';
import { addImports, defineNuxtModule } from 'nuxt/kit';

interface ModuleOptions {
  prefix: false | string;
}

const capitalize = (name: string) => {
  return name.charAt(0).toUpperCase() + name.slice(1);
};

const tauriModules = [
  { module: tauriApp, prefix: 'App', importPath: '@tauri-apps/api/app' },
  { module: tauriPath, prefix: 'Path', importPath: '@tauri-apps/api/path' },
  { module: tauriWebviewWindow, prefix: 'WebviewWindow', importPath: '@tauri-apps/api/webviewWindow' },
  { module: tauriShell, prefix: 'Shell', importPath: '@tauri-apps/plugin-shell' },
  { module: tauriOs, prefix: 'Os', importPath: '@tauri-apps/plugin-os' },
  { module: tauriNotification, prefix: 'Notification', importPath: '@tauri-apps/plugin-notification' },
  { module: tauriFs, prefix: 'Fs', importPath: '@tauri-apps/plugin-fs' },
  { module: tauriStore, prefix: 'Store', importPath: '@tauri-apps/plugin-store' },
  { module: tauriDialog, prefix: 'Dialog', importPath: '@tauri-apps/plugin-dialog' },
  { module: tauriAutostart, prefix: 'Autostart', importPath: '@tauri-apps/plugin-autostart' },
  { module: tauriUpdater, prefix: 'Updater', importPath: '@tauri-apps/plugin-updater' },
  { module: tauriProcess, prefix: 'Process', importPath: '@tauri-apps/plugin-process' },
  { module: tauriOpener, prefix: 'Opener', importPath: '@tauri-apps/plugin-opener' },
  { module: tauriLog, prefix: 'Log', importPath: '@tauri-apps/plugin-log' },
];

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: 'nuxt-tauri',
    configKey: 'tauri',
  },
  defaults: {
    prefix: 'useTauri',
  },
  setup(options) {
    tauriModules.forEach(({ module, prefix, importPath }) => {
      Object.keys(module).filter((name) => name !== 'default')
        .forEach((name) => {
          const prefixedName = `${options.prefix}${prefix}` || '';
          const as = prefixedName ? prefixedName + capitalize(name) : name;
          addImports({ from: importPath, name, as });
        });
    });
  },
});
