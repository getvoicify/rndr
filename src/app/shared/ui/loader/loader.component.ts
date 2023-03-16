import { ChangeDetectionStrategy, Component, HostBinding, Input } from '@angular/core';
import { CommonModule } from '@angular/common';

type mode = 'determinate' | 'indeterminate' | 'buffer' | 'query';
@Component({
  selector: 'rndr-loader',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './loader.component.html',
  styleUrls: ['./loader.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class LoaderComponent {
  @Input() mode: mode = 'indeterminate';
  @Input() progress?: number = 0;

  @HostBinding() class = this.mode;

  get progressStyle() {
    if (!this.progress) {
      return '0%';
    }
    if (this.progress >= 100) {
      return '100%';
    }
    return `${this.progress}%`;
  }
}
