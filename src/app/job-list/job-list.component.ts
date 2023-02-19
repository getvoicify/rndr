import { ChangeDetectionStrategy, Component, HostBinding, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveComponent } from '../base/reactive.component';
import { RenderJobListService } from './render-job-list.service';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-job-list',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './job-list.component.html',
  styleUrls: ['./job-list.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class JobListComponent extends ReactiveComponent implements OnInit {
  @HostBinding('class') class = 'flex flex-col flex-1 overflow-hidden items-center';
  state = this.connect({
    jobs: this.jobsService.jobs$
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
}
