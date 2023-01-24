import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  template: `
    <div class="flex flex-col w-full h-full">
      <router-outlet></router-outlet>
    </div>
  `,
  styles: [],
  standalone: true,
  imports: [
    RouterOutlet
  ]
})
export class AppComponent {
}

