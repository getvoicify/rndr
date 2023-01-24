import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AwsCredentialsComponent } from './aws-credentials.component';

describe('AwsCredentialsComponent', () => {
  let component: AwsCredentialsComponent;
  let fixture: ComponentFixture<AwsCredentialsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ AwsCredentialsComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AwsCredentialsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
