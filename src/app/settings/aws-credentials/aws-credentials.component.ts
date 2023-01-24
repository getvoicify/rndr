import { ChangeDetectionStrategy, Component, HostBinding, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveComponent } from '../../base/reactive.component';
import { AWSCredentialFormValue } from '../../models';
import { BehaviorSubject, combineLatest, map, startWith, Subject, switchMap, tap } from 'rxjs';
import { BridgeService } from '../../base/services';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';

@Component({
  selector: 'app-aws-credentials',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './aws-credentials.component.html',
  styleUrls: ['./aws-credentials.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class AwsCredentialsComponent extends ReactiveComponent {
  loading$ = new BehaviorSubject<boolean>(false);
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
  getCredentials$ = new Subject<void>();
  @HostBinding('class') classes = 'content-container flex flex-col w-full';
  private keys: (keyof AWSCredentialFormValue)[] = [
    'AWS_ACCESS_KEY_ID',
    'AWS_SECRET_ACCESS_KEY',
    'AWS_REGION'
  ]
  state = this.connect({
    loading: this.loading$,
    awsEnv: this.getCredentials$.pipe(
      switchMap(() => combineLatest(this.keys.map(key => this.bridgeService.getEnv$(key))).pipe(
        map(values => values.reduce((acc, value, i) => ({ ...acc, [this.keys[i]]: value }), {} as AWSCredentialFormValue)),
        startWith({} as AWSCredentialFormValue),
        tap(value => {
          const keys = Object.keys(value);
          this.loading$.next(keys.length === 0);
        }),
        tap(this.updateForm.bind(this))
      ))
    ),
  });

  secretFieldType = 'password';

  awsFormGroup = inject(FormBuilder).group({
    AWS_ACCESS_KEY_ID: [this.state.awsEnv?.AWS_ACCESS_KEY_ID, { nonNullable: true, validators: [Validators.required] }],
    AWS_SECRET_ACCESS_KEY: [this.state.awsEnv?.AWS_SECRET_ACCESS_KEY, { nonNullable: true, validators: [Validators.required] }],
    AWS_REGION: [this.state.awsEnv?.AWS_REGION, { nonNullable: true, validators: [Validators.required] }]
  })

  constructor(private bridgeService: BridgeService) {
    super();
    this.getCredentials$.next();
  }

  updateForm(value: AWSCredentialFormValue) {
    this.awsFormGroup.patchValue(value);
  }

  async save() {
    this.loading$.next(true);
    const values = this.awsFormGroup.value;
    await this.bridgeService.setAwsEnv({
      'AWS_ACCESS_KEY_ID': values.AWS_ACCESS_KEY_ID ?? '',
      'AWS_SECRET_ACCESS_KEY': values.AWS_SECRET_ACCESS_KEY ?? '',
      'AWS_REGION': values.AWS_REGION ?? ''
    });
    this.getCredentials$.next();
    this.loading$.next(false);
  }
}
