import { Injectable, OnDestroy } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import {
  catchError,
  combineLatest,
  defer,
  distinctUntilChanged, filter,
  from,
  map,
  Observable,
  of, skipUntil,
  Subject, switchMap,
  takeUntil, tap, timer
} from 'rxjs';
import { appDataDir } from '@tauri-apps/api/path';
import { BaseDirectory, createDir, exists, removeDir } from '@tauri-apps/api/fs';
import { Command } from '@tauri-apps/api/shell';
import { environment } from '../../../environments/environment';
import { envFileName } from '../../app-constants';
import { AWSCredentialFormValue, AWSEnvForm } from '../../models';
import { Event, listen } from '@tauri-apps/api/event';
import { StackService } from './stack.service';

export type Features = 'python' | 'docker' | 'aws' | 'git';
export const requiredFeatures: Features[] = ['python', 'docker', 'aws', 'git'];

@Injectable({
  providedIn: 'root'
})
export class BridgeService implements OnDestroy {
  constructor(private stackService: StackService) { }
  private readonly destroy$ = new Subject<void>();
  readonly installSubject$ = new Subject<string>();
  checkFeature$: (feature: Features) => Observable<boolean> = (feature: Features) => defer(() => invoke<boolean>('check_os_feature', {feature}));
  private readonly checkRequiredFeatures$: Observable<boolean[]> = combineLatest(
    requiredFeatures.map(feature => this.checkFeature$(feature).pipe(
      distinctUntilChanged()
    ))
  );
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

  async installExtDeps() {
    const subject = this.installSubject$;
    const dir = await appDataDir();
    const extDepsDir = `${dir}.brh-ext-deps`;
    const command = new Command('git-clone', ['clone', environment.depsRepo, extDepsDir]);

    const depsDirExists = await exists('.brh-ext-deps', { dir: BaseDirectory.AppData });

    if (depsDirExists) {
      await removeDir('.brh-ext-deps', { recursive: true, dir: BaseDirectory.AppData });
    }
    await createDir('.brh-ext-deps', { recursive: true, dir: BaseDirectory.AppData });


    command.on('close', ({code}: { code: number, signal?: unknown }) => {
      console.log(code);
      if (code === 0) {
        subject.complete();
      } else {
        subject.error(new Error(`git clone exited with code ${code}`));
      }
    });

    command.stdout.on('data', line => console.log(`command stdout: "${line}"`));
    command.stderr.on('data', line => console.error(`command stderr: "${line}"`));

    await command.execute();

    command.on('error', (error) => {
      subject.error(error);
    });

    return subject.asObservable();
  }

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

  async processRender() {
    const event$ = new Subject<Event<boolean>>();

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
      filter((event: Event<boolean>) => event.payload),
      takeUntil(this.destroy$),
    ).subscribe({
      next: (event) => console.log(event)
    });
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }
}