import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StartRenderComponent } from './start-render.component';

describe('StartRenderComponent', () => {
  let component: StartRenderComponent;
  let fixture: ComponentFixture<StartRenderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ StartRenderComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(StartRenderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
