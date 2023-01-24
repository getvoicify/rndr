import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { APP_INITIALIZER, Provider } from '@angular/core';
const requestPermissionFn = async () => {
  const hasPermission = await isPermissionGranted();
  if (!hasPermission) {
    console.info('Requesting notification permission');
    await requestPermission().then(permission => console.log(permission)).catch(console.error);
  }
}

export const notificationPermissionInitializerProvider: Provider = {
  provide: APP_INITIALIZER,
  useFactory: () => requestPermissionFn,
  multi: true
}