import { ChangeDetectionStrategy, Component, HostBinding, HostListener, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { appWindow } from '@tauri-apps/api/window';
import { distinctUntilChanged, from, iif, map, scan, shareReplay, startWith, Subject, switchMap, tap } from 'rxjs';
import { BridgeService } from '../../../base/services';
import { ReactiveComponent } from '../../../base/reactive.component';

@Component({
  selector: 'rndr-toolbar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './toolbar.component.html',
  styleUrls: ['./toolbar.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ToolbarComponent extends ReactiveComponent implements OnInit {

  maximizeEventSub$ = new Subject<void>();

  maximizeEvent$ = this.maximizeEventSub$.asObservable().pipe(
    scan((acc) => !acc, false),
    switchMap(shouldMaximise =>
      iif(() => shouldMaximise, from(appWindow.maximize()), from(appWindow.unmaximize()))
    )
  );

  @HostBinding('class') class = 'flex flex-row items-center w-full p-[1rem] text-white select-none cursor-move fixed top-0 left-0';

  focused$ = this.bridgeService.listenToEvent<boolean>('focused').pipe(
    map(focused => focused.payload),
    tap(focused => {
      console.log('focused', focused)
    }),
    distinctUntilChanged(),
    shareReplay(1),
    startWith(true)
  );

  state = this.connect({
    focused: this.focused$,
  })

  constructor(private bridgeService: BridgeService) {
    super();
  }

  override ngOnInit() {
    super.ngOnInit();
    this.maximizeEvent$.subscribe();
  }

  async close() {
    await appWindow.close();
  }

  async minimize() {
    await appWindow.minimize();
  }

  async maximize() {
    await appWindow.maximize();
  }

  async unmaximize() {
    await appWindow.unmaximize();
  }

  toggleMaximize() {
    this.maximizeEventSub$.next();
  }
}
