import { ChangeDetectionStrategy, Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveComponent } from '../../base/reactive.component';
import { StackService } from '../../base/services';
import { filter, map, Subject, switchMap } from 'rxjs';
import { isCompleteResult, isErrorResult, isPendingResult, isProcessingResult } from '../../models';

@Component({
  selector: 'app-stack-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './stack-list.component.html',
  styleUrls: ['./stack-list.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush
})
export class StackListComponent extends ReactiveComponent {
  private readonly stackName$ = new Subject<string>();

  get isCreatingStack() {
    return (isProcessingResult(this.state.stackEvent?.payload ?? {}));
  }

  private readonly stackEvent$ = this.stackName$.asObservable().pipe(
    switchMap(name => this.stackService.stackEvent$.pipe(
      filter(event => event.payload.stackName === name),
    ))
  );

  state = this.connect({
    hasStacks: this.stackService.hasStacks$,
    stackEvent: this.stackEvent$,
    processing: this.stackEvent$.pipe(
      map(event => isProcessingResult(event.payload)),
    )
  });

  constructor(private stackService: StackService) {
    super();
  }

  async createStack(value?: string) {
    if (!value) {
      return;
    }
    this.stackName$.next(value);
    await this.stackService.createStack(value);
  }
}
