import { FormControl } from '@angular/forms';

export type AWSEnvForm = {
  awsAccessKeyId: FormControl<string>;
  awsSecretAccessKey: FormControl<string>;
  region: FormControl<string>;
}

export type AWSCredentialFormValue = {
  'AWS_ACCESS_KEY_ID': string;
  'AWS_SECRET_ACCESS_KEY': string;
  'AWS_REGION': string;
}
