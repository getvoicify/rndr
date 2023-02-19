import { Inject, Injectable } from '@angular/core';
import { BridgeService } from '../base/services';
import { defer, exhaustMap, map, merge, Observable, Subject, tap, throttleTime } from 'rxjs';
import { invoke } from '@tauri-apps/api';
import { PageResponse, WINDOW_TOKEN } from '../models';
import { Job, jobMapper, JobRaw } from '../models/job';

type GetJobConfig = {
  order: 'asc' | 'desc';
  page: number;
  perPage: number;
}

@Injectable({
  providedIn: 'root',
})
export class RenderJobListService {

  getJobsConfig: GetJobConfig = {
    order: 'desc',
    page: 1,
    perPage: 10
  }
  getJobListSubject$ = new Subject<void>();
  jobs$: Observable<PageResponse<Job>> = merge(
    this.bridgeService.events$,
    this.getJobListSubject$
  ).pipe(
    throttleTime(1000 * 30),
    exhaustMap(() => defer(() => this.getJobsFn(
      this.getJobsConfig.order, this.getJobsConfig.page, this.getJobsConfig.perPage))
    ),
    map(({ data, total_count }) => ({
      data: data.map(jobMapper),
      total_count
    })),
    tap(console.log)
  );
  constructor(
    private readonly bridgeService: BridgeService,
    @Inject(WINDOW_TOKEN) private readonly window: Window
  ) { }

  private async getJobsFn(order: 'asc' | 'desc' = 'desc', page: number = 1, perPage: number = 10) {
    const appDataDir = await this.bridgeService.getAppDataDir();
    const filePath = `${appDataDir}.config/.joblist.csv`;
    return invoke<PageResponse<JobRaw>>('parse_csv', { filePath, order, page, perPage });
  }

  getJobs(order: 'asc' | 'desc' = 'desc', page: number = 1, perPage: number = 10) {
    this.getJobsConfig = { order, page, perPage };
    this.getJobListSubject$.next();
  }

  private async hashResponseFn(value: any): Promise<string> {
    const str = typeof value === 'string' ? value : JSON.stringify(value);
    const encoder = new TextEncoder();
    const data = encoder.encode(str);
    const { window } = this;
    const buffer = await window.crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(buffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  }
}
