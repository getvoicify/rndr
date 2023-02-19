import { InjectionToken } from '@angular/core';

export const WINDOW_TOKEN = new InjectionToken<Window>('WINDOW_TOKEN', {
  providedIn: 'root',
  factory: () => window ?? {} as Window
})