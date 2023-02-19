import { Component, HostBinding, NgZone, OnDestroy } from '@angular/core';
import { ReactiveComponent } from '../base/reactive.component';
import { BridgeService, Features } from '../base/services';
import { catchError, map, of, startWith, Subject, switchMap, takeUntil } from 'rxjs';
import { JsonPipe, NgIf, NgTemplateOutlet } from '@angular/common';
import { ActivatedRoute, Router } from '@angular/router';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-missing-deps',
  standalone: true,
  templateUrl: './missing-deps.component.html',
  imports: [
    JsonPipe,
    NgIf,
    NgTemplateOutlet,
    MatButtonModule
  ],
  styleUrls: ['./missing-deps.component.scss']
})
export class MissingDepsComponent extends ReactiveComponent implements OnDestroy {

  private readonly urls: Record<Features, string> = {
    aws: 'https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html',
    docker: 'https://docs.docker.com/get-docker/',
    git: 'https://git-scm.com/downloads',
    python: 'https://www.python.org/downloads/',
  }

  private readonly destroy$ = new Subject<void>();
  state = this.connect({
    missingDeps: this.bridgeService.features$
  });

  constructor(
    private activatedRoute: ActivatedRoute,
    private bridgeService: BridgeService,
    private router: Router,
    private zone: NgZone,
  ) {
    super();
  }

  @HostBinding('class') get hostClasses() {
    return 'grid w-full h-full place-content-center';
  }

  override ngOnDestroy() {
    super.ngOnDestroy();
    this.destroy$.next();
    this.destroy$.complete();
  }

  async showInstallationInfo(feature: Features) {
    await this.bridgeService.openExternal(this.urls[feature]);
  }

  async close() {
    await this.bridgeService.relaunch();
  }
}