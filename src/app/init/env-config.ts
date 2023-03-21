import { envFileName } from '../app-constants';
import { BaseDirectory, createDir, exists, writeFile } from '@tauri-apps/api/fs';
import { APP_INITIALIZER, Provider } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import { appDataDir } from '@tauri-apps/api/path';
import { BridgeService } from '../base/services';

const checkENvFileFn = async (service: BridgeService) => {
  // has dir?
  const hasDir = await exists('.config', { dir: BaseDirectory.AppData });

  if (!hasDir) {
    // create dir
    await createDir('.config', { dir: BaseDirectory.AppData });
  }

  const hasEnvFile = await exists(`.config/${envFileName}`, { dir: BaseDirectory.AppData });
  if (!hasEnvFile) {
    console.info('Creating env file');
    await writeFile(`.config/${envFileName}`, '', { dir: BaseDirectory.AppData });
  }
  await service.processRender();
}

export const checkEnvFileProvider: Provider = {
  provide: APP_INITIALIZER,
  useFactory: (service: BridgeService) => async () => checkENvFileFn(service),
  multi: true,
  deps: [BridgeService]
}