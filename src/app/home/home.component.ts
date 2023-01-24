import { Component, HostBinding } from '@angular/core';
import { ReactiveComponent } from '../base/reactive.component';
import { RouterLink, RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
  standalone: true,
  imports: [
    RouterOutlet,
    RouterLink
  ]
})
export class HomeComponent extends ReactiveComponent {
  @HostBinding('class') get hostClasses() {
    return 'flex w-full h-full';
  }
}