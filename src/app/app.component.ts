import { Component, HostBinding } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { ToolbarComponent } from './shared/ui';

@Component({
  selector: 'app-root',
  template: `
    <rndr-toolbar data-tauri-drag-region></rndr-toolbar>
    <div class="flex flex-col w-full wrapper mt-[56px]">
      <router-outlet></router-outlet>
    </div>
  `,
  styles: [`
    .wrapper {
      height: calc(100% - 56px);
    }
    
  `],
  standalone: true,
  imports: [
    RouterOutlet,
    ToolbarComponent
  ]
})
export class AppComponent {
  @HostBinding('class') class = 'bg-transparent w-full h-full block';
}

