import { Inject, Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api';
import {
  asyncScheduler,
  catchError,
  defer, map,
  merge, Observable,
  observeOn,
  of,
  retry,
  Subject,
  switchMap,
  tap,
  throwError,
  timer
} from 'rxjs';
import { appDataDir, homeDir } from '@tauri-apps/api/path';
import { Event, listen } from '@tauri-apps/api/event';
import { CreateStackResultModel, isErrorResult } from '../../models';
import { SNACKBAR_SERVICE_TOKEN, SnackbarService } from './snackbar.service';
import { Router } from '@angular/router';
import { coreEvent$ } from '../../core/install-event.logger';

@Injectable({
  providedIn: 'root'
})
export class StackService {

  hasStacksRepo$ = defer(() => invoke<boolean>('has_stack_file_repo')).pipe(
    catchError(e => of(false))
  );


  hasStacks$: Observable<boolean> = defer(() => invoke<string[]>('get_stack_list')).pipe(
    tap(stacks => console.log(stacks)),
    catchError(e => of([])),
    map(stacks => stacks.length > 0)
  );

  private readonly stackEventSub$: Subject<Event<CreateStackResultModel>> = new Subject<Event<CreateStackResultModel>>();
  stackEvent$ = coreEvent$<any>('create-stack').pipe(
    observeOn(asyncScheduler),
    tap((event) => {
      console.log(event);
      if (isErrorResult(event.payload)) {
        this.snackService.open(
          event.payload.stackStatus.Error,
          10000,
          'fill',
          'bottom',
          'center',
          'error',
          'Go to settings',
          async () => {
            await this.router.navigate(['/settings']);
            this.snackService.close();
          }
        );
        throw new Error(event.payload.stackStatus.Error);
      }
    }),
    switchMap(event => isErrorResult(event.payload) ? throwError(event.payload.stackStatus.Error) : of(event)),
    retry({
      delay: (error, retryCount) => {
        return retryCount === 10 ? throwError(error) : timer(1000 * retryCount);
      }
    }),
  );
  constructor(
    @Inject(SNACKBAR_SERVICE_TOKEN) private snackService: SnackbarService,
    private router: Router
  ) {
    listen<any>("create-stack", (event) => {
      this.stackEventSub$.next({
        ...event,
        payload: {
          ...event.payload ?? {},
          stackName: event.payload?.stack_name ?? '',
          stackStatus: event.payload?.stack_status ?? ''
        }
      });
    }).catch(this.stackEventSub$.error);

    merge(
      coreEvent$<string>("inbound://installing_dependency")
    ).pipe().subscribe(e => {
      console.log(e);
    })
  }

  async createStack(value: string) {
    const depsPath = await appDataDir();
    const stackFile = `${depsPath}.brh-ext-deps/aws/cloud-render-cloudformation.yml`;
    invoke('create_stack', { stackName: value, stackFile, depsPath }).catch(this.stackEventSub$.error);
  }
}
