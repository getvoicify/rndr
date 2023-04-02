import { ChangeDetectionStrategy, Component, HostBinding, OnInit } from '@angular/core';
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
export class MissingVarsComponent implements OnInit{

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
    awsAccessKeyId: new FormControl('', { nonNullable: true, validators: [Validators.required] }),
    awsSecretAccessKey: new FormControl('', { nonNullable: true, validators: [Validators.required] }),
    region: new FormControl('', { nonNullable: true, validators: [Validators.required] })
  });

  @HostBinding('class') get hostClasses() {
    return 'grid w-full h-full place-content-center';
  }

  constructor(private bridgeService: BridgeService, private router: Router) { }

  ngOnInit(): void {
    this.bridgeService.getAwsCreds$.subscribe(creds => {
      for (const k in creds) {
        const key = k as keyof AWSEnvForm;
        if (!creds[key]) {
          continue;
        }
        this.awsFormGroup.controls[key].setValue(creds[key]!);
      }
    });
  }

  async setAwsEnv() {
    if (!this.awsFormGroup.valid) {
      console.error('Invalid form');
      return;
    }
    const { awsAccessKeyId, awsSecretAccessKey, region } = this.awsFormGroup.value;
    try {
      await this.bridgeService.setAwsCred({
        awsSecretAccessKey: awsSecretAccessKey ?? '',
        awsAccessKeyId: awsAccessKeyId ?? '',
        region: region ?? this.reg[0].code
      });
    } catch (e) {
      console.error(e);
      return;
    }

    await this.router.navigate(['/']);
  }
}
