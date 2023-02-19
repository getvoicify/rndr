import { ChangeDetectionStrategy, Component, HostBinding, NgZone, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { StackService } from '../../base/services';
import { filter, interval, Observable, startWith, Subject, takeUntil, tap } from 'rxjs';
import { isCompleteResult } from '../../models';
import { Router } from '@angular/router';

@Component({
  selector: 'app-stack-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './stack-list.component.html',
  styleUrls: ['./stack-list.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class StackListComponent implements OnDestroy {
  private readonly destroy$ = new Subject<void>();

  @HostBinding('class') get hostClass() {
    return 'grid place-items-center w-full h-full';
  }

  arrayOfTexts = ['Warming up the GPUs', 'Sharpening pencils', 'Watching YouTube videos', 'Coffee break', '...'];

  cycleTexts$ = new Observable<string>(subscriber => {
    let i = 0;
    let prevIndex = -1;

    const intervalSubscription = interval(5000).subscribe(() => {
      let index = Math.floor(Math.random() * this.arrayOfTexts.length);
      while (index === prevIndex) {
        index = Math.floor(Math.random() * this.arrayOfTexts.length);
      }
      prevIndex = index;
      subscriber.next(this.arrayOfTexts[index]);
      i++;
    });

    return () => {
      intervalSubscription.unsubscribe();
    };
  }).pipe(
    takeUntil(this.destroy$),
    startWith(this.arrayOfTexts[0]),
  );

  constructor(private stackService: StackService, router: Router, zone: NgZone) {
    this.stackService.stackEvent$.pipe(
      filter(event => isCompleteResult(event.payload)),
      tap(event => {
        zone.run(() => router.navigate(['/'])).catch(console.error);
      }),
      takeUntil(this.destroy$)
    ).subscribe();
    this.createStack('render-stack').catch(console.error);
  }

  async createStack(value?: string) {
    if (!value) {
      return;
    }
    await this.stackService.createStack(value);
  }

  ngOnDestroy() {
    this.destroy$.next();
    this.destroy$.complete();
  }
}
