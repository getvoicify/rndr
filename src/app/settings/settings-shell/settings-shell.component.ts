import { ChangeDetectionStrategy, Component, HostBinding } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AwsCredentialsComponent } from '../aws-credentials/aws-credentials.component';

@Component({
  selector: 'app-settings-shell',
  standalone: true,
  imports: [CommonModule, AwsCredentialsComponent],
  template: `
    <app-aws-credentials></app-aws-credentials>
  `,
  styles: [
  ],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class SettingsShellComponent {
  @HostBinding('class') classes = 'flex flex-col flex-1 w-full h-full';
}
