import { ChangeDetectionStrategy, Component, HostBinding, Inject, inject, TemplateRef, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormBuilder, ReactiveFormsModule } from '@angular/forms';
import { CreateRenderConfig, isErrorRenderEvent, isLoadRenderEvent } from '../../models/render';
import { RenderService } from '../../base/services/render.service';
import { ReactiveComponent } from '../../base/reactive.component';
import { BehaviorSubject, map, merge, Observable, startWith, tap } from 'rxjs';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { SNACKBAR_SERVICE_TOKEN, SnackbarService } from '../../base/services';
import { MatDialog, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { Router, RouterModule } from '@angular/router';
import { appWindow, FileDropEvent } from '@tauri-apps/api/window';
import { Event as TauriEvent, UnlistenFn } from '@tauri-apps/api/event';

type DropEventType = 'drop' | 'hover' | 'cancel';

@Component({
  selector: 'app-start-render',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule, MatButtonModule, MatIconModule, MatDialogModule, RouterModule],
  templateUrl: './start-render.component.html',
  styleUrls: ['./start-render.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class StartRenderComponent extends ReactiveComponent {

  fileDropEventSub$: BehaviorSubject<FileDropEvent> = new BehaviorSubject<FileDropEvent>({type: 'cancel'});
  private fileDropEvent$: Observable<FileDropEvent> = this.fileDropEventSub$.asObservable();

  private filenameSub$ = new BehaviorSubject<string>('');

  private filePath$: Observable<string | undefined> = merge(
    this.fileDropEvent$.pipe(
      tap(event => {
        if (event.type === 'drop') {
          this.file = undefined;
        }
      }),
      map(event =>  event.type !== 'cancel' && event.type !== "hover" ? event.paths : undefined),
      map(files => files && (files).length > 0 ? files.filter(file => file.endsWith('.blend')) : undefined),
      map(files => files && (files).length > 0 ? files[0] : undefined),
    ),
    this.filenameSub$.asObservable()
  );


  private _fileDropEventUnlistener?: Promise<UnlistenFn>;

  @ViewChild('fileInput', { static: true }) fileInput!: HTMLInputElement;
  @HostBinding('class') classes = 'flex flex-col items-center justify-center h-full';
  file?: File;

  state = this.connect({
    loading: this.renderService.createRenderEvent$.pipe(
      map(event => isLoadRenderEvent(event))
    ),
    error: this.renderService.createRenderEvent$.pipe(
      map(event => isErrorRenderEvent(event))
    ),
    hoverState: this.fileDropEvent$.pipe(
      map(event => event?.type ?? 'idle'),
      startWith('idle')
    ),
    file: this.filePath$
  });

  @ViewChild('advanceSettings', { static: true }) advanceSettingsTemplate!: TemplateRef<any>;
  dialogRef?: MatDialogRef<any>;

  constructor(
    private renderService: RenderService,
    @Inject(SNACKBAR_SERVICE_TOKEN) private snackService: SnackbarService,
    private dialog: MatDialog,
    private router: Router
    ) {
    super();
  }

  override ngOnInit() {
    super.ngOnInit();
    this._fileDropEventUnlistener = appWindow.onFileDropEvent(this.onFileDropEvent.bind(this));
  }

  override ngOnDestroy() {
    super.ngOnDestroy();
    this._fileDropEventUnlistener?.then(unlisten => unlisten());
  }

  onFileDropEvent(event: TauriEvent<FileDropEvent>) {
    this.fileDropEventSub$.next(event.payload);
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
    this.filenameSub$.next(this.file?.name ?? '');
  }

  startRender() {
    if (!this.file && !this.state.file) {
      this.snackService.open('No file selected');
      return;
    }
    if (this.configFormGroup.invalid) {
      return;
    }
    const config: CreateRenderConfig = this.configFormGroup.value as CreateRenderConfig;
    if (this.file) {
      this.renderService.createRender(this.file, config);
    } else {
      this.renderService.createRender(this.state.file ?? '', config);
    }
    this.configFormGroup.reset();
    this.router.navigate(['/jobs']).catch(console.error);
  }

  removeFile() {
    this.fileDropEventSub$.next({type: 'cancel'});
    this.file = undefined;
  }

  showAdvancedSettings() {
    this.dialogRef?.close();
    this.dialogRef = this.dialog.open(this.advanceSettingsTemplate, {
      minHeight: '520px',
      minWidth: '240px',
    });
  }

  closeDialog() {
    this.dialogRef?.close();
  }
}
