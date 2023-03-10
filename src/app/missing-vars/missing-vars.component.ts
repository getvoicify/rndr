import { ChangeDetectionStrategy, Component, HostBinding } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { BridgeService } from '../base/services';
import { AWSEnvForm } from '../models';
import { Router } from '@angular/router';

@Component({
  selector: 'app-missing-vars',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './missing-vars.component.html',
  styleUrls: ['./missing-vars.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class MissingVarsComponent {

  private reg = [
    {name: 'US East (Ohio) w GPU', code: 'us-east-2'},
    {name: 'US East (N. Virginia)  w GPU', code: 'us-east-1'},
    {name: 'US West (N. California) w GPU', code: 'us-west-1'},
    {name: 'US West (Oregon) w GPU', code: 'us-west-2'},
    {name: 'Canada (Central) w GPU', code: 'ca-central-1'},
    {name: 'EU (Frankfurt) w GPU', code: 'eu-central-1'},
    {name: 'EU (Ireland) w GPU', code: 'eu-west-1'},
    {name: 'EU (London) w GPU', code: 'eu-west-2'},
    {name: 'EU (Paris)', code: 'eu-west-3'},
    {name: 'Asia Pacific (Mumbai) w GPU', code: 'ap-south-1'},
    {name: 'Asia Pacific (Seoul) w GPU', code: 'ap-northeast-2'},
    {name: 'Asia Pacific (Singapore) w GPU', code: 'ap-southeast-1'},
    {name: 'Asia Pacific (Sydney) w GPU', code: 'ap-southeast-2'},
    {name: 'Asia Pacific (Tokyo) w GPU', code: 'ap-northeast-1'},
    {name: 'South America (São Paulo) w GPU', code: 'sa-east-1'},
  ];
  get awsRegions() {
    return this.reg.sort(
      (a, b) => (a.name > b.name) ? 1 : -1)
      .filter(r => r.name.includes('GPU'))
      .map(r => r.code);
  }
  awsFormGroup = new FormGroup<AWSEnvForm>({
    accessKey: new FormControl('', { nonNullable: true, validators: [Validators.required] }),
    secretKey: new FormControl('', { nonNullable: true, validators: [Validators.required] }),
    region: new FormControl('', { nonNullable: true, validators: [Validators.required] })
  });

  @HostBinding('class') get hostClasses() {
    return 'grid w-full h-full place-content-center';
  }

  constructor(private bridgeService: BridgeService, private router: Router) { }

  async setAwsEnv() {
    if (!this.awsFormGroup.valid) {
      console.error('Invalid form');
      return;
    }
    const values = this.awsFormGroup.value;
    await this.bridgeService.setAwsEnv({
      'AWS_ACCESS_KEY_ID': values.accessKey ?? '',
      'AWS_SECRET_ACCESS_KEY': values.secretKey ?? '',
      'AWS_REGION': values.region ?? ''
    });

    await this.router.navigate(['/']);
  }
}
