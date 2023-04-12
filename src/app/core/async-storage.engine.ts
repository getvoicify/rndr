import { Store as TauriStore } from "tauri-plugin-store-api";
import { defer, map, Observable } from 'rxjs';
import { AsyncStorageEngine } from './async-store/symbols';

export class TauriAsyncStorageEngine implements AsyncStorageEngine {

  private store: TauriStore;

  constructor() {
    this.store = new TauriStore('.brh.dat')
  }

  length(): Observable<number> {
    return defer(() => this.store.length());
  }

  getItem(key: string): Observable<any> {
    return defer(() => this.store.get(key));
  }

  setItem(key: any, val: any): void {
    this.store.set(key, val).then(() => this.store.save());
  }

  removeItem(key: any): void {
    this.store.delete(key).then(() => this.store.save())
  }

  clear(): void {
    this.store.clear().then(() => this.store.save())
  }

  key(val: number): Observable<string> {
    return defer(() => this.store.keys()).pipe(
      map(keys => keys[val] ?? '')
    )
  }

}