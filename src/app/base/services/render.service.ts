import { Injectable, OnDestroy } from '@angular/core';
import { BehaviorSubject, catchError, defer, EMPTY, filter, Subject, switchMap, takeUntil, tap } from 'rxjs';
import { CreateRenderConfig, CreateRenderEvent, isLoadRenderEvent } from '../../models/render';
import { BaseDirectory, writeBinaryFile } from '@tauri-apps/api/fs';
import { appDataDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';

@Injectable({
  providedIn: 'root'
})
export class RenderService implements OnDestroy {
  destroy$ = new Subject<void>();
  private readonly createRenderEventSub$ = new BehaviorSubject<CreateRenderEvent>({
    status: 'idle'
  });

  createRenderEvent$ = this.createRenderEventSub$.asObservable();

  constructor() {
    this.createRenderEvent$.pipe(
      tap(console.log),
      filter(isLoadRenderEvent),
      switchMap(event => defer(() => this.getFileBinary(event.payload.file)).pipe(
        switchMap(binary => this.saveBlenderFile(event.payload.file.name, binary as Uint8Array)),
        switchMap(() => this.startRender(event.payload.file.name, event.payload.config)),
        tap(() => this.createRenderEventSub$.next({status: 'success'})),
        catchError(error => {
          this.createRenderEventSub$.next({
            status: 'error',
            error
          });
          return EMPTY;
        })
      )),
      takeUntil(this.destroy$)
    ).subscribe();
  }

  private async  saveBlenderFile(filename: string, binary: Uint8Array) {
    const dir = await appDataDir();
    await invoke('create_blender_file', { filePath: `${dir}.config/.blender/${filename}` });
    await writeBinaryFile(`.config/.blender/${filename}`, binary, {dir: BaseDirectory.AppData});
  }

  private async startRender(filename: string, config: CreateRenderConfig) {
    const dir = await appDataDir();
    const depsPath = `${dir}.brh-ext-deps/rendercli`;
    const jobList = `${dir}.config/.joblist.csv`;
    await invoke('start_render', { filePath: `${dir}.config/.blender/${filename}`, config, depsPath, jobList });
  }

  createRender(file: File, config: CreateRenderConfig) {
    this.createRenderEventSub$.next({
      status: 'loading',
      payload: {
        file,
        config
      }
    });
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }



  getFileBinary(file: File): Promise<Uint8Array> {
    // Create a FileReader object
    const reader = new FileReader();
    return new Promise((resolve, reject) => {
      reader.onload = () => {
        resolve(new Uint8Array(reader.result as ArrayBuffer));
      };
      reader.onerror = reject;
      reader.readAsArrayBuffer(file);
    });
  }
}
