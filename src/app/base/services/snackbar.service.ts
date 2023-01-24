import { MatSnackBar, MatSnackBarConfig } from '@angular/material/snack-bar';
import { inject, InjectionToken, Provider } from '@angular/core';

const snackbarService = () => {
  const snackbar = inject(MatSnackBar);

  return {
    open: (
      message: string,
      duration: number = 5000,
      appearance: 'fill' | 'outline' | 'soft' = 'fill',
      verticalPosition: 'top' | 'bottom' = 'bottom',
      horizontalPosition: 'start' | 'center' | 'end' = 'center',
      type: 'info' | 'success' | 'error' = 'info',
      action?: string,
      actionHandler?: () => void
    ) => {
      const config: MatSnackBarConfig = {
        duration,
        verticalPosition,
        horizontalPosition,
        panelClass: [`alert-type-${appearance}-${type}`]
      };
      const ref = snackbar.open(message, action, config);
      if (action && actionHandler) {
        ref.onAction().subscribe(() => {
          actionHandler();
        });
      }
    },
    close: () => {
      snackbar.dismiss();
    }
  }
}

export type SnackbarService = ReturnType<typeof snackbarService>;
export const SNACKBAR_SERVICE_TOKEN = new InjectionToken<SnackbarService>('snackbarService', {
  providedIn: 'root',
  factory: snackbarService
});