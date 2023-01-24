import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MissingVarsComponent } from './missing-vars.component';

describe('MissingVarsComponent', () => {
  let component: MissingVarsComponent;
  let fixture: ComponentFixture<MissingVarsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ MissingVarsComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MissingVarsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
