import { ChangeDetectorRef, Component, inject, OnDestroy, OnInit } from "@angular/core";
import { from, mergeMap, Observable, ReplaySubject, Subject, takeUntil, tap } from 'rxjs';

export type ObservableDictionary<T> = { [P in keyof T]: Observable<T[P]> };

const OnInitSubject = Symbol("OnInitSubject");
const OnDestroySubject = Symbol("OnDestroySubject");
const OnChangesSubject = Symbol("OnChangesSubject");

@Component({
  selector: "reactive-component",
  template: "",
  standalone: true,
})
export class ReactiveComponent implements OnInit, OnDestroy {
  private [OnInitSubject] = new ReplaySubject<void>(1);
  private [OnDestroySubject] = new ReplaySubject<void>(1);

  protected cd?: ChangeDetectorRef;
  private [OnChangesSubject] = new Subject<void>();
  public connect<T>(sources: ObservableDictionary<T>): T {
    this.cd = inject(ChangeDetectorRef);
    const sink = {} as T;
    const keys = Object.keys(sources) as (keyof T)[];
    const updateSink$ = from(keys).pipe(
      takeUntil(this[OnDestroySubject]),
      mergeMap((key) => sources[key].pipe(
        tap((value) => sink[key] = value)
      )),
    )
    updateSink$.subscribe({
      next: () => this[OnChangesSubject].next(),
      error: console.error
    });
    return sink;
  }

  ngOnDestroy(): void {
    this[OnDestroySubject].next();
    this[OnDestroySubject].complete();
  }

  ngOnInit(): void {
    this[OnInitSubject].next();
    this[OnInitSubject].complete();
    this[OnChangesSubject].asObservable().pipe(
      takeUntil(this[OnDestroySubject])
    ).subscribe({
      next: () => this.cd?.detectChanges(),
    })

  }
}