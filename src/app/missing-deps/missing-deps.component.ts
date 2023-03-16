import { Component, HostBinding, OnInit } from '@angular/core';
import { ReactiveComponent } from '../base/reactive.component';
import { BridgeService } from '../base/services';
import { Router } from '@angular/router';
import { LoaderComponent } from '../shared/ui';
import { interval, Observable, startWith, switchMap } from 'rxjs';
import { AsyncPipe } from '@angular/common';

@Component({
  selector: 'app-missing-deps',
  standalone: true,
  templateUrl: './missing-deps.component.html',
  imports: [
    LoaderComponent,
    AsyncPipe
  ],
  styleUrls: ['./missing-deps.component.scss']
})
export class MissingDepsComponent extends ReactiveComponent implements OnInit {

  arrayOfTexts = ['Warming up the GPUs', 'Sharpening pencils', 'Watching tutorials', 'Coffee break'];

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
    startWith(this.arrayOfTexts[0]),
  );
  state = this.connect({});

  constructor(
    private bridgeService: BridgeService,
    private router: Router,
  ) {
    super();
  }

  override ngOnInit() {
    super.ngOnInit();
    this.bridgeService.startInstallation()
      .pipe(
        switchMap(() => this.router.navigate(['/']))
      )
      .subscribe();
  }

  @HostBinding('class') get hostClasses() {
    return 'grid w-full h-full relative';
  }
}