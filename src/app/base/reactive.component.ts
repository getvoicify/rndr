import { ChangeDetectorRef, Component, inject, OnDestroy, OnInit } from "@angular/core";
import { from, mergeMap, Observable, ReplaySubject, takeUntil, tap } from "rxjs";

export type ObservableDictionary<T> = { [P in keyof T]: Observable<T[P]> };

const OnInitSubject = Symbol("OnInitSubject");
const OnDestroySubject = Symbol("OnDestroySubject");

@Component({
  selector: "reactive-component",
  template: "",
  standalone: true,
})
export class ReactiveComponent implements OnInit, OnDestroy {
  private [OnInitSubject] = new ReplaySubject<void>(1);
  private [OnDestroySubject] = new ReplaySubject<void>(1);
  public connect<T>(sources: ObservableDictionary<T>): T {
    const cd = inject(ChangeDetectorRef);
    const sink = {} as T;
    const keys = Object.keys(sources) as (keyof T)[];
    const updateSink$ = from(keys).pipe(
      takeUntil(this[OnDestroySubject]),
      mergeMap((key) => sources[key].pipe(
        tap((value) => sink[key] = value)
      )),
    )
    updateSink$.subscribe(() => cd.markForCheck());
    return sink;
  }

  ngOnDestroy(): void {
    this[OnDestroySubject].next();
    this[OnDestroySubject].complete();
  }

  ngOnInit(): void {
    this[OnInitSubject].next();
    this[OnInitSubject].complete();
  }
}