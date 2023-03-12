import { ChangeDetectionStrategy, Component, EventEmitter, HostBinding, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Job } from '../../models/job';
import { MatIconModule } from '@angular/material/icon';
import { TruncateFilePathPipe } from '../../shared/ui/truncate-file-path.pipe';
import { BehaviorSubject } from 'rxjs';
import * as uuid from 'uuid';

@Component({
  selector: 'rndr-job-item',
  standalone: true,
  imports: [CommonModule, MatIconModule, TruncateFilePathPipe],
  templateUrl: './job-item.component.html',
  styleUrls: ['./job-item.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class JobItemComponent {
  @Input() job?: Job;
  @HostBinding('class') class = 'flex flex-row items-center w-full p-[1rem]';
  readonly uniqueId: string = uuid.v4();
  @Output() onMenuOpened = new EventEmitter<string>();

  menuStateClasses = '';
  openedStateClasses = 'transform transition ease-out duration-100 opacity-100 scale-100';
  closedStateClasses = 'transform transition ease-in duration-75 opacity-0 scale-0';
  private readonly menuClassesSub$ = new BehaviorSubject<string>(this.closedStateClasses);
  menuStateClasses$ = this.menuClassesSub$.asObservable();

  closeMenu() {
    this.menuClassesSub$.next(this.closedStateClasses);
  }

  toggleMenuState() {
    const currentClasses = this.menuClassesSub$.value;
    if (currentClasses === this.closedStateClasses) {
      this.menuClassesSub$.next(this.openedStateClasses);
      this.onMenuOpened.emit(this.uniqueId);
    } else {
      this.menuClassesSub$.next(this.closedStateClasses);
    }
  }

  download(job: Job) {
    if (job.status === 'Success') {
      console.log('download', job);
      this.closeMenu();
    }
  }
}
