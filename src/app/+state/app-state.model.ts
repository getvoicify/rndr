import { loadJson } from '@angular/compiler-cli/ngcc/src/utils';


export interface LocalLoading {
  mode: 'local';
  id: string;
}

export interface GlobalLoading {
  mode: 'global';
}

export type Loading = LocalLoading | GlobalLoading;

export const isLocalLoading = (loading: Loading): loading is LocalLoading => {
  return loading.mode === 'local';
}

export const isGlobalLoading = (loading: Loading): loading is GlobalLoading => {
  return loading.mode === 'global';
}

export interface AppStateModel {
  hasStacks: boolean;
  loading: Loading[]
}