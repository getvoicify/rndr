import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RenderShellComponent } from './render-shell.component';

describe('RenderShellComponent', () => {
  let component: RenderShellComponent;
  let fixture: ComponentFixture<RenderShellComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ RenderShellComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RenderShellComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
