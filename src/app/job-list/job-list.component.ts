import { ChangeDetectionStrategy, Component, HostBinding, OnInit, QueryList, ViewChildren } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveComponent } from '../base/reactive.component';
import { RenderJobListService } from './render-job-list.service';
import { MatIconModule } from '@angular/material/icon';
import { tap } from 'rxjs';
import { RouterModule } from '@angular/router';
import { JobItemComponent } from './job-item/job-item.component';
import { PageResponse } from '../models';
import { Job } from '../models/job';

@Component({
  selector: 'app-job-list',
  standalone: true,
  imports: [CommonModule, MatIconModule, RouterModule, JobItemComponent],
  templateUrl: './job-list.component.html',
  styleUrls: ['./job-list.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class JobListComponent extends ReactiveComponent implements OnInit {
  @HostBinding('class') class = 'flex flex-col flex-1 overflow-hidden items-center';
  @ViewChildren(JobItemComponent) jobItems?: QueryList<JobItemComponent>;
  state: {
    jobs?: PageResponse<Job>
  } = this.connect({
    jobs: this.jobsService.jobs$.pipe(
      tap(console.log)
    )
  });
  constructor(
    private jobsService: RenderJobListService
  ) {
    super();
  }

  override ngOnInit(): void {
    super.ngOnInit();
    this.jobsService.getJobs();
  }

  closeOtherMenus(id: string) {
    this.jobItems?.filter(item => item.uniqueId !== id).forEach(item => item.closeMenu());
  }
}
