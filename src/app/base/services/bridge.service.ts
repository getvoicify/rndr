import { Injectable, OnDestroy } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import {
  catchError,
  combineLatest,
  defer,
  distinctUntilChanged,
  from,
  map,
  Observable,
  of,
  Subject,
  takeUntil
} from 'rxjs';
import { appDataDir } from '@tauri-apps/api/path';
import { BaseDirectory, createDir, exists, removeDir } from '@tauri-apps/api/fs';
import { Command } from '@tauri-apps/api/shell';
import { environment } from '../../../environments/environment';
import { envFileName } from '../../app-constants';
import { AWSCredentialFormValue } from '../../models';
import { Event, listen } from '@tauri-apps/api/event';
import { StackService } from './stack.service';
import { process } from '@tauri-apps/api';

export type Features = 'python' | 'docker' | 'aws' | 'git';
export const requiredFeatures: Features[] = ['python', 'docker', 'aws', 'git'];

@Injectable({
  providedIn: 'root'
})
export class BridgeService implements OnDestroy {
  private readonly eventSubject$ = new Subject<Event<boolean | unknown>>();
  events$ = this.eventSubject$.asObservable();
  constructor(private stackService: StackService) { }
  private readonly destroy$ = new Subject<void>();
  readonly installSubject$ = new Subject<string>();
  checkFeature$: (feature: Features) => Observable<boolean> = (feature: Features) => defer(() => invoke<boolean>('check_os_feature', {feature}));
  private readonly checkRequiredFeatures$: Observable<boolean[]> = combineLatest(
    requiredFeatures.map(feature => this.checkFeature$(feature).pipe(
      distinctUntilChanged()
    ))
  );

  readonly hasAllDependencies$: Observable<boolean> = defer(() => invoke<boolean>('has_dependencies'));
  canRun$: Observable<boolean> = this.checkRequiredFeatures$.pipe(
    map(features => features.every(feature => feature))
  );

  features$: Observable<Record<Features, boolean>> = this.checkRequiredFeatures$.pipe(
    map((features: boolean[]) => {
      const result = {} as Record<Features, boolean>;
      features.forEach((feature, index) => {
        result[requiredFeatures[index] as Features] = feature;
      });
      return result;
    })
  );

  private readonly fileExists$ = (fileName: string, dir: BaseDirectory = BaseDirectory.AppData) =>
    defer(() => exists(fileName, { dir }));

  hasExtDeps$: Observable<boolean> = this.fileExists$('.brh-ext-deps').pipe(
    catchError((err) => {
      console.error(err);
      return of(false);
    })
  );

  extDepsGitInit$: Observable<boolean> = defer(() => exists(`.brh-ext-deps/.git`, { dir: BaseDirectory.AppData })).pipe(
    catchError((err) => {
      console.error(err);
      return of(false);
    })
  );

  hasEnv$(env: string): Observable<boolean> {
    return defer(() => invoke<boolean>('check_env_var', {name: env}));
  }

  getEnv$ = (env: string): Observable<string> => defer(() => invoke<string>('get_env_var', {name: env}));

  async setAwsEnv(param: AWSCredentialFormValue) {
    const dir = await appDataDir();
    const keys = Object.keys(param) as (keyof AWSCredentialFormValue)[];
    const promises = keys.map(key => invoke('add_or_update_env_var', {
      fileName: `${dir}.config/${envFileName}`,
      key,
      value: param[key]
    }));
    const result = await Promise.all(promises);
    console.log('DONE', result);
  }

  listenToEvent<T>(event: string): Observable<Event<T>> {
    return new Observable<Event<T>>(subscriber => {
      listen<T>(event, (event) => {
        subscriber.next(event);
      }).catch(err => subscriber.error(err));
    });
  }

  async processRender() {
    const event$ = new Subject<Event<boolean | unknown>>();

    await listen<boolean>('update-process', (event) => {
      event$.next(event);
    });

    const dir = await appDataDir();
    const cli = `${dir}.brh-ext-deps/rendercli`;
    const jobList = `${dir}.config/.joblist.csv`;

    from(invoke('process_render', {depsPath: cli, jobList}))
      .pipe(
        takeUntil(this.destroy$)
      )
      .subscribe({
      error: (err) => console.error(err)
    });

    event$.asObservable().pipe(
      takeUntil(this.destroy$),
    ).subscribe({
      next: (event) => {
        this.eventSubject$.next(event);
        this.handleEvent(event);
      }
    });
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }

  private async handleEvent(event: Event<boolean | unknown>) {
    if (typeof event.payload === 'string' && this.noJobList(event as Event<string>)) {
      await this.relaunch().catch(console.error);
      return;
    }
  }

  private noJobList(event: Event<string>): boolean {
    return event.payload.includes('FileNotFoundError') && event.payload.includes('.joblist.csv');
  }

  async relaunch() {
    await process.exit(0);
  }

  async getAppDataDir() {
    return await appDataDir();
  }

  async openExternal(url: string) {
    await invoke('open_url', {url});
  }

  startInstallation(): Observable<boolean> {
    return defer(() => invoke<boolean>('start_installation'));
  }
}