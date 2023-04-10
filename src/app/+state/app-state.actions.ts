import { AppStateModel, Loading } from './app-state.model';

export class GetAppState {
  static readonly type: string = '[APP-STATE]: Get App state';
}

export class SetAppState {
  static readonly type: string = '[APP-STATE]: Set App state';
  constructor(public state: AppStateModel) {}
}

export class PushLoading {
  static readonly type: string = '[APP-STATE]: Push loading';
  constructor(public loading: Loading) {}
}

/**
 * Usage:
 * Instantiate with a LocalLoading instance to clear a specific loader
 * or instantiate with an empty constructor to clear all loaders
 */
export class PopLoading {
  static readonly type: string = '[APP-STATE]: Pop loading';
  constructor(public loading?: Loading) {}
}



