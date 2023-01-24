import { ChangeDetectionStrategy, Component, HostBinding, Inject, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';
import { CreateRenderConfig, isErrorRenderEvent, isLoadRenderEvent } from '../../models/render';
import { RenderService } from '../../base/services/render.service';
import { ReactiveComponent } from '../../base/reactive.component';
import { map } from 'rxjs';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { SNACKBAR_SERVICE_TOKEN, SnackbarService } from '../../base/services';

@Component({
  selector: 'app-start-render',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule, MatButtonModule, MatIconModule],
  templateUrl: './start-render.component.html',
  styleUrls: ['./start-render.component.css'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class StartRenderComponent extends ReactiveComponent {
  @HostBinding('class') classes = 'flex flex-col items-center justify-center h-full';
  private file?: File;

  state = this.connect({
    loading: this.renderService.createRenderEvent$.pipe(
      map(event => isLoadRenderEvent(event))
    ),
    error: this.renderService.createRenderEvent$.pipe(
      map(event => isErrorRenderEvent(event))
    )
  })

  constructor(private renderService: RenderService, @Inject(SNACKBAR_SERVICE_TOKEN) private snackService: SnackbarService,) {
    super();
  }

  configFormGroup = inject(FormBuilder).group({
    scene: ['Scene', {notNull: true, Validators: {required: true}}],
    samples: [128, {notNull: true, Validators: {required: true}}],
    percentage: [100, {notNull: true, Validators: {required: true}}],
    startframe: [1, {notNull: true, Validators: {required: true}}],
    endframe: [1, {notNull: true, Validators: {required: true}}],
    breakpoint: [null, {notNull: false, Validators: {required: false}}]
  });

  selectFile($event: Event) {
    const target = $event.target as HTMLInputElement;
    this.file = (target.files ?? [])[0];
  }

  startRender() {
    if (!this.file) {
      this.snackService.open('No file selected');
      return;
    }
    if (this.configFormGroup.invalid) {
      return;
    }
    const config: CreateRenderConfig = this.configFormGroup.value as CreateRenderConfig;
    this.renderService.createRender(this.file, config);
  }
}
