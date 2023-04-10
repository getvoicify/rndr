import { Injectable } from '@angular/core';
import { Action, NgxsOnInit, State, StateContext } from '@ngxs/store';
import { AppStateModel, isGlobalLoading, isLocalLoading, Loading } from './app-state.model';
import { PopLoading, PushLoading } from './app-state.actions';

@State<AppStateModel>({
  name: 'appState',
  defaults: {
    hasStacks: false,
    loading: []
  }
})
@Injectable()
export class AppState implements NgxsOnInit {
  ngxsOnInit({ patchState }: StateContext<AppStateModel>): void {
      patchState({
        loading: []
      });
  }

  @Action(PushLoading)
  pushLoading({patchState, getState}: StateContext<AppStateModel>, {loading}: PushLoading) {
    const loadingArr = getState().loading;

    const hasGlobalLoading = isGlobalLoading(loading) && getState().loading.filter(loading => loading.mode === 'global').length > 0;
    const hasLocalLoading = isLocalLoading(loading) && loadingArr.find(l => l.mode === 'local' && loading.id === l.id) !== undefined;

    if (hasGlobalLoading) {
      return;
    }

    if (hasLocalLoading) {
      return;
    }

    loadingArr.push(loading);

    patchState({
      loading: loadingArr
    });

  }

  @Action(PopLoading)
  popLoading({patchState, getState}: StateContext<AppStateModel>, {loading}: PopLoading) {
    const loadingArr = getState().loading;
    let newLoading: Loading[] = [];

    if (!loading) {
      patchState({
        loading: []
      });
      return;
    }

    if (isGlobalLoading(loading)) {
      newLoading = loadingArr.filter(l => l.mode !== 'global');
    }

    if (isLocalLoading(loading)) {
      newLoading = loadingArr.filter(l => l.mode === 'local' && l.id === loading.id);
    }

    patchState({
      loading: newLoading
    });

  }

}