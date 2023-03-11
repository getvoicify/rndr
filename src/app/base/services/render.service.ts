import { Injectable, OnDestroy } from '@angular/core';
import { BehaviorSubject, catchError, defer, EMPTY, filter, iif, of, Subject, switchMap, takeUntil, tap } from 'rxjs';
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
      filter(isLoadRenderEvent),
      switchMap(event => {
        const { file, config } = event.payload;
        const isFileBased = typeof file !== 'string';
        return iif(() => isFileBased, defer(() => this.getFileBinary(file as File)).pipe(
          switchMap(binary => this.saveBlenderFile((file as File).name, binary as Uint8Array)),
          switchMap(() => this.startRender( config, (file as File).name)),
        ), of(file as string).pipe(
          switchMap(path => this.startRender(config, undefined, path))
        ))
      }),
      tap(() => this.createRenderEventSub$.next({status: 'success'})),
      catchError(error => {
        this.createRenderEventSub$.next({
          status: 'error',
          error
        });
        return EMPTY;
      }),
      takeUntil(this.destroy$)
    ).subscribe();
  }

  private async  saveBlenderFile(filename: string, binary: Uint8Array) {
    const dir = await appDataDir();
    await invoke('create_blender_file', { filePath: `${dir}.config/.blender/${filename}` });
    await writeBinaryFile(`.config/.blender/${filename}`, binary, {dir: BaseDirectory.AppData});
  }

  private async startRender(config: CreateRenderConfig, filename?: string, path?: string) {
    const dir = await appDataDir();
    const depsPath = `${dir}.brh-ext-deps/rendercli`;
    const jobList = `${dir}.config/.joblist.csv`;
    let filePath: string;
    if (filename) {
      filePath = `${dir}.config/.blender/${filename}`;
    } else {
      filePath = path ?? '';
    }
    await invoke('start_render', { filePath, config, depsPath, jobList });
  }

  createRender(file: File | string, config: CreateRenderConfig) {
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
