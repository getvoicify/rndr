import { Component, HostBinding, NgZone, OnDestroy } from '@angular/core';
import { ReactiveComponent } from '../base/reactive.component';
import { BridgeService } from '../base/services';
import { catchError, map, of, startWith, Subject, switchMap, takeUntil } from 'rxjs';
import { JsonPipe, NgIf } from '@angular/common';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-missing-deps',
  standalone: true,
  templateUrl: './missing-deps.component.html',
  imports: [
    JsonPipe,
    NgIf
  ],
  styleUrls: ['./missing-deps.component.scss']
})
export class MissingDepsComponent extends ReactiveComponent implements OnDestroy {
  private readonly destroy$ = new Subject<void>();
  private readonly triggerInstallExtDeps$ = new Subject<void>();
  state = this.connect({
    missingDeps: this.bridgeService.features$,
    installExtDeps: this.activatedRoute.queryParamMap.pipe(
      map(queryParamMap => queryParamMap.get('install-ext-deps') === 'true')
    ),
    hasExtDeps: this.triggerInstallExtDeps$.pipe(
      startWith(false),
      switchMap(() => this.bridgeService.installExtDeps()),
      map(() => true),
      catchError((err) => {
        console.error(err);
        return of(false);
      })
    ),
  });

  constructor(
    private activatedRoute: ActivatedRoute,
    private bridgeService: BridgeService,
    private router: Router,
    private zone: NgZone,
  ) {
    super();
    this.bridgeService.installSubject$.pipe(
      takeUntil(this.destroy$),
    ).subscribe({
      complete: async () => {
        await this.zone.run( () => this.router.navigate(['']));
      }
    });
  }

  @HostBinding('class') get hostClasses() {
    return 'grid w-full h-full place-content-center';
  }

  installExtDeps() {
    this.triggerInstallExtDeps$.next();
  }

  override ngOnDestroy() {
    super.ngOnDestroy();
    this.destroy$.next();
    this.destroy$.complete();
  }
}