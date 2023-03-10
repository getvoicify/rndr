import { Component, HostBinding, HostListener, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { appWindow } from '@tauri-apps/api/window';
import { from, iif, scan, Subject, switchMap } from 'rxjs';

@Component({
  selector: 'rndr-toolbar',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './toolbar.component.html',
  styleUrls: ['./toolbar.component.scss']
})
export class ToolbarComponent implements OnInit {

  maximizeEventSub$ = new Subject<void>();

  maximizeEvent$ = this.maximizeEventSub$.asObservable().pipe(
    scan((acc) => !acc, false),
    switchMap(shouldMaximise =>
      iif(() => shouldMaximise, from(appWindow.maximize()), from(appWindow.unmaximize()))
    )
  );

  @HostBinding('class') class = 'flex flex-row items-center w-full p-[1rem] bg-[#000000] text-white user-select-none cursor-move fixed top-0 left-0';

  ngOnInit() {
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
