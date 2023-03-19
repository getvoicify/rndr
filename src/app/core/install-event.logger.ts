import { Event, listen } from '@tauri-apps/api/event';
import { Observable, Subject, tap } from 'rxjs';
import { InjectionToken, Provider } from '@angular/core';

export const coreEvent$ = <T>(event: string) => new Observable<Event<T>>(subscriber => {
  const unlisten = listen<T>(event, e => {
    subscriber.next(e);
  })
    .catch(err => subscriber.error(err));
  return () => unlisten.then(() => subscriber.complete())
});

const installEventLogger = (event: Event<unknown>) => {
  console.log(event.payload);
}

const installEventsSub: Subject<Event<string>> = new Subject<Event<string>>();
const installEvents$: Observable<Event<string>> = installEventsSub.asObservable().pipe(
  tap(event => installEventLogger(event))
);

export const INSTALL_EVENT_TOKEN = new InjectionToken<Observable<Event<string>>>('INSTALL_EVENT_TOKEN');

export const eventLoggerProvider: Provider = {
  provide: INSTALL_EVENT_TOKEN,
  useFactory: () => {
    listen<string>('inbound://installing_dependency', e => {
      installEventsSub.next(e);
    })
      .catch(console.log);
    return installEvents$;
  }
}
