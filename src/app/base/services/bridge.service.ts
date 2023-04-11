import { Injectable, OnDestroy } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { catchError, defer, from, map, Observable, of, Subject, takeUntil, tap } from 'rxjs';
import { appDataDir } from '@tauri-apps/api/path';
import { BaseDirectory, exists } from '@tauri-apps/api/fs';
import { envFileName } from '../../app-constants';
import { AWSCredentialFormValue, AwsCredentialsResponse } from '../../models';
import { Event, listen } from '@tauri-apps/api/event';
import { process } from '@tauri-apps/api';

export type Features = 'python' | 'docker' | 'aws' | 'git';
export const requiredFeatures: Features[] = ['python', 'docker', 'aws', 'git'];

@Injectable({
  providedIn: 'root'
})
export class BridgeService implements OnDestroy {
  private readonly eventSubject$ = new Subject<Event<boolean | unknown>>();
  events$ = this.eventSubject$.asObservable();
  private readonly destroy$ = new Subject<void>();

  readonly hasAllDependencies$: Observable<boolean> = defer(() => invoke<boolean>('check_aws_auth_file'));

  readonly getAwsCreds$ = defer(() => invoke<AwsCredentialsResponse>('get_aws_credentials')).pipe(
    catchError((err) => {
      console.error(err);
      return of({} as AwsCredentialsResponse);
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

  setAwsCred(
      {awsAccessKeyId, awsSecretAccessKey, region}: {
        awsAccessKeyId: string,
        awsSecretAccessKey: string,
        region: string
      }
  ) {
    return invoke<void>("write_aws_auth_to_file", {
      awsAccessKeyId,
      awsSecretAccessKey,
      region
    });
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
    return defer(() => invoke<string>('create_stack_file_repo')).pipe(
      map(() => true),
      catchError(e => of(false))
    );
  }
}
