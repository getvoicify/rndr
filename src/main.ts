import { APP_INITIALIZER, enableProdMode, ErrorHandler, importProvidersFrom } from '@angular/core';
import { bootstrapApplication } from '@angular/platform-browser';
import { AppComponent } from './app/app.component';

import { environment } from './environments/environment';
import { provideRouter, Router, withEnabledBlockingInitialNavigation } from '@angular/router';
import { appRoutes } from './app/app-routes';
import { initNotification } from './app/init';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import * as Sentry from '@sentry/angular-ivy';
import { BrowserTracing } from '@sentry/tracing';
import { eventLoggerProvider, TauriAsyncStorageEngine } from './app/core';
import { NgxsModule } from '@ngxs/store';
import { NgxsDispatchPluginModule } from '@ngxs-labs/dispatch-decorator';
import { AppState } from './app/+state';
import { RndrAsyncStoragePluginModule } from './app/core/async-store';

Sentry.init({
  dsn: "https://17a6d86eac26433a92d41de11e7a044c@o4504853594832896.ingest.sentry.io/4504864717733888",
  integrations: [
    new BrowserTracing({
      tracePropagationTargets: ["localhost", "https://yourserver.io/api"],
      routingInstrumentation: Sentry.routingInstrumentation,
    }),
  ],
  tracesSampleRate: 0.2,
});

if (environment.production) {
  enableProdMode();
}

bootstrapApplication(AppComponent, {
  providers: [
    importProvidersFrom(BrowserAnimationsModule),
    importProvidersFrom(MatSnackBarModule),
    importProvidersFrom(NgxsModule.forRoot([AppState], {
      developmentMode: !environment.production
    })),
    importProvidersFrom(RndrAsyncStoragePluginModule.forRoot(TauriAsyncStorageEngine)),
    importProvidersFrom(NgxsDispatchPluginModule.forRoot()),
    provideRouter(appRoutes, withEnabledBlockingInitialNavigation()),
    initNotification,
    importProvidersFrom(BrowserAnimationsModule),
    {
      provide: ErrorHandler,
      useValue: Sentry.createErrorHandler({
        showDialog: !environment.production,
      }),
    },
    {
      provide: Sentry.TraceService,
      deps: [Router],
    },
    {
      provide: APP_INITIALIZER,
      useFactory: () => () => {},
      deps: [Sentry.TraceService],
      multi: true,
    },
    eventLoggerProvider
]
}).catch((err) => console.error(err));
