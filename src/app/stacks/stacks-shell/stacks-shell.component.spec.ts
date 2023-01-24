import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StacksShellComponent } from './stacks-shell.component';

describe('StacksShellComponent', () => {
  let component: StacksShellComponent;
  let fixture: ComponentFixture<StacksShellComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ StacksShellComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(StacksShellComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
